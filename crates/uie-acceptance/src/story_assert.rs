//! Telemetry and IPC trace assertions for acceptance tests.

use uie_core::infer::InferOutcome;
use uie_ipc::IpcMessage;
use uie_telemetry::{assert_ipc_infer_trace_summary, TraceSummary};

/// Every infer-based story must produce a non-empty execution trace.
pub fn assert_infer_outcome_trace(outcome: &InferOutcome, story_id: &str) {
    assert!(
        !outcome.trace.phases.is_empty(),
        "{story_id}: infer outcome must include trace phases"
    );
    assert!(
        outcome.trace.metrics.total_ms >= 0.0,
        "{story_id}: total_ms must be present"
    );
}

/// US-020 / pipeline hops: infer over the byte channel must return a serialized trace.
pub fn assert_ipc_infer_trace(message: &IpcMessage, story_id: &str) {
    match message {
        IpcMessage::InferResult { trace, .. } => {
            let summary = trace
                .as_ref()
                .unwrap_or_else(|| panic!("{story_id}: InferResult.trace must be Some"));
            assert_ipc_infer_trace_summary(summary, story_id);
        }
        other => panic!("{story_id}: expected InferResult, got {other:?}"),
    }
}

/// Alias for wire-format traces (re-exported for story docs).
pub fn assert_trace_summary(summary: &TraceSummary, story_id: &str) {
    assert_ipc_infer_trace_summary(summary, story_id);
}