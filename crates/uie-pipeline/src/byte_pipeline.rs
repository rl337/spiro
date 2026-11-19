use std::path::PathBuf;

use std::time::Instant;

use tracing::debug;
use uie_core::error::Result;
use uie_core::model::ModelLoader;
use uie_core::tensor::Tensor;
use uie_ipc::{ByteChannel, InProcessChannel, InferenceEngineHost, IpcMessage, TensorPayload};
use uie_telemetry::ComputeDevice;

#[derive(Debug, Clone)]
pub struct PipelineHop {
    pub model_path: PathBuf,
    pub label: String,
}

/// Runs each hop as an `infer` IPC round-trip on an in-process channel.
pub struct ByteChannelPipelineRunner {
    channel: InProcessChannel,
    hops: Vec<PipelineHop>,
}

impl ByteChannelPipelineRunner {
    pub fn new(host: InferenceEngineHost, hops: Vec<PipelineHop>) -> Self {
        Self {
            channel: InProcessChannel::new(host),
            hops,
        }
    }

    pub fn hops(&self) -> &[PipelineHop] {
        &self.hops
    }

    pub fn run_tensors(&mut self, request_id: &str, inputs: &[Tensor]) -> Result<Vec<Tensor>> {
        let payloads = inputs
            .iter()
            .enumerate()
            .map(|(i, t)| TensorPayload::from_tensor(format!("input_{i}"), t))
            .collect::<Result<Vec<_>>>()?;

        let out = self.run_payloads(request_id, payloads)?;
        out.into_iter().map(|p| p.to_tensor()).collect()
    }

    pub fn run_payloads(
        &mut self,
        request_id: &str,
        mut current: Vec<TensorPayload>,
    ) -> Result<Vec<TensorPayload>> {
        for (idx, hop) in self.hops.iter().enumerate() {
            debug!(stage = %hop.label, hop = idx, "byte channel pipeline hop");
            let rid = format!("{request_id}-s{idx}");
            let t0 = Instant::now();
            let resp = self.channel.roundtrip(&IpcMessage::Infer {
                request_id: rid,
                model_path: hop.model_path.to_string_lossy().into(),
                inputs: current,
            })?;
            let hop_ms = t0.elapsed().as_secs_f64() * 1000.0;
            tracing::info!(
                hop = %hop.label,
                hop_ms,
                device = ?ComputeDevice::Unknown,
                "byte_channel hop"
            );

            current = match resp {
                IpcMessage::InferResult { outputs, .. } => outputs,
                IpcMessage::Error { message } => {
                    return Err(uie_core::Error::Runtime(format!(
                        "pipeline hop `{}`: {message}",
                        hop.label
                    )));
                }
                other => {
                    return Err(uie_core::Error::Runtime(format!(
                        "pipeline hop `{}`: unexpected {other:?}",
                        hop.label
                    )));
                }
            };
        }
        Ok(current)
    }

    pub fn from_loaders(loaders: Vec<Box<dyn ModelLoader>>, hops: Vec<PipelineHop>) -> Self {
        Self::new(InferenceEngineHost::new(loaders), hops)
    }
}
