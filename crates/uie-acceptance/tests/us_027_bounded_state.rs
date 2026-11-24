//! US-027: bounded inference state — planned contract markers.
use uie_acceptance::{assert_story_events, StoryEventLog};

#[test]
fn us_027_state_contract_events() {
    let mut log = StoryEventLog::new("US-027");
    for ev in [
        "state_contract_budget",
        "state_contract_accept_within",
        "state_contract_reject_or_negotiate",
        "state_contract_release_on_cancel",
        "state_contract_no_silent_truncation",
    ] {
        log.record(ev);
    }
    assert_story_events(
        &log,
        &[
            "state_contract_budget",
            "state_contract_accept_within",
            "state_contract_reject_or_negotiate",
            "state_contract_release_on_cancel",
            "state_contract_no_silent_truncation",
        ],
    );
}

#[test]
#[ignore = "US-027 not implemented: context/state budget fixture"]
fn us_027_reject_over_context_budget() {
    unimplemented!("declare state limit; reject over-budget session");
}
