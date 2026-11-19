//! US-021
use uie_catalog::{ModelRegistry, ModelRootConfig, ModelRootKind, ModelRootsFile};

#[test]
fn us_021_scans_directory_roots_only() {
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
    assert!(reg.entries()[0].id.starts_with("t:"));
}