//! Model discovery via pluggable detectors (directory scan + test overlays).

use crate::registry::{ModelEntry, ScanStats};
use crate::roots::ModelRootsFile;
use crate::scan::scan_roots;

/// Discovers [`ModelEntry`] items under configured roots (and optional overlays).
pub trait ModelDetector: Send + Sync {
    fn detect(&self, roots: &ModelRootsFile) -> (Vec<ModelEntry>, ScanStats);
}

/// Walks LM Studio / ComfyUI / generic directory layouts (US-021).
#[derive(Debug, Default, Clone, Copy)]
pub struct DirectoryModelDetector;

impl ModelDetector for DirectoryModelDetector {
    fn detect(&self, roots: &ModelRootsFile) -> (Vec<ModelEntry>, ScanStats) {
        scan_roots(roots)
    }
}

/// Directory scan of the workspace [`models/mock`](../../../models/mock) tree (LM + Comfy layouts).
#[derive(Debug, Clone)]
pub struct TestModelDetector {
    inner: DirectoryModelDetector,
    mock_roots: ModelRootsFile,
}

impl TestModelDetector {
    pub fn with_mock_roots(mock_roots: ModelRootsFile) -> Self {
        Self {
            inner: DirectoryModelDetector,
            mock_roots,
        }
    }

    /// `models/mock/lmstudio` + `models/mock/comfyui` under `repo_root`.
    pub fn workspace_mock(repo_root: &std::path::Path) -> Self {
        Self::with_mock_roots(crate::mock::mock_model_roots(repo_root))
    }
}

impl ModelDetector for TestModelDetector {
    fn detect(&self, roots: &ModelRootsFile) -> (Vec<ModelEntry>, ScanStats) {
        let (mut entries, mut stats) = self.inner.detect(roots);
        let (mock_entries, mock_stats) = self.inner.detect(&self.mock_roots);
        entries.extend(mock_entries);
        stats.roots_scanned += mock_stats.roots_scanned;
        stats.roots_skipped += mock_stats.roots_skipped;
        stats.files_indexed += mock_stats.files_indexed;
        entries.sort_by(|a, b| a.id.cmp(&b.id));
        (entries, stats)
    }
}
