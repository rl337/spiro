//! IPC infer trace checks shared by acceptance and crate integration tests.

use crate::TraceSummary;

#[derive(Debug, Clone)]
pub struct IpcInferTrace {
    pub run_id: String,
    pub total_ms: f64,
}

impl IpcInferTrace {
    pub fn from_summary(summary: &TraceSummary) -> Self {
        Self {
            run_id: summary.run_id.clone(),
            total_ms: summary.total_ms,
        }
    }
}

pub fn assert_ipc_infer_trace_summary(summary: &TraceSummary, story_id: &str) {
    assert!(
        !summary.run_id.is_empty(),
        "{story_id}: trace summary must include run_id"
    );
    assert!(
        summary.total_ms >= 0.0,
        "{story_id}: trace summary missing total_ms"
    );
}