//! Mock model roots: stub weights laid out like LM Studio and ComfyUI trees.

use std::path::{Path, PathBuf};

use crate::roots::{ModelRootConfig, ModelRootKind, ModelRootsFile};

/// Registry overlay paths under `repo_root/models/mock/`.
pub fn mock_model_roots(repo_root: &Path) -> ModelRootsFile {
    let base = repo_root.join("models/mock");
    ModelRootsFile {
        roots: vec![
            ModelRootConfig {
                id: "mock-lmstudio".into(),
                path: base.join("lmstudio"),
                kind: ModelRootKind::LmStudio,
                max_depth: 6,
            },
            ModelRootConfig {
                id: "mock-comfy".into(),
                path: base.join("comfyui"),
                kind: ModelRootKind::ComfyUi,
                max_depth: 5,
            },
        ],
    }
}

pub fn mock_roots_base(repo_root: &Path) -> PathBuf {
    repo_root.join("models/mock")
}
