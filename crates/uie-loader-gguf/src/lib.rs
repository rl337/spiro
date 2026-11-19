//! Load LM Studio–style `.gguf` weights as [US-010](https://github.com) text→text executors.

use uie_core::error::Result;

mod complete;
mod engine;
mod loader;
mod model;

pub use complete::{complete_from_prompt, CompleteOptions, CompleteResult};
pub use engine::GgufEngine;

/// One-shot CLI helper: load GGUF then complete (still loads once per call).
pub fn complete_gguf_file(
    path: &std::path::Path,
    prompt: &str,
    opts: &CompleteOptions,
) -> Result<CompleteResult> {
    let engine = GgufEngine::load(path)?;
    complete_from_prompt(&engine, prompt, opts)
}
pub use loader::GgufModelLoader;
pub use model::{prompt_tensor, GgufLlmExecutor};
