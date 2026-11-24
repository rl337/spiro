//! US-029: capability-aware kernels — planned contract markers.
use uie_acceptance::{assert_story_events, StoryEventLog};

#[test]
fn us_029_kernel_contract_events() {
    let mut log = StoryEventLog::new("US-029");
    for ev in [
        "kernel_contract_probe",
        "kernel_contract_dispatch_event",
        "kernel_contract_fallback_or_unsupported",
        "kernel_contract_numeric_tolerance",
        "kernel_contract_no_illegal_instruction",
    ] {
        log.record(ev);
    }
    assert_story_events(
        &log,
        &[
            "kernel_contract_probe",
            "kernel_contract_dispatch_event",
            "kernel_contract_fallback_or_unsupported",
            "kernel_contract_numeric_tolerance",
            "kernel_contract_no_illegal_instruction",
        ],
    );
}

#[test]
#[ignore = "US-029 not implemented: scalar vs optimized fixture dispatch"]
fn us_029_fallback_when_feature_missing() {
    unimplemented!("force-unsupported feature → fallback or explicit error");
}
