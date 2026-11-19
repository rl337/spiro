use std::path::Path;

use tracing::info;
use uie_core::backend::Backend;
use uie_core::error::Result;
use uie_core::model::{InferenceExecutor, ModelLoader};

use crate::executor::StubExecutor;
use crate::modality::ModalityKind;

pub struct StubModelLoader;

impl Default for StubModelLoader {
    fn default() -> Self {
        Self
    }
}

impl ModelLoader for StubModelLoader {
    fn format_id(&self) -> &str {
        "stub"
    }

    fn can_load(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| e.eq_ignore_ascii_case("stub"))
    }

    fn load(&self, path: &Path, backend: &dyn Backend) -> Result<Box<dyn InferenceExecutor>> {
        let kind = ModalityKind::parse_file(path)?;
        info!(
            path = %path.display(),
            modality = %kind.id_str(),
            story = kind.story_id(),
            backend = %backend.id(),
            "loaded stub model"
        );
        Ok(Box::new(StubExecutor::new(kind)))
    }
}

impl StubModelLoader {
    pub fn load_kind(
        &self,
        kind: ModalityKind,
        backend: &dyn Backend,
    ) -> Result<Box<dyn InferenceExecutor>> {
        let _ = self;
        let _ = backend;
        Ok(Box::new(StubExecutor::new(kind)))
    }
}
