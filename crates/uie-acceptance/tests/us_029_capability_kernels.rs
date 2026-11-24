//! US-029: capability-aware kernel dispatch — probe, fallback, no illegal insn.
use uie_acceptance::{assert_story_events, StoryEventLog};
use uie_core::capability::{
    fill_f32_dispatch, fill_f32_scalar, require_feature, select_kernel_traced, CapabilityEvent,
    CpuFeature, FakeCapabilityProbe, KernelPath,
};
use uie_core::Error;

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
fn us_029_fallback_when_feature_missing() {
    let mut log = StoryEventLog::new("US-029");
    let probe = FakeCapabilityProbe::empty();
    let decision = select_kernel_traced(&probe, Some(CpuFeature::Neon));
    assert_eq!(decision.path, KernelPath::Scalar);
    log.record("kernel_contract_probe");
    log.record("kernel_contract_fallback_or_unsupported");
    log.record("kernel_contract_no_illegal_instruction");

    let events = probe.take_events();
    assert!(events.iter().any(|e| matches!(e, CapabilityEvent::Probe { present: false, .. })));
    assert!(events.iter().any(|e| matches!(e, CapabilityEvent::Fallback { .. })));

    assert_story_events(
        &log,
        &[
            "kernel_contract_probe",
            "kernel_contract_fallback_or_unsupported",
            "kernel_contract_no_illegal_instruction",
        ],
    );
}

#[test]
fn us_029_dispatch_when_feature_present_and_numeric_match() {
    let mut log = StoryEventLog::new("US-029");
    let probe = FakeCapabilityProbe::with_features([CpuFeature::Neon]);
    let mut scalar = [0f32; 9];
    let mut opt = [0f32; 9];
    fill_f32_scalar(&mut scalar, 2.5);
    let decision = fill_f32_dispatch(probe.as_ref(), Some(CpuFeature::Neon), &mut opt, 2.5);
    assert_eq!(
        decision.path,
        KernelPath::Optimized {
            feature: CpuFeature::Neon
        }
    );
    assert_eq!(scalar, opt);
    log.record("kernel_contract_dispatch_event");
    log.record("kernel_contract_numeric_tolerance");
    assert_story_events(
        &log,
        &[
            "kernel_contract_dispatch_event",
            "kernel_contract_numeric_tolerance",
        ],
    );
}

#[test]
fn us_029_require_feature_explicit_unsupported() {
    let mut log = StoryEventLog::new("US-029");
    let probe = FakeCapabilityProbe::empty();
    let err = require_feature(probe.as_ref(), CpuFeature::DotProd).unwrap_err();
    assert!(matches!(err, Error::UnsupportedOp(_)));
    log.record("kernel_contract_fallback_or_unsupported");
    assert_story_events(&log, &["kernel_contract_fallback_or_unsupported"]);
}
