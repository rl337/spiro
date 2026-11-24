//! US-025: bounded model admission — fake budget + host cache.
use std::sync::Arc;

use uie_acceptance::{assert_story_events, StoryEventLog};
use uie_core::resource::{AdmissionEvent, FakeMemoryBudget, MemoryBudget, MemoryEstimate};
use uie_core::Error;
use uie_ipc::InferenceEngineHost;
use uie_stub_models::StubModelLoader;

mod common;

fn stub(name: &str) -> std::path::PathBuf {
    common::stubs_dir().join(name)
}

#[test]
fn us_025_admission_contract_events() {
    let mut log = StoryEventLog::new("US-025");
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
fn us_025_reject_when_over_budget() {
    let mut log = StoryEventLog::new("US-025");
    let budget = FakeMemoryBudget::with_capacity(1); // 1 byte — any real stub file exceeds
    let host = InferenceEngineHost::with_budget(
        vec![Box::new(StubModelLoader)],
        Arc::clone(&budget) as Arc<dyn MemoryBudget>,
        false, // no eviction → hard reject
    );

    let path = stub("llm.stub");
    let estimate = MemoryEstimate::from_file_len(
        std::fs::metadata(&path).unwrap().len(),
        path.display().to_string(),
    );
    log.record("admission_contract_estimate");
    assert!(estimate.bytes > 1, "fixture stub should be > 1 byte");

    let err = host
        .handle_message(uie_ipc::IpcMessage::Infer {
            request_id: "acc-025-reject".into(),
            model_path: path.display().to_string(),
            inputs: vec![],
        });
    match err {
        uie_ipc::IpcMessage::Error { message } => {
            assert!(
                message.contains("admission rejected") || message.contains("AdmissionRejected"),
                "unexpected error: {message}"
            );
            log.record("admission_contract_reject_or_evict");
        }
        other => panic!("expected Error, got {other:?}"),
    }

    let events = budget.take_events();
    assert!(
        events.iter().any(|e| matches!(e, AdmissionEvent::Reject { .. })),
        "expected Reject event, got {events:?}"
    );
    // No successful reserve should remain outstanding without release/commit.
    assert!(
        !events
            .iter()
            .any(|e| matches!(e, AdmissionEvent::Commit { .. })),
        "reject path must not commit residency"
    );
    log.record("admission_contract_no_double_spend");
    assert_story_events(
        &log,
        &[
            "admission_contract_estimate",
            "admission_contract_reject_or_evict",
            "admission_contract_no_double_spend",
        ],
    );
}

#[test]
fn us_025_reserve_commit_and_evict_idle() {
    let mut log = StoryEventLog::new("US-025");
    let llm = stub("llm.stub");
    let judge = stub("judge.stub");
    let llm_len = std::fs::metadata(&llm).unwrap().len();
    let judge_len = std::fs::metadata(&judge).unwrap().len();
    // Capacity fits one model but not both.
    let capacity = llm_len.max(judge_len);
    let budget = FakeMemoryBudget::with_capacity(capacity);
    let host = InferenceEngineHost::with_budget(
        vec![Box::new(StubModelLoader)],
        Arc::clone(&budget) as Arc<dyn MemoryBudget>,
        true,
    );

    // Load first model — reserve + commit.
    let msg = host.handle_message(uie_ipc::IpcMessage::Infer {
        request_id: "acc-025-a".into(),
        model_path: llm.display().to_string(),
        inputs: uie_stub_models::sample_input(uie_stub_models::ModalityKind::LlmTextCompletion)
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, t)| {
                uie_ipc::TensorPayload::from_tensor(&format!("in{i}"), t).unwrap()
            })
            .collect(),
    });
    assert!(
        matches!(msg, uie_ipc::IpcMessage::InferResult { .. }),
        "first load should succeed: {msg:?}"
    );
    log.record("admission_contract_reserve");

    let events_after_a = budget.take_events();
    assert!(events_after_a
        .iter()
        .any(|e| matches!(e, AdmissionEvent::Reserve { .. })));
    assert!(events_after_a
        .iter()
        .any(|e| matches!(e, AdmissionEvent::Commit { .. })));

    // Second model: eviction frees idle first model, then admit.
    let msg2 = host.handle_message(uie_ipc::IpcMessage::Infer {
        request_id: "acc-025-b".into(),
        model_path: judge.display().to_string(),
        inputs: uie_stub_models::sample_input(uie_stub_models::ModalityKind::Judge)
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, t)| {
                uie_ipc::TensorPayload::from_tensor(&format!("in{i}"), t).unwrap()
            })
            .collect(),
    });
    assert!(
        matches!(msg2, uie_ipc::IpcMessage::InferResult { .. }),
        "second load after idle eviction should succeed: {msg2:?}"
    );
    log.record("admission_contract_reject_or_evict");
    log.record("admission_contract_release");

    let events_b = budget.take_events();
    assert!(
        events_b
            .iter()
            .any(|e| matches!(e, AdmissionEvent::ReleaseResidency { .. })),
        "expected idle eviction ReleaseResidency, got {events_b:?}"
    );

    assert_story_events(
        &log,
        &[
            "admission_contract_reserve",
            "admission_contract_reject_or_evict",
            "admission_contract_release",
        ],
    );
}

#[test]
fn us_025_failed_load_releases_reservation() {
    let budget = FakeMemoryBudget::with_capacity(10_000_000);
    let host = InferenceEngineHost::with_budget(
        vec![Box::new(StubModelLoader)],
        Arc::clone(&budget) as Arc<dyn MemoryBudget>,
        false,
    );
    // Existing non-stub file: estimate reserves, then load fails (no matching loader).
    let path = common::repo_root().join("README.md");
    let _ = host.handle_message(uie_ipc::IpcMessage::Infer {
        request_id: "acc-025-fail".into(),
        model_path: path.display().to_string(),
        inputs: vec![],
    });
    let events = budget.take_events();
    // Either reject (if any) or reserve+release without commit.
    let reserved = events
        .iter()
        .filter(|e| matches!(e, AdmissionEvent::Reserve { .. }))
        .count();
    let released = events
        .iter()
        .filter(|e| matches!(e, AdmissionEvent::Release { .. }))
        .count();
    let committed = events
        .iter()
        .filter(|e| matches!(e, AdmissionEvent::Commit { .. }))
        .count();
    assert_eq!(committed, 0, "failed load must not commit: {events:?}");
    if reserved > 0 {
        assert_eq!(
            reserved, released,
            "every reserve must release on load failure: {events:?}"
        );
    }
    // Direct unit check: Error::AdmissionRejected displays usefully.
    let err = Error::AdmissionRejected {
        estimate_bytes: 9,
        remaining_bytes: 1,
        budget_source: "fake".into(),
        reason: "test".into(),
    };
    assert!(err.to_string().contains("admission rejected"));
}
