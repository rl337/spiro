//! US-001: workspace ships core crates.
mod support;

use uie_acceptance::{assert_story_events, StoryEventLog};

#[test]
fn us_001_workspace_has_core_and_demo_crates() {
    let mut log = StoryEventLog::new("US-001");
    let root = support::repo_root();
    assert!(root.join("crates/uie-core").is_dir());
    log.record("uie_core_present");
    assert!(root.join("crates/uie-spiro").is_dir());
    log.record("uie_spiro_present");
    assert!(root.join("docs/stories/001-why-uie.md").is_file());
    log.record("story_doc_present");
    assert_story_events(&log, &["uie_core_present", "story_doc_present"]);
}