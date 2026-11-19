use std::path::Path;
use std::sync::Mutex;

use tracing::{info, instrument};
use tract_core::dim::DimLike;
use tract_onnx::prelude::*;
use uie_core::backend::Backend;
use uie_core::error::{Error, Result};
use uie_core::model::{InferenceExecutor, IoSpec, ModelLoader, ModelMetadata};
use uie_core::tensor::{DataType, Shape, Tensor};
use uie_telemetry::{ComputeDevice, ExecutionRecorder};

use crate::convert::{tensor_to_tvalue, tvalue_to_tensor};

pub struct OnnxModelLoader;

impl OnnxModelLoader {
    pub fn new() -> Self {
        Self
    }
}

impl Default for OnnxModelLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelLoader for OnnxModelLoader {
    fn format_id(&self) -> &str {
        "onnx"
    }

    fn can_load(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| e.eq_ignore_ascii_case("onnx"))
    }

    fn load(&self, path: &Path, backend: &dyn Backend) -> Result<Box<dyn InferenceExecutor>> {
        let _ = backend;
        info!(path = %path.display(), backend = %backend.id(), "loading ONNX model");
        let model = OnnxModel::from_path(path)?;
        Ok(Box::new(model))
    }
}

pub struct OnnxModel {
    metadata: ModelMetadata,
    inputs: Vec<IoSpec>,
    outputs: Vec<IoSpec>,
    runnable: Mutex<TypedRunnableModel<TypedFact, Box<dyn TypedFact>, TValue>>,
}

impl OnnxModel {
    pub fn from_path(path: &Path) -> Result<Self> {
        let runnable = tract_onnx::onnx()
            .model_for_path(path)
            .map_err(|e| Error::ModelLoad(e.to_string()))?
            .into_optimized()
            .map_err(|e| Error::ModelLoad(e.to_string()))?
            .into_runnable()
            .map_err(|e| Error::ModelLoad(e.to_string()))?;

        let mut inputs = Vec::new();
        for i in 0..runnable.model().inputs.len() {
            let fact = runnable
                .model()
                .input_fact(i)
                .map_err(|e| Error::ModelLoad(e.to_string()))?;
            inputs.push(io_spec_from_fact(fact, format!("input_{i}"))?);
        }

        let mut outputs = Vec::new();
        for i in 0..runnable.model().outputs.len() {
            let fact = runnable
                .model()
                .output_fact(i)
                .map_err(|e| Error::ModelLoad(e.to_string()))?;
            outputs.push(io_spec_from_fact(fact, format!("output_{i}"))?);
        }

        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("onnx")
            .to_string();

        Ok(Self {
            metadata: ModelMetadata {
                name,
                format: "onnx".into(),
                ops: runnable.model().nodes().len(),
            },
            inputs,
            outputs,
            runnable: Mutex::new(runnable),
        })
    }
}

fn io_spec_from_fact(fact: &TypedFact, name: String) -> Result<IoSpec> {
    let shape = Shape::new(
        fact.shape
            .to_tvec()
            .iter()
            .map(|d| {
                d.to_usize()
                    .ok_or_else(|| Error::InvalidShape(format!("dynamic dim in {name}")))
            })
            .collect::<Result<Vec<_>>>()?,
    );
    let dtype = match fact.datum_type {
        tract_onnx::prelude::DatumType::F32 => DataType::F32,
        other => {
            return Err(Error::UnsupportedOp(format!(
                "ONNX dtype {:?} not supported in UIE yet",
                other
            )));
        }
    };
    Ok(IoSpec { name, shape, dtype })
}

impl InferenceExecutor for OnnxModel {
    fn metadata(&self) -> &ModelMetadata {
        &self.metadata
    }

    fn inputs(&self) -> &[IoSpec] {
        &self.inputs
    }

    fn outputs(&self) -> &[IoSpec] {
        &self.outputs
    }

    #[instrument(skip(self, inputs), fields(model = %self.metadata.name))]
    fn infer(&self, inputs: &[Tensor], telemetry: &mut ExecutionRecorder) -> Result<Vec<Tensor>> {
        telemetry.phase("onnx_validate_inputs", ComputeDevice::Cpu, || {
            if inputs.len() != self.inputs.len() {
                return Err(Error::Runtime(format!(
                    "model expects {} inputs, got {}",
                    self.inputs.len(),
                    inputs.len()
                )));
            }
            for (tensor, spec) in inputs.iter().zip(&self.inputs) {
                if tensor.dtype() != spec.dtype {
                    return Err(Error::DtypeMismatch {
                        expected: spec.dtype,
                        got: tensor.dtype(),
                    });
                }
                if tensor.shape().dims() != spec.shape.dims() {
                    return Err(Error::InvalidShape(format!(
                        "input `{}`: expected {:?}, got {:?}",
                        spec.name,
                        spec.shape.dims(),
                        tensor.shape().dims()
                    )));
                }
            }
            Ok(())
        })?;

        telemetry.phase("onnx_tract_run", ComputeDevice::Cpu, || {
            let tract_inputs = inputs
                .iter()
                .map(tensor_to_tvalue)
                .collect::<Result<Vec<_>>>()?;

            let mut model = self
                .runnable
                .lock()
                .map_err(|_| Error::Runtime("ONNX model lock poisoned".into()))?;

            let outputs = model
                .run(tract_inputs.into())
                .map_err(|e| Error::Runtime(e.to_string()))?;

            outputs
                .iter()
                .enumerate()
                .map(|(i, tv)| {
                    let name = self
                        .outputs
                        .get(i)
                        .map(|s| s.name.as_str())
                        .unwrap_or("output");
                    tvalue_to_tensor(tv, name)
                })
                .collect()
        })
    }
}
