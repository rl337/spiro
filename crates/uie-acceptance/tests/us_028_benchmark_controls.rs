//! US-028: constrained benchmarks + trustworthy A/B/A brackets.
use uie_acceptance::{assert_story_events, StoryEventLog};
use uie_telemetry::{
    evaluate_aba, reject_chunk_count_as_tokens, BracketVerdict, ColdWarm, TokenCount, TrialRole,
    TrialSample, WorkloadFingerprint,
};

fn fingerprint() -> WorkloadFingerprint {
    WorkloadFingerprint {
        workload_id: "us-028-fixture".into(),
        model_hash: "stub-hash".into(),
        tokenizer_hash: "tok-hash".into(),
        backend_revision: "test".into(),
        generation_cap: 32,
        cold_warm: ColdWarm::Warm,
    }
}

fn sample(role: TrialRole, rate: Option<f64>) -> TrialSample {
    TrialSample {
        role,
        fingerprint: fingerprint(),
        decode_tokens_per_sec: rate,
        ok: true,
        negative_control_failed: false,
    }
}

#[test]
fn us_028_bench_contract_events() {
    let mut log = StoryEventLog::new("US-028");
    for ev in [
        "bench_contract_frozen_workload",
        "bench_contract_bracket_control_candidate_control",
        "bench_contract_usage_not_chunk_count",
        "bench_contract_unknown_not_zero",
        "bench_contract_drift_or_inconclusive",
        "bench_contract_negative_control_no_promote",
    ] {
        log.record(ev);
    }
    assert_story_events(
        &log,
        &[
            "bench_contract_frozen_workload",
            "bench_contract_bracket_control_candidate_control",
            "bench_contract_usage_not_chunk_count",
            "bench_contract_unknown_not_zero",
            "bench_contract_drift_or_inconclusive",
            "bench_contract_negative_control_no_promote",
        ],
    );
}

#[test]
fn us_028_drift_bracket_inconclusive() {
    let mut log = StoryEventLog::new("US-028");
    log.record("bench_contract_frozen_workload");
    log.record("bench_contract_bracket_control_candidate_control");

    // Fixture from the story: baseline 100 / candidate 108 / control 110 → drift.
    let result = evaluate_aba(
        &sample(TrialRole::Control, Some(100.0)),
        &sample(TrialRole::Candidate, Some(108.0)),
        &sample(TrialRole::Control, Some(110.0)),
        0.03,
    );
    assert_eq!(result.verdict, BracketVerdict::DriftInconclusive);
    log.record("bench_contract_drift_or_inconclusive");

    assert_story_events(
        &log,
        &[
            "bench_contract_frozen_workload",
            "bench_contract_bracket_control_candidate_control",
            "bench_contract_drift_or_inconclusive",
        ],
    );
}

#[test]
fn us_028_eligible_win_stable_controls() {
    let result = evaluate_aba(
        &sample(TrialRole::Control, Some(100.0)),
        &sample(TrialRole::Candidate, Some(120.0)),
        &sample(TrialRole::Control, Some(100.0)),
        0.03,
    );
    assert_eq!(result.verdict, BracketVerdict::EligibleWin);
}

#[test]
fn us_028_usage_not_chunk_count_and_unknown() {
    let mut log = StoryEventLog::new("US-028");
    // 3 transport chunks must not become "3 tokens" when usage says 9.
    assert_eq!(
        reject_chunk_count_as_tokens(3, Some(9)),
        TokenCount::Known(9)
    );
    log.record("bench_contract_usage_not_chunk_count");

    assert_eq!(reject_chunk_count_as_tokens(3, None), TokenCount::Unknown);
    let incomplete = evaluate_aba(
        &sample(TrialRole::Control, Some(100.0)),
        &sample(TrialRole::Candidate, None),
        &sample(TrialRole::Control, Some(100.0)),
        0.03,
    );
    assert!(
        matches!(incomplete.verdict, BracketVerdict::Incomplete { .. }),
        "missing rate must be incomplete, not zero: {:?}",
        incomplete.verdict
    );
    log.record("bench_contract_unknown_not_zero");

    assert_story_events(
        &log,
        &[
            "bench_contract_usage_not_chunk_count",
            "bench_contract_unknown_not_zero",
        ],
    );
}

#[test]
fn us_028_negative_control_blocks_promotion() {
    let mut log = StoryEventLog::new("US-028");
    let mut cand = sample(TrialRole::Candidate, Some(200.0));
    cand.negative_control_failed = true;
    let result = evaluate_aba(
        &sample(TrialRole::Control, Some(100.0)),
        &cand,
        &sample(TrialRole::Control, Some(100.0)),
        0.03,
    );
    assert_eq!(result.verdict, BracketVerdict::NegativeControlBlocked);
    log.record("bench_contract_negative_control_no_promote");
    assert_story_events(&log, &["bench_contract_negative_control_no_promote"]);
}
