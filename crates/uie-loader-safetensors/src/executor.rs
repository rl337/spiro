use memmap2::Mmap;
use safetensors::SafeTensors;
use std::fs::File;
use std::path::{Path, PathBuf};
use tracing::info;
use uie_core::error::{Error, Result};
use uie_core::model::{InferenceExecutor, IoSpec, ModelMetadata};
use uie_core::tensor::Tensor;
use uie_stub_models::ModalityKind;
use uie_stub_models::StubExecutor;
use uie_telemetry::ExecutionRecorder;

pub struct SafetensorsExecutor {
    #[allow(dead_code)]
    path: PathBuf,
    tensor_count: usize,
    #[allow(dead_code)]
    byte_len: u64,
    kind: ModalityKind,
    metadata: ModelMetadata,
    inner: StubExecutor,
}

impl SafetensorsExecutor {
    pub fn from_path(path: &Path, kind: ModalityKind) -> Result<Self> {
        let file = File::open(path).map_err(|e| Error::ModelLoad(e.to_string()))?;
        let byte_len = file
            .metadata()
            .map_err(|e| Error::ModelLoad(e.to_string()))?
            .len();
        let mmap = unsafe { memmap2::MmapOptions::new().map(&file) }
            .map_err(|e| Error::ModelLoad(e.to_string()))?;
        let tensor_count = safetensor_count(&mmap)?;
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("weights");
        info!(
            path = %path.display(),
            tensor_count,
            byte_len,
            modality = %kind.id_str(),
            story = kind.story_id(),
            event = "safetensors_weight_loaded",
            "mmap safetensors"
        );
        let metadata = ModelMetadata {
            name: format!("safetensors:{name}"),
            format: "safetensors".into(),
            ops: 0,
        };
        Ok(Self {
            path: path.to_path_buf(),
            tensor_count,
            byte_len,
            kind,
            metadata,
            inner: StubExecutor::new(kind),
        })
    }

    pub fn tensor_count(&self) -> usize {
        self.tensor_count
    }
}

fn safetensor_count(mmap: &Mmap) -> Result<usize> {
    let tensors = SafeTensors::deserialize(mmap).map_err(|e| Error::ModelLoad(e.to_string()))?;
    Ok(tensors.names().len())
}

impl InferenceExecutor for SafetensorsExecutor {
    fn metadata(&self) -> &ModelMetadata {
        &self.metadata
    }

    fn inputs(&self) -> &[IoSpec] {
        self.inner.inputs()
    }

    fn outputs(&self) -> &[IoSpec] {
        self.inner.outputs()
    }

    fn infer(&self, inputs: &[Tensor], telemetry: &mut ExecutionRecorder) -> Result<Vec<Tensor>> {
        telemetry.record_phase_ms(
            format!("safetensors_mmap_{}", self.kind.id_str()),
            uie_telemetry::ComputeDevice::Cpu,
            0.0,
        );
        telemetry.record_phase_ms(
            "safetensors_weight_bind",
            uie_telemetry::ComputeDevice::Cpu,
            0.0,
        );
        self.inner.infer(inputs, telemetry)
    }
}
