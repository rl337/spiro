//! US-001: workspace ships core crates.
mod common;

use uie_acceptance::{assert_story_events, StoryEventLog};

#[test]
fn us_001_workspace_has_core_and_demo_crates() {
    let mut log = StoryEventLog::new("US-001");
    let root = common::repo_root();
    assert!(root.join("crates/uie-core").is_dir());
    log.record("uie_core_present");
    assert!(root.join("crates/uie-spiro").is_dir());
    log.record("uie_spiro_present");
    assert!(root.join("docs/stories/001-why-uie.md").is_file());
    log.record("story_doc_present");
    let mandate = root.join("docs/mandate.md");
    assert!(mandate.is_file(), "missing docs/mandate.md");
    let mandate_text = std::fs::read_to_string(&mandate).expect("read mandate");
    assert!(
        mandate_text.to_lowercase().contains("edge")
            && mandate_text.to_lowercase().contains("embedded"),
        "mandate must emphasize edge and embedded"
    );
    log.record("mandate_edge_embedded");
    assert_story_events(
        &log,
        &[
            "uie_core_present",
            "story_doc_present",
            "mandate_edge_embedded",
        ],
    );
}
