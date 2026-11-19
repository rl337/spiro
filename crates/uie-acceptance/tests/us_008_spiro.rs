//! US-008
use uie_acceptance::{assert_story_events, StoryEventLog};

#[test]
fn us_008_spiro_crate_has_main() {
    let mut log = StoryEventLog::new("US-008");
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/uie-spiro/src/main.rs");
    assert!(root.is_file());
    log.record("spiro_demo_entrypoint");
    assert_story_events(&log, &["spiro_demo_entrypoint"]);
}