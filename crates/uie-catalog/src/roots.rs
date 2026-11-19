use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use uie_core::error::{Error, Result};

/// Well-known layout hints; roots are still **directories only** in config.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelRootKind {
    /// `~/.lmstudio/models` — nested `publisher/repo/*.gguf`.
    LmStudio,
    /// ComfyUI `models/` tree (`checkpoints/`, `vae/`, …).
    ComfyUi,
    /// Any tree: scan by file extension only.
    Generic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRootConfig {
    pub id: String,
    pub path: PathBuf,
    #[serde(default = "default_kind")]
    pub kind: ModelRootKind,
    #[serde(default = "default_max_depth")]
    pub max_depth: usize,
}

fn default_kind() -> ModelRootKind {
    ModelRootKind::Generic
}

fn default_max_depth() -> usize {
    8
}

/// On-disk file: **only directories**, no per-model list.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModelRootsFile {
    #[serde(default)]
    pub roots: Vec<ModelRootConfig>,
}

impl ModelRootsFile {
    pub fn load(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| Error::ModelLoad(format!("read {}: {e}", path.display())))?;
        serde_yaml::from_str(&text).map_err(|e| Error::ModelLoad(e.to_string()))
    }

    pub fn default_laptop() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        Self {
            roots: vec![
                ModelRootConfig {
                    id: "lmstudio".into(),
                    path: expand_home(format!("{home}/.lmstudio/models")),
                    kind: ModelRootKind::LmStudio,
                    max_depth: 6,
                },
                ModelRootConfig {
                    id: "comfy".into(),
                    path: PathBuf::from("models"),
                    kind: ModelRootKind::ComfyUi,
                    max_depth: 4,
                },
            ],
        }
    }
}

pub fn expand_home(path: impl Into<PathBuf>) -> PathBuf {
    let p = path.into();
    if let Some(s) = p.to_str() {
        if let Some(rest) = s.strip_prefix("~/") {
            if let Ok(home) = std::env::var("HOME") {
                return PathBuf::from(home).join(rest);
            }
        }
        if s == "~" {
            if let Ok(home) = std::env::var("HOME") {
                return PathBuf::from(home);
            }
        }
    }
    p
}