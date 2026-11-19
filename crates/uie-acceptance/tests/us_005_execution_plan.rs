//! US-005
use uie_acceptance::{assert_story_events, StoryEventLog};
use uie_core::model::Graph;
use uie_core::ops::{Op, OpKind};

#[test]
fn us_005_graph_to_plan_has_ops() {
    let mut log = StoryEventLog::new("US-005");
    let g = Graph {
        inputs: vec!["a".into()],
        outputs: vec!["b".into()],
        ops: vec![Op::noop("fill")],
    };
    let plan = g.to_plan();
    assert_eq!(plan.ops.len(), 1);
    log.record("execution_plan_emitted");
    assert_story_events(&log, &["execution_plan_emitted"]);
}