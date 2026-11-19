//! US-021
use uie_acceptance::{assert_story_events, StoryEventLog};
use uie_catalog::{ModelRegistry, ModelRootConfig, ModelRootKind, ModelRootsFile};

#[test]
fn us_021_scans_directory_roots_only() {
    let mut log = StoryEventLog::new("US-021");
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("lm");
    std::fs::create_dir_all(root.join("pub/m")).unwrap();
    std::fs::write(root.join("pub/m/w.gguf"), b"g").unwrap();

    let reg = ModelRegistry::refresh(&ModelRootsFile {
        roots: vec![ModelRootConfig {
            id: "t".into(),
            path: root,
            kind: ModelRootKind::LmStudio,
            max_depth: 5,
        }],
    });
    assert_eq!(reg.entries().len(), 1);
    log.record("registry_scan_entry");
    assert!(reg.entries()[0].id.starts_with("t:"));
    log.record("registry_entry_id_prefix");
    assert_story_events(&log, &["registry_scan_entry", "registry_entry_id_prefix"]);
}