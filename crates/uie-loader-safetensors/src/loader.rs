use std::path::Path;

use uie_catalog::{ModelEntry, ModelFormat};
use uie_core::backend::Backend;
use uie_core::error::Result;
use uie_core::model::{InferenceExecutor, ModelLoader};

use crate::executor::SafetensorsExecutor;
use crate::role::modality_for_catalog_entry;

#[derive(Debug, Default, Clone, Copy)]
pub struct SafetensorsModelLoader;

impl ModelLoader for SafetensorsModelLoader {
    fn format_id(&self) -> &str {
        "safetensors"
    }

    fn can_load(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| e.eq_ignore_ascii_case("safetensors"))
    }

    fn load(&self, path: &Path, _backend: &dyn Backend) -> Result<Box<dyn InferenceExecutor>> {
        let kind = crate::role::guess_from_path(path);
        Ok(Box::new(SafetensorsExecutor::from_path(path, kind)?))
    }
}

impl SafetensorsModelLoader {
    /// Load using registry role/format (preferred for Comfy/LM Studio ids).
    pub fn load_catalog_entry(
        &self,
        entry: &ModelEntry,
        backend: &dyn Backend,
    ) -> Result<Box<dyn InferenceExecutor>> {
        let _ = backend;
        if entry.format != ModelFormat::Safetensors {
            return Err(uie_core::error::Error::ModelLoad(format!(
                "expected safetensors entry, got {:?}",
                entry.format
            )));
        }
        let kind = modality_for_catalog_entry(entry);
        Ok(Box::new(SafetensorsExecutor::from_path(&entry.path, kind)?))
    }
}
