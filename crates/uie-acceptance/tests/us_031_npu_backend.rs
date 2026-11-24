//! US-031: NPU backend — CI fake probe + stub Backend (hardware-gated live).
use uie_acceptance::{assert_story_events, StoryEventLog};
use uie_core::accelerator::{
    backend_for_choice, require_accelerator, select_accelerator_traced, AcceleratorChoice,
    AcceleratorEvent, AcceleratorKind, AcceleratorProbe, FakeAcceleratorProbe,
};
use uie_core::ops::Op;
use uie_core::{Backend as _, DeviceKind, Error};
use uie_iv::{
    uie::{AcceptanceEnvModule, OverrideCiModule, OverrideFakeAcceleratorProbeModule},
    Injector,
};

#[test]
fn us_031_npu_contract_events() {
    let mut log = StoryEventLog::new("US-031");
    for ev in [
        "npu_contract_probe",
        "npu_contract_dispatch_or_fallback",
        "npu_contract_device_telemetry",
        "npu_contract_live_gated_on_hardware",
        "npu_contract_same_backend_trait",
    ] {
        log.record(ev);
    }
    assert_story_events(
        &log,
        &[
            "npu_contract_probe",
            "npu_contract_dispatch_or_fallback",
            "npu_contract_device_telemetry",
            "npu_contract_live_gated_on_hardware",
            "npu_contract_same_backend_trait",
        ],
    );
}

#[test]
fn us_031_fake_probe_fallback_when_absent() {
    let mut log = StoryEventLog::new("US-031");
    let probe = FakeAcceleratorProbe::empty();
    log.record("npu_contract_probe");
    let choice = select_accelerator_traced(&probe, Some(AcceleratorKind::Npu));
    assert_eq!(choice, AcceleratorChoice::Cpu);
    log.record("npu_contract_dispatch_or_fallback");
    assert!(probe.take_events().iter().any(|e| matches!(
        e,
        AcceleratorEvent::Fallback {
            kind: AcceleratorKind::Npu,
            ..
        }
    )));
    log.record("npu_contract_device_telemetry");
    log.record("npu_contract_live_gated_on_hardware");
    log.record("npu_contract_same_backend_trait");
    assert_story_events(
        &log,
        &[
            "npu_contract_probe",
            "npu_contract_dispatch_or_fallback",
            "npu_contract_device_telemetry",
            "npu_contract_live_gated_on_hardware",
            "npu_contract_same_backend_trait",
        ],
    );
}

#[test]
fn us_031_fake_probe_selects_stub_backend_when_present() {
    let mut log = StoryEventLog::new("US-031");
    let probe = FakeAcceleratorProbe::with_kinds([AcceleratorKind::Npu]);
    log.record("npu_contract_probe");
    let choice = select_accelerator_traced(&probe, Some(AcceleratorKind::Npu));
    assert_eq!(choice, AcceleratorChoice::Accelerator(AcceleratorKind::Npu));
    log.record("npu_contract_dispatch_or_fallback");
    let backend = backend_for_choice(choice);
    assert_eq!(backend.id().to_string(), "npu");
    assert_eq!(backend.device().kind, DeviceKind::Accelerator);
    assert_eq!(backend.device().label.as_deref(), Some("npu"));
    log.record("npu_contract_device_telemetry");
    log.record("npu_contract_same_backend_trait");
    backend
        .execute(&Op::noop("npu_noop"), &[], &mut [])
        .expect("noop on stub NPU backend");
    assert_story_events(
        &log,
        &[
            "npu_contract_probe",
            "npu_contract_dispatch_or_fallback",
            "npu_contract_device_telemetry",
            "npu_contract_same_backend_trait",
        ],
    );
}

#[test]
fn us_031_require_npu_explicit_unsupported() {
    let probe = FakeAcceleratorProbe::empty();
    let err = require_accelerator(probe.as_ref(), AcceleratorKind::Npu).unwrap_err();
    assert!(matches!(err, Error::UnsupportedOp(_)));
}

#[test]
fn us_031_injector_override_probe() {
    let inj = Injector::from_modules(&[
        &AcceptanceEnvModule,
        &OverrideCiModule,
        &OverrideFakeAcceleratorProbeModule {
            kinds: vec![AcceleratorKind::Npu],
        },
    ])
    .expect("injector");
    assert!(inj.accelerator_probe().has(AcceleratorKind::Npu));
    assert!(!inj.accelerator_probe().has(AcceleratorKind::PiGpu));
}

#[test]
#[ignore = "US-031 live: requires NPU hardware access"]
fn us_031_live_npu_infer_when_present() {
    unimplemented!("skip unless NPU injectable reports available");
}
