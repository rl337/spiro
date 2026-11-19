//! US-008
use uie_acceptance::{assert_story_events, StoryEventLog};

#[test]
fn us_008_spiro_crate_has_main() {
    let mut log = StoryEventLog::new("US-008");
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/uie-spiro/src/main.rs");
    assert!(root.is_file());
    log.record("spiro_demo_entrypoint");
    let mod_rs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/uie-spiro/src/spiro.rs");
    assert!(mod_rs.is_file());
    log.record("spiro_module_sources_present");
    assert_story_events(&log, &["spiro_demo_entrypoint", "spiro_module_sources_present"]);
}