use serde::{Deserialize, Serialize};

use crate::device::ComputeDevice;
use crate::metrics::InferMetrics;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseSpan {
    pub name: String,
    pub device: ComputeDevice,
    pub duration_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionTrace {
    pub run_id: String,
    pub model_name: Option<String>,
    pub model_format: Option<String>,
    /// User story id for stub and catalog rows (e.g. `US-010`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_story: Option<String>,
    pub backend_id: Option<String>,
    pub transport: Option<String>,
    pub phases: Vec<PhaseSpan>,
    pub metrics: InferMetrics,
}

/// Wire-friendly subset for IPC / dashboards.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceSummary {
    pub run_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_story: Option<String>,
    pub total_ms: f64,
    pub ttft_ms: Option<f64>,
    pub decode_tokens_per_sec: Option<f64>,
    pub cpu_time_ms: f64,
    pub gpu_time_ms: Option<f64>,
}

impl From<&ExecutionTrace> for TraceSummary {
    fn from(t: &ExecutionTrace) -> Self {
        Self {
            run_id: t.run_id.clone(),
            user_story: t.user_story.clone(),
            total_ms: t.metrics.total_ms,
            ttft_ms: t.metrics.ttft_ms,
            decode_tokens_per_sec: t.metrics.decode_tokens_per_sec,
            cpu_time_ms: t.metrics.cpu_time_ms,
            gpu_time_ms: t.metrics.gpu_time_ms,
        }
    }
}
