use uie_telemetry::{assert_phases, ExecutionTrace};

use crate::telemetry::{PHASE_POLICY_INFER, PHASE_VALIDATE_INPUTS};

/// US-010+ acceptance: every stub run must emit these phases.
pub fn assert_stub_executor_telemetry(trace: &ExecutionTrace) {
    assert_phases(trace, &[PHASE_VALIDATE_INPUTS, PHASE_POLICY_INFER]);
    assert_eq!(
        trace.model_format.as_deref(),
        Some("stub"),
        "stub trace should tag format=stub"
    );
}