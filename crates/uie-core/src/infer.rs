use uie_telemetry::{ComputeDevice, ExecutionRecorder, ExecutionTrace};

use crate::backend::Backend;
use crate::error::Result;
use crate::model::InferenceExecutor;
use crate::runtime::Session;
use crate::tensor::Tensor;

/// Result of [`InferenceRuntime::infer`].
#[derive(Debug)]
pub struct InferOutcome {
    pub outputs: Vec<Tensor>,
    pub trace: ExecutionTrace,
}

pub fn run_infer_with_trace(
    backend: &dyn Backend,
    model: &dyn InferenceExecutor,
    inputs: &[Tensor],
    session: &mut Session<'_>,
    recorder: ExecutionRecorder,
) -> Result<InferOutcome> {
    if inputs.len() != model.inputs().len() {
        return Err(crate::error::Error::Runtime(format!(
            "expected {} inputs, got {}",
            model.inputs().len(),
            inputs.len()
        )));
    }

    let mut rec = recorder.with_backend(backend.id().to_string()).with_model(
        model.metadata().name.clone(),
        model.metadata().format.clone(),
    );

    let outputs = model.infer(inputs, &mut rec)?;

    rec.phase("runtime_session_bind", ComputeDevice::Cpu, || {
        for (i, spec) in model.outputs().iter().enumerate() {
            if let Some(tensor) = outputs.get(i) {
                session.bind(spec.name.clone(), tensor.clone());
            }
        }
        Ok(())
    })?;

    let trace = rec.finish();
    Ok(InferOutcome { outputs, trace })
}
