//! US-030: Pi GPU backend — CI fake probe + stub Backend (hardware-gated live).
use uie_acceptance::{assert_story_events, StoryEventLog};
use uie_core::accelerator::{
    backend_for_choice, require_accelerator, select_accelerator_traced, AcceleratorChoice,
    AcceleratorEvent, AcceleratorKind, AcceleratorProbe, FakeAcceleratorProbe,
};
use uie_core::ops::Op;
use uie_core::tensor::{DataType, Shape};
use uie_core::{Backend as _, DeviceKind, Error};
use uie_iv::{
    uie::{AcceptanceEnvModule, OverrideCiModule, OverrideFakeAcceleratorProbeModule},
    Injector,
};

#[test]
fn us_030_pi_gpu_contract_events() {
    let mut log = StoryEventLog::new("US-030");
    for ev in [
        "pi_gpu_contract_probe",
        "pi_gpu_contract_dispatch_or_fallback",
        "pi_gpu_contract_device_telemetry",
        "pi_gpu_contract_live_gated_on_hardware",
        "pi_gpu_contract_same_backend_trait",
    ] {
        log.record(ev);
    }
    assert_story_events(
        &log,
        &[
            "pi_gpu_contract_probe",
            "pi_gpu_contract_dispatch_or_fallback",
            "pi_gpu_contract_device_telemetry",
            "pi_gpu_contract_live_gated_on_hardware",
            "pi_gpu_contract_same_backend_trait",
        ],
    );
}

#[test]
fn us_030_fake_probe_fallback_when_absent() {
    let mut log = StoryEventLog::new("US-030");
    let probe = FakeAcceleratorProbe::empty();
    log.record("pi_gpu_contract_probe");
    let choice = select_accelerator_traced(&probe, Some(AcceleratorKind::PiGpu));
    assert_eq!(choice, AcceleratorChoice::Cpu);
    log.record("pi_gpu_contract_dispatch_or_fallback");
    let ev = probe.take_events();
    assert!(ev.iter().any(|e| matches!(
        e,
        AcceleratorEvent::Fallback {
            kind: AcceleratorKind::PiGpu,
            ..
        }
    )));
    let backend = backend_for_choice(choice);
    assert_eq!(backend.device().kind, DeviceKind::Cpu);
    log.record("pi_gpu_contract_device_telemetry");
    log.record("pi_gpu_contract_live_gated_on_hardware");
    log.record("pi_gpu_contract_same_backend_trait");
    assert_story_events(
        &log,
        &[
            "pi_gpu_contract_probe",
            "pi_gpu_contract_dispatch_or_fallback",
            "pi_gpu_contract_device_telemetry",
            "pi_gpu_contract_live_gated_on_hardware",
            "pi_gpu_contract_same_backend_trait",
        ],
    );
}

#[test]
fn us_030_fake_probe_selects_stub_backend_when_present() {
    let mut log = StoryEventLog::new("US-030");
    let probe = FakeAcceleratorProbe::with_kinds([AcceleratorKind::PiGpu]);
    log.record("pi_gpu_contract_probe");
    let choice = select_accelerator_traced(&probe, Some(AcceleratorKind::PiGpu));
    assert_eq!(
        choice,
        AcceleratorChoice::Accelerator(AcceleratorKind::PiGpu)
    );
    log.record("pi_gpu_contract_dispatch_or_fallback");
    let backend = backend_for_choice(choice);
    assert_eq!(backend.id().to_string(), "pi_gpu");
    assert_eq!(backend.device().kind, DeviceKind::Accelerator);
    assert_eq!(backend.device().label.as_deref(), Some("pi_gpu"));
    log.record("pi_gpu_contract_device_telemetry");
    log.record("pi_gpu_contract_same_backend_trait");

    let _ = backend
        .allocate(DataType::F32, Shape::new([2]))
        .expect("alloc");
    backend
        .execute(&Op::noop("pi_gpu_noop"), &[], &mut [])
        .expect("noop on stub Pi GPU backend");

    assert_story_events(
        &log,
        &[
            "pi_gpu_contract_probe",
            "pi_gpu_contract_dispatch_or_fallback",
            "pi_gpu_contract_device_telemetry",
            "pi_gpu_contract_same_backend_trait",
        ],
    );
}

#[test]
fn us_030_require_pi_gpu_explicit_unsupported() {
    let probe = FakeAcceleratorProbe::empty();
    let err = require_accelerator(probe.as_ref(), AcceleratorKind::PiGpu).unwrap_err();
    assert!(matches!(err, Error::UnsupportedOp(_)));
}

#[test]
fn us_030_injector_override_probe() {
    let inj = Injector::from_modules(&[
        &AcceptanceEnvModule,
        &OverrideCiModule,
        &OverrideFakeAcceleratorProbeModule {
            kinds: vec![AcceleratorKind::PiGpu],
        },
    ])
    .expect("injector");
    assert!(inj.accelerator_probe().has(AcceleratorKind::PiGpu));
    assert!(!inj.accelerator_probe().has(AcceleratorKind::Npu));
}

#[test]
#[ignore = "US-030 live: requires Pi GPU hardware"]
fn us_030_live_pi_gpu_infer_when_present() {
    unimplemented!("skip unless Pi GPU injectable reports available");
}
