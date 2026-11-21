use std::path::PathBuf;

use uie_catalog::{
    DirectoryModelDetector, ModelFormat, ModelRegistry, ModelRole, ModelRootConfig, ModelRootKind,
    ModelRootsFile, TestModelDetector,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn test_detector_includes_workspace_mock_stubs() {
    let roots = ModelRootsFile {
        roots: vec![ModelRootConfig {
            id: "empty".into(),
            path: repo_root().join("models/mock/nonexistent"),
            kind: ModelRootKind::Generic,
            max_depth: 2,
        }],
    };
    let detector = TestModelDetector::workspace_mock(&repo_root());
    let reg = ModelRegistry::refresh_with_detector(&roots, &detector);
    let mut log = uie_telemetry::StoryEventLog::new("US-021");
    assert!(
        reg.entries()
            .iter()
            .any(|e| e.id.starts_with("mock-lmstudio:") && e.format == ModelFormat::Stub),
        "expected mock LM studio stub"
    );
    assert!(reg.entries().iter().any(|e| {
        e.root_id == "mock-comfy"
            && e.role == ModelRole::DiffusionCheckpoint
            && e.format == ModelFormat::Stub
    }));
    log.record("registry_mock_detector");
    uie_telemetry::assert_story_events(&log, &["registry_mock_detector"]);
}

#[test]
fn directory_detector_excludes_mock_overlay() {
    let roots = ModelRootsFile::default();
    let reg = ModelRegistry::refresh_with_detector(&roots, &DirectoryModelDetector);
    assert!(reg
        .entries()
        .iter()
        .all(|e| !e.root_id.starts_with("mock-")));
}
