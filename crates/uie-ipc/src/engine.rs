use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use tracing::{debug, info};
use uie_core::backend::cpu::CpuBackend;
use uie_core::model::{InferenceExecutor, ModelLoader};
use uie_core::{InferenceRuntime, Session};

use crate::message::{InferRequest, IpcMessage, TensorPayload, PROTOCOL_VERSION};

pub struct InferenceEngineHost {
    runtime: InferenceRuntime,
    models: Mutex<HashMap<PathBuf, Arc<dyn InferenceExecutor>>>,
}

impl InferenceEngineHost {
    pub fn new(loaders: Vec<Box<dyn ModelLoader>>) -> Self {
        let mut runtime = InferenceRuntime::new(Box::new(CpuBackend::new()));
        for loader in loaders {
            runtime.register_loader(loader);
        }
        Self {
            runtime,
            models: Mutex::new(HashMap::new()),
        }
    }

    pub fn handle_message(&self, msg: IpcMessage) -> IpcMessage {
        match msg {
            IpcMessage::Hello { protocol_version } => {
                if protocol_version != PROTOCOL_VERSION {
                    return IpcMessage::Error {
                        message: format!(
                            "protocol mismatch: client={protocol_version} engine={PROTOCOL_VERSION}"
                        ),
                    };
                }
                IpcMessage::HelloAck {
                    protocol_version: PROTOCOL_VERSION,
                    engine: "uie-ipc".into(),
                }
            }
            IpcMessage::Infer {
                request_id,
                model_path,
                inputs,
            } => {
                let req = InferRequest {
                    request_id,
                    model_path,
                    inputs,
                };
                match self.infer(req) {
                    Ok(resp) => resp,
                    Err(e) => IpcMessage::Error {
                        message: e.to_string(),
                    },
                }
            }
            other => IpcMessage::Error {
                message: format!("unexpected message: {other:?}"),
            },
        }
    }

    fn infer(&self, req: InferRequest) -> uie_core::Result<IpcMessage> {
        let path = PathBuf::from(&req.model_path);
        let model = self.load_cached(&path)?;
        let tensors: Vec<_> = req
            .inputs
            .iter()
            .map(TensorPayload::to_tensor)
            .collect::<uie_core::Result<_>>()?;

        let mut session = self.runtime.create_session();
        for (payload, tensor) in req.inputs.iter().zip(&tensors) {
            session.bind(&payload.name, tensor.clone());
        }

        let outputs = self
            .runtime
            .infer(model.as_ref(), &tensors, &mut session)?;

        let out_payloads = outputs
            .iter()
            .zip(model.outputs())
            .map(|(t, spec)| TensorPayload::from_tensor(&spec.name, t))
            .collect::<uie_core::Result<Vec<_>>>()?;

        debug!(request_id = %req.request_id, "infer complete");
        Ok(IpcMessage::InferResult {
            request_id: req.request_id,
            outputs: out_payloads,
        })
    }

    fn load_cached(&self, path: &Path) -> uie_core::Result<Arc<dyn InferenceExecutor>> {
        let mut guard = self
            .models
            .lock()
            .map_err(|_| uie_core::Error::Runtime("model cache lock poisoned".into()))?;
        if let Some(m) = guard.get(path) {
            return Ok(Arc::clone(m));
        }
        info!(path = %path.display(), "loading model for ipc");
        let loaded = self.runtime.load_model(path)?;
        let arc: Arc<dyn InferenceExecutor> = Arc::from(loaded);
        guard.insert(path.to_path_buf(), Arc::clone(&arc));
        Ok(arc)
    }
}