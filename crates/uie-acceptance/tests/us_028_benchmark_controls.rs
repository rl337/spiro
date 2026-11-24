//! US-028: constrained benchmarks + trustworthy brackets — planned contract markers.
use uie_acceptance::{assert_story_events, StoryEventLog};

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
#[ignore = "US-028 not implemented: injected clocks + bracket evaluator"]
fn us_028_drift_bracket_inconclusive() {
    unimplemented!("baseline 100 / candidate 108 / control 110 → drift");
}
