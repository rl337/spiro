//! US-006
use uie_acceptance::{assert_story_events, StoryEventLog};
use uie_core::ops::OpKind;

#[test]
fn us_006_fill_op_is_constructible() {
    let mut log = StoryEventLog::new("US-006");
    let _ = OpKind::Fill { value: 1.0 };
    log.record("op_kind_fill_constructed");
    assert_story_events(&log, &["op_kind_fill_constructed"]);
}
