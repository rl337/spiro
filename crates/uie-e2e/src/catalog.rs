//! Shared catalog construction for E2E (optional mock overlay).

use std::env;
use std::path::Path;

use uie_catalog::{ModelCatalog, ModelRootsFile, TestModelDetector};

pub fn include_mock_overlay() -> bool {
    !matches!(
        env::var("UIE_E2E_MOCK_MODELS").as_deref(),
        Ok("0") | Ok("false") | Ok("no")
    )
}

pub fn roots_from_env(repo_root: &Path) -> uie_core::Result<ModelRootsFile> {
    let path = env::var("SPIRO_MODEL_ROOTS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| repo_root.join("config/model_roots.yaml"));
    if path.is_file() {
        ModelRootsFile::load(&path)
    } else {
        Ok(ModelRootsFile::default())
    }
}

pub fn catalog_for_e2e(repo_root: &Path) -> uie_core::Result<ModelCatalog> {
    let roots = roots_from_env(repo_root)?;
    if include_mock_overlay() {
        let detector = TestModelDetector::workspace_mock(repo_root);
        Ok(ModelCatalog::from_roots_with_detector(roots, &detector))
    } else {
        Ok(ModelCatalog::from_roots(roots))
    }
}

pub fn catalog_with_overlay(repo_root: &Path, roots: ModelRootsFile, mock: bool) -> ModelCatalog {
    if mock {
        let detector = TestModelDetector::workspace_mock(repo_root);
        ModelCatalog::from_roots_with_detector(roots, &detector)
    } else {
        ModelCatalog::from_roots(roots)
    }
}
