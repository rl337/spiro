use uie_telemetry::{assert_phases, ExecutionTrace};

use crate::telemetry::{PHASE_EMBED_ENCODE, PHASE_EMBED_VALIDATE};

/// US-033 acceptance: embed runs emit validate + encode phases and stub tags.
pub fn assert_embed_telemetry(trace: &ExecutionTrace) {
    assert_phases(trace, &[PHASE_EMBED_VALIDATE, PHASE_EMBED_ENCODE]);
    assert_eq!(
        trace.model_format.as_deref(),
        Some("stub"),
        "embed stub trace should tag format=stub"
    );
    assert_eq!(
        trace.user_story.as_deref(),
        Some("US-033"),
        "embed stub trace should tag user_story=US-033"
    );
}
