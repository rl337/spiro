//! US-001: workspace ships core crates.
mod support;

#[test]
fn us_001_workspace_has_core_and_demo_crates() {
    let root = support::repo_root();
    assert!(root.join("crates/uie-core").is_dir());
    assert!(root.join("crates/uie-spiro").is_dir());
    assert!(root.join("docs/stories/001-why-uie.md").is_file());
}