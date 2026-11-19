//! Load-once GGUF weights (shared across `infer` calls).

use std::path::{Path, PathBuf};

use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::LlamaModel;

use uie_core::error::{Error, Result};

pub struct GgufEngine {
    pub path: PathBuf,
    pub backend: LlamaBackend,
    pub model: LlamaModel,
}

impl GgufEngine {
    pub fn load(path: &Path) -> Result<Self> {
        let backend =
            LlamaBackend::init().map_err(|e| Error::ModelLoad(format!("llama backend: {e}")))?;
        let model_params = LlamaModelParams::default();
        let model = LlamaModel::load_from_file(&backend, path, &model_params)
            .map_err(|e| Error::ModelLoad(format!("load gguf {}: {e}", path.display())))?;
        Ok(Self {
            path: path.to_path_buf(),
            backend,
            model,
        })
    }
}
