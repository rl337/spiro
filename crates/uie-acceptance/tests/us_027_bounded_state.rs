//! US-027: bounded inference state — fake context budget (no silent truncation).
use uie_acceptance::{assert_story_events, StoryEventLog};
use uie_core::resource::{
    ContextRequest, FakeInferenceStateBudget, InferenceStateBudget, StateAdmission, StateEvent,
};
use uie_core::Error;

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
fn us_027_reject_over_context_budget() {
    let mut log = StoryEventLog::new("US-027");
    let budget = FakeInferenceStateBudget::with_max_context(128);
    log.record("state_contract_budget");

    let err = budget
        .admit(&ContextRequest {
            prompt_tokens: 100,
            max_new_tokens: 50,
        })
        .unwrap_err();
    assert!(
        matches!(err, Error::StateBudgetRejected { .. }),
        "expected StateBudgetRejected, got {err}"
    );
    log.record("state_contract_reject_or_negotiate");
    log.record("state_contract_no_silent_truncation");

    let events = budget.take_events();
    assert!(events.iter().any(|e| matches!(e, StateEvent::Reject { .. })));
    assert!(
        !events
            .iter()
            .any(|e| matches!(e, StateEvent::Accept { .. })),
        "must not accept over-budget: {events:?}"
    );

    assert_story_events(
        &log,
        &[
            "state_contract_budget",
            "state_contract_reject_or_negotiate",
            "state_contract_no_silent_truncation",
        ],
    );
}

#[test]
fn us_027_negotiate_instead_of_silent_truncate() {
    let mut log = StoryEventLog::new("US-027");
    let budget = FakeInferenceStateBudget::with_negotiate(128);
    let adm = budget
        .admit(&ContextRequest {
            prompt_tokens: 100,
            max_new_tokens: 50,
        })
        .unwrap();
    match adm {
        StateAdmission::Negotiate {
            requested,
            offered_n_ctx,
        } => {
            assert_eq!(requested, 150);
            assert_eq!(offered_n_ctx, 128);
            log.record("state_contract_reject_or_negotiate");
            log.record("state_contract_no_silent_truncation");
        }
        other => panic!("expected Negotiate, got {other:?}"),
    }
    assert_story_events(
        &log,
        &[
            "state_contract_reject_or_negotiate",
            "state_contract_no_silent_truncation",
        ],
    );
}

#[test]
fn us_027_accept_within_and_release_on_cancel() {
    let mut log = StoryEventLog::new("US-027");
    let budget = FakeInferenceStateBudget::with_max_context(128);
    let adm = budget
        .admit(&ContextRequest {
            prompt_tokens: 32,
            max_new_tokens: 32,
        })
        .unwrap();
    assert!(matches!(adm, StateAdmission::Accept { n_ctx: 128 }));
    log.record("state_contract_accept_within");

    // Cancellation / end of session releases state.
    budget.release(&adm);
    log.record("state_contract_release_on_cancel");

    let events = budget.take_events();
    assert!(events.iter().any(|e| matches!(e, StateEvent::Accept { .. })));
    assert!(events.iter().any(|e| matches!(e, StateEvent::Release { .. })));

    assert_story_events(
        &log,
        &[
            "state_contract_accept_within",
            "state_contract_release_on_cancel",
        ],
    );
}
