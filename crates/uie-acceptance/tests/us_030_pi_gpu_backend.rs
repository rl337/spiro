//! US-030: Pi GPU backend — planned contract markers (hardware-gated).
use uie_acceptance::{assert_story_events, StoryEventLog};

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
#[ignore = "US-030 not implemented: requires Pi GPU hardware or fake Backend"]
fn us_030_live_pi_gpu_infer_when_present() {
    unimplemented!("skip unless Pi GPU injectable reports available");
}
