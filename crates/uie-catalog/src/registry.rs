use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::detector::{DirectoryModelDetector, ModelDetector};
use crate::roots::ModelRootsFile;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelFormat {
    Gguf,
    Onnx,
    Safetensors,
    Stub,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelRole {
    Llm,
    Judge,
    DiffusionCheckpoint,
    DiffusionUnet,
    TextEncoder,
    Vae,
    Lora,
    ControlNet,
    Upscaler,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelEntry {
    /// Stable id, e.g. `lmstudio:meta-llama/Llama-3.2-1B/foo.gguf`.
    pub id: String,
    pub root_id: String,
    pub path: PathBuf,
    pub format: ModelFormat,
    pub role: ModelRole,
    /// Publisher / folder grouping for stack assembly (Comfy checkpoint base name, LM Studio publisher).
    pub family: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScanStats {
    pub roots_scanned: usize,
    pub roots_skipped: usize,
    pub files_indexed: usize,
}

/// Scanned index of weights under configured roots.
#[derive(Debug, Clone, Default)]
pub struct ModelRegistry {
    entries: Vec<ModelEntry>,
    by_id: HashMap<String, usize>,
    stats: ScanStats,
}

impl ModelRegistry {
    pub fn refresh_from_file(path: &std::path::Path) -> uie_core::Result<Self> {
        let file = ModelRootsFile::load(path)?;
        Ok(Self::refresh(&file))
    }

    pub fn refresh(roots: &ModelRootsFile) -> Self {
        Self::refresh_with_detector(roots, &DirectoryModelDetector)
    }

    pub fn refresh_with_detector(roots: &ModelRootsFile, detector: &dyn ModelDetector) -> Self {
        let (entries, stats) = detector.detect(roots);
        let mut by_id = HashMap::new();
        for (i, e) in entries.iter().enumerate() {
            by_id.insert(e.id.clone(), i);
        }
        Self {
            entries,
            by_id,
            stats,
        }
    }

    pub fn entries(&self) -> &[ModelEntry] {
        &self.entries
    }

    pub fn get(&self, id: &str) -> Option<&ModelEntry> {
        self.by_id.get(id).map(|&i| &self.entries[i])
    }

    pub fn stats(&self) -> &ScanStats {
        &self.stats
    }

    pub fn by_role(&self, role: ModelRole) -> impl Iterator<Item = &ModelEntry> {
        self.entries.iter().filter(move |e| e.role == role)
    }
}

/// Roots + registry with explicit rescan.
#[derive(Debug, Clone)]
pub struct ModelCatalog {
    roots: ModelRootsFile,
    registry: ModelRegistry,
}

impl ModelCatalog {
    pub fn load(path: &std::path::Path) -> uie_core::Result<Self> {
        let roots = ModelRootsFile::load(path)?;
        let registry = ModelRegistry::refresh(&roots);
        Ok(Self { roots, registry })
    }

    pub fn from_roots(roots: ModelRootsFile) -> Self {
        Self::from_roots_with_detector(roots, &DirectoryModelDetector)
    }

    pub fn from_roots_with_detector(roots: ModelRootsFile, detector: &dyn ModelDetector) -> Self {
        let registry = ModelRegistry::refresh_with_detector(&roots, detector);
        Self { roots, registry }
    }

    pub fn rescan(&mut self) {
        self.registry = ModelRegistry::refresh(&self.roots);
    }

    pub fn rescan_with_detector(&mut self, detector: &dyn ModelDetector) {
        self.registry = ModelRegistry::refresh_with_detector(&self.roots, detector);
    }

    pub fn roots(&self) -> &ModelRootsFile {
        &self.roots
    }

    pub fn registry(&self) -> &ModelRegistry {
        &self.registry
    }

    pub fn resolve_path(&self, id: &str) -> Option<&PathBuf> {
        self.registry.get(id).map(|e| &e.path)
    }
}
