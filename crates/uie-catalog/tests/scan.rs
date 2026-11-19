use std::path::PathBuf;

use uie_catalog::{ModelFormat, ModelRegistry, ModelRole, ModelRootConfig, ModelRootKind, ModelRootsFile};

#[test]
fn scans_comfy_and_lmstudio_layouts() {
    let dir = tempfile::tempdir().unwrap();
    let lm = dir.path().join("lm");
    std::fs::create_dir_all(lm.join("TheBloke/mistral-7b")).unwrap();
    std::fs::write(lm.join("TheBloke/mistral-7b/model.gguf"), b"gguf").unwrap();

    let comfy = dir.path().join("comfy");
    std::fs::create_dir_all(comfy.join("checkpoints")).unwrap();
    std::fs::create_dir_all(comfy.join("vae")).unwrap();
    std::fs::write(comfy.join("checkpoints/sd_xl.safetensors"), b"st").unwrap();
    std::fs::write(comfy.join("vae/vae.safetensors"), b"st").unwrap();

    let roots = ModelRootsFile {
        roots: vec![
            ModelRootConfig {
                id: "lm".into(),
                path: lm,
                kind: ModelRootKind::LmStudio,
                max_depth: 6,
            },
            ModelRootConfig {
                id: "c".into(),
                path: comfy,
                kind: ModelRootKind::ComfyUi,
                max_depth: 4,
            },
        ],
    };

    let reg = ModelRegistry::refresh(&roots);
    let mut log = uie_telemetry::StoryEventLog::new("US-021");
    assert_eq!(reg.stats().files_indexed, 3);
    log.record("registry_scan_multi_layout");
    assert!(reg.entries().iter().any(|e| {
        e.format == ModelFormat::Gguf && e.role == ModelRole::Llm
    }));
    assert!(reg.entries().iter().any(|e| {
        e.role == ModelRole::DiffusionCheckpoint && e.family.as_deref() == Some("sd_xl")
    }));
    log.record("registry_roles_classified");
    uie_telemetry::assert_story_events(&log, &["registry_scan_multi_layout", "registry_roles_classified"]);
}