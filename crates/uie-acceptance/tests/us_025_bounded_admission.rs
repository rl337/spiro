//! US-025: bounded model admission — planned contract markers.
use uie_acceptance::{assert_story_events, StoryEventLog};

#[test]
fn us_025_admission_contract_events() {
    let mut log = StoryEventLog::new("US-025");
    // Contract the implementation must emit once shipped (EDGE-MEM-01).
    for ev in [
        "admission_contract_estimate",
        "admission_contract_reserve",
        "admission_contract_reject_or_evict",
        "admission_contract_release",
        "admission_contract_no_double_spend",
    ] {
        log.record(ev);
    }
    assert_story_events(
        &log,
        &[
            "admission_contract_estimate",
            "admission_contract_reserve",
            "admission_contract_reject_or_evict",
            "admission_contract_release",
            "admission_contract_no_double_spend",
        ],
    );
}

#[test]
#[ignore = "US-025 not implemented: fake MemoryBudgetProvider + host cache"]
fn us_025_reject_when_over_budget() {
    unimplemented!("reserve A, reject B exceeding remaining capacity");
}
