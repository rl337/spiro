use uie_telemetry::{ComputeDevice, ExecutionRecorder, ExecutionTrace};

use crate::backend::Backend;
use crate::error::Result;
use crate::model::InferenceExecutor;
use crate::runtime::Session;
use crate::tensor::Tensor;

/// Result of [`InferenceRuntime::infer_with_trace`].
#[derive(Debug)]
pub struct InferOutcome {
    pub outputs: Vec<Tensor>,
    pub trace: ExecutionTrace,
}

/// Run `model.infer` and attach phases to `recorder` (executor device tagged CPU until loaders report GPU).
pub fn infer_with_trace(
    backend: &dyn Backend,
    model: &dyn InferenceExecutor,
    inputs: &[Tensor],
    session: &mut Session<'_>,
    mut recorder: ExecutionRecorder,
) -> Result<InferOutcome> {
    if inputs.len() != model.inputs().len() {
        return Err(crate::error::Error::Runtime(format!(
            "expected {} inputs, got {}",
            model.inputs().len(),
            inputs.len()
        )));
    }

    let mut rec = recorder
        .with_backend(backend.id().to_string())
        .with_model(
            model.metadata().name.clone(),
            model.metadata().format.clone(),
        );
    let device = executor_compute_device(model);
    let outputs = rec.phase("executor_infer", device, || model.infer(inputs))?;

    for (i, spec) in model.outputs().iter().enumerate() {
        if let Some(tensor) = outputs.get(i) {
            session.bind(spec.name.clone(), tensor.clone());
        }
    }

    let trace = rec.finish();
    Ok(InferOutcome { outputs, trace })
}

fn executor_compute_device(model: &dyn InferenceExecutor) -> ComputeDevice {
    match model.metadata().format.as_str() {
        "gguf" => {
            if std::env::var("UIE_GGUF_GPU").is_ok_and(|v| v == "1") {
                ComputeDevice::Gpu
            } else {
                ComputeDevice::Cpu
            }
        }
        "onnx" => ComputeDevice::Cpu,
        _ => ComputeDevice::Cpu,
    }
}