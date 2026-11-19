use uie_telemetry::{assert_phases, ExecutionTrace};

use crate::telemetry::{phase_modality, PHASE_MODALITY_PREFIX, PHASE_POLICY_INFER, PHASE_VALIDATE_INPUTS};
use crate::ModalityKind;

/// US-010+ acceptance: every stub run must emit these phases and stub metadata.
pub fn assert_stub_executor_telemetry(trace: &ExecutionTrace) {
    assert_phases(trace, &[PHASE_VALIDATE_INPUTS, PHASE_POLICY_INFER]);
    assert!(
        trace
            .phases
            .iter()
            .any(|p| p.name.starts_with(PHASE_MODALITY_PREFIX)),
        "stub trace must include a `{PHASE_MODALITY_PREFIX}*` phase"
    );
    assert_eq!(
        trace.model_format.as_deref(),
        Some("stub"),
        "stub trace should tag format=stub"
    );
    assert!(
        trace.user_story.as_deref().is_some_and(|s| s.starts_with("US-")),
        "stub trace should tag user_story=US-0NN"
    );
}

/// Modality-specific stub acceptance (US-010–019 per-kind tests).
pub fn assert_stub_modality_telemetry(trace: &ExecutionTrace, kind: ModalityKind) {
    assert_stub_executor_telemetry(trace);
    assert_eq!(
        trace.user_story.as_deref(),
        Some(kind.story_id()),
        "stub trace user_story must match modality"
    );
    assert!(
        trace
            .phases
            .iter()
            .any(|p| p.name == phase_modality(kind)),
        "missing phase {}",
        phase_modality(kind)
    );
}