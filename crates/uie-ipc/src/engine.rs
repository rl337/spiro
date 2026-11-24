use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use tracing::{debug, info};
use uie_core::backend::cpu::CpuBackend;
use uie_core::model::{InferenceExecutor, ModelLoader};
use uie_core::resource::{MemoryBudget, MemoryEstimate, UnlimitedMemoryBudget};
use uie_core::InferenceRuntime;
use uie_telemetry::{ExecutionRecorder, TraceSummary};

use crate::message::{InferRequest, IpcMessage, TensorPayload, PROTOCOL_VERSION};

pub struct InferenceEngineHost {
    runtime: InferenceRuntime,
    models: Mutex<HashMap<PathBuf, Arc<dyn InferenceExecutor>>>,
    budget: Arc<dyn MemoryBudget>,
    /// When a reserve fails, drop cached models held only by the host (US-025).
    evict_idle_on_pressure: bool,
}

impl InferenceEngineHost {
    /// Unlimited budget (backward compatible).
    pub fn new(loaders: Vec<Box<dyn ModelLoader>>) -> Self {
        Self::with_budget(loaders, Arc::new(UnlimitedMemoryBudget), false)
    }

    /// Bounded admission via an injected [`MemoryBudget`] (US-025).
    pub fn with_budget(
        loaders: Vec<Box<dyn ModelLoader>>,
        budget: Arc<dyn MemoryBudget>,
        evict_idle_on_pressure: bool,
    ) -> Self {
        let mut runtime = InferenceRuntime::new(Box::new(CpuBackend::new()));
        for loader in loaders {
            runtime.register_loader(loader);
        }
        Self {
            runtime,
            models: Mutex::new(HashMap::new()),
            budget,
            evict_idle_on_pressure,
        }
    }

    pub fn budget(&self) -> &dyn MemoryBudget {
        self.budget.as_ref()
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

        let recorder = ExecutionRecorder::new(req.request_id.clone()).with_transport("ipc");
        let outcome = self
            .runtime
            .infer(model.as_ref(), &tensors, &mut session, recorder)?;

        let out_payloads = outcome
            .outputs
            .iter()
            .zip(model.outputs())
            .map(|(t, spec)| TensorPayload::from_tensor(&spec.name, t))
            .collect::<uie_core::Result<Vec<_>>>()?;

        debug!(
            request_id = %req.request_id,
            total_ms = outcome.trace.metrics.total_ms,
            ttft_ms = ?outcome.trace.metrics.ttft_ms,
            "infer complete"
        );
        Ok(IpcMessage::InferResult {
            request_id: req.request_id,
            outputs: out_payloads,
            trace: Some(TraceSummary::from(&outcome.trace)),
        })
    }

    fn estimate_path(path: &Path) -> MemoryEstimate {
        match std::fs::metadata(path) {
            Ok(meta) => MemoryEstimate::from_file_len(meta.len(), path.display().to_string()),
            Err(_) => MemoryEstimate {
                bytes: 0,
                confidence: uie_core::EstimateConfidence::Unknown,
                source: path.display().to_string(),
            },
        }
    }

    /// Evict cache entries solely held by the host; release residency on the budget.
    fn evict_idle_locked(
        budget: &dyn MemoryBudget,
        guard: &mut HashMap<PathBuf, Arc<dyn InferenceExecutor>>,
    ) -> usize {
        let idle: Vec<PathBuf> = guard
            .iter()
            .filter(|(_, m)| Arc::strong_count(m) == 1)
            .map(|(p, _)| p.clone())
            .collect();
        for path in &idle {
            guard.remove(path);
            budget.release_residency(&path.display().to_string());
            info!(path = %path.display(), "evicted idle cached model (US-025)");
        }
        idle.len()
    }

    fn load_cached(&self, path: &Path) -> uie_core::Result<Arc<dyn InferenceExecutor>> {
        let mut guard = self
            .models
            .lock()
            .map_err(|_| uie_core::Error::Runtime("model cache lock poisoned".into()))?;
        if let Some(m) = guard.get(path) {
            return Ok(Arc::clone(m));
        }

        let estimate = Self::estimate_path(path);
        let reservation = match self.budget.try_reserve(&estimate) {
            Ok(r) => r,
            Err(e) if self.evict_idle_on_pressure => {
                let n = Self::evict_idle_locked(self.budget.as_ref(), &mut guard);
                if n == 0 {
                    return Err(e);
                }
                self.budget.try_reserve(&estimate)?
            }
            Err(e) => return Err(e),
        };

        info!(
            path = %path.display(),
            estimate_bytes = estimate.bytes,
            budget = self.budget.budget_source(),
            "loading model for ipc (admission reserved)"
        );

        let loaded = match self.runtime.load_model(path) {
            Ok(m) => m,
            Err(e) => {
                self.budget.release(reservation);
                return Err(e);
            }
        };
        let arc: Arc<dyn InferenceExecutor> = Arc::from(loaded);
        let key = path.display().to_string();
        self.budget.commit(reservation, &key);
        guard.insert(path.to_path_buf(), Arc::clone(&arc));
        Ok(arc)
    }
}
