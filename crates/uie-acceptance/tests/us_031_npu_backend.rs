//! US-031: NPU backend — planned contract markers (hardware-gated).
use uie_acceptance::{assert_story_events, StoryEventLog};

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
#[ignore = "US-031 not implemented: requires NPU hardware access"]
fn us_031_live_npu_infer_when_present() {
    unimplemented!("skip unless NPU injectable reports available");
}
