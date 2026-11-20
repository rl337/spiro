use std::env;
use std::path::{Path, PathBuf};

use uie_catalog::{expand_home, ModelCatalog, ModelRootConfig, ModelRootKind, ModelRootsFile};
use uie_core::Result;

pub fn flag_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1).cloned())
}

pub fn roots_file(repo_root: &Path) -> Result<ModelRootsFile> {
    let path = env::var("SPIRO_MODEL_ROOTS")
        .map(PathBuf::from)
        .unwrap_or_else(|_| repo_root.join("config/model_roots.yaml"));
    if path.is_file() {
        ModelRootsFile::load(&path)
    } else {
        Ok(default_roots())
    }
}

fn default_roots() -> ModelRootsFile {
    let home = env::var("HOME").unwrap_or_else(|_| ".".into());
    ModelRootsFile {
        roots: vec![ModelRootConfig {
            id: "lmstudio".into(),
            path: expand_home(format!("{home}/.lmstudio/models")),
            kind: ModelRootKind::LmStudio,
            max_depth: 8,
        }],
    }
}

pub fn find_qwen3_2b_path(catalog: &ModelCatalog) -> Result<PathBuf> {
    let needles = ["qwen", "2b"];
    for entry in catalog.registry().entries() {
        if entry.path.extension().is_some_and(|e| e == "gguf") {
            let hay = format!("{} {}", entry.id, entry.path.display()).to_lowercase();
            if needles.iter().all(|n| hay.contains(n)) {
                eprintln!("matched registry id: {}", entry.id);
                return Ok(entry.path.clone());
            }
        }
    }
    Err(uie_core::Error::ModelLoad(
        "no Qwen3 ~2B .gguf under SPIRO_MODEL_ROOTS — see docs/runbooks/qwen3-2b-e2e.md".into(),
    ))
}
