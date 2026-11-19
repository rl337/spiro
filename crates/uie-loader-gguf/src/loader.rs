use std::path::Path;

use tracing::info;
use uie_core::backend::Backend;
use uie_core::error::Result;
use uie_core::model::{InferenceExecutor, ModelLoader};

use crate::model::GgufLlmExecutor;

pub struct GgufModelLoader {
    pub max_new_tokens: usize,
}

impl GgufModelLoader {
    pub fn new() -> Self {
        Self {
            max_new_tokens: 64,
        }
    }

    pub fn with_max_new_tokens(mut self, n: usize) -> Self {
        self.max_new_tokens = n;
        self
    }
}

impl Default for GgufModelLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelLoader for GgufModelLoader {
    fn format_id(&self) -> &str {
        "gguf"
    }

    fn can_load(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| e.eq_ignore_ascii_case("gguf"))
    }

    fn load(&self, path: &Path, backend: &dyn Backend) -> Result<Box<dyn InferenceExecutor>> {
        let _ = backend;
        info!(path = %path.display(), "loading GGUF LLM");
        let exec = GgufLlmExecutor::from_path(path, self.max_new_tokens)?;
        Ok(Box::new(exec))
    }
}