//! Load-once GGUF weights (shared across `infer` calls).

use std::path::{Path, PathBuf};

use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::LlamaModel;

use uie_core::accelerator::AcceleratorKind;
use uie_core::error::{Error, Result};

use crate::gpu::GgufGpuOffload;

/// Options for loading GGUF weights (US-030 GPU offload).
#[derive(Debug, Clone, Copy)]
pub struct GgufLoadOptions {
    /// Layers placed on GPU; `0` keeps weights on CPU.
    pub n_gpu_layers: u32,
}

impl Default for GgufLoadOptions {
    fn default() -> Self {
        Self { n_gpu_layers: 0 }
    }
}

impl From<GgufGpuOffload> for GgufLoadOptions {
    fn from(offload: GgufGpuOffload) -> Self {
        Self {
            n_gpu_layers: offload.n_gpu_layers,
        }
    }
}

pub struct GgufEngine {
    pub path: PathBuf,
    pub backend: LlamaBackend,
    pub model: LlamaModel,
    /// Layers requested at load (`0` = CPU-only).
    pub n_gpu_layers: u32,
    /// Accelerator kind used for offload, if any.
    pub offload_kind: Option<AcceleratorKind>,
}

impl GgufEngine {
    pub fn load(path: &Path) -> Result<Self> {
        Self::load_with_options(path, GgufLoadOptions::default(), None)
    }

    pub fn load_with_offload(path: &Path, offload: GgufGpuOffload) -> Result<Self> {
        Self::load_with_options(path, GgufLoadOptions::from(offload), offload.kind)
    }

    pub fn load_with_options(
        path: &Path,
        opts: GgufLoadOptions,
        offload_kind: Option<AcceleratorKind>,
    ) -> Result<Self> {
        let backend =
            LlamaBackend::init().map_err(|e| Error::ModelLoad(format!("llama backend: {e}")))?;
        let mut model_params = LlamaModelParams::default();
        if opts.n_gpu_layers > 0 {
            if !backend.supports_gpu_offload() {
                return Err(Error::UnsupportedOp(format!(
                    "n_gpu_layers={} requested but llama reports no GPU offload (US-030)",
                    opts.n_gpu_layers
                )));
            }
            model_params = model_params.with_n_gpu_layers(opts.n_gpu_layers);
        }
        let model = LlamaModel::load_from_file(&backend, path, &model_params)
            .map_err(|e| Error::ModelLoad(format!("load gguf {}: {e}", path.display())))?;
        Ok(Self {
            path: path.to_path_buf(),
            backend,
            model,
            n_gpu_layers: opts.n_gpu_layers,
            offload_kind,
        })
    }

    pub fn gpu_offload_active(&self) -> bool {
        self.n_gpu_layers > 0
    }
}
