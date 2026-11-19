//! Load LM Studio–style `.gguf` weights as [US-010](https://github.com) text→text executors.

use uie_core::error::Result;

mod chat;
mod complete;
mod engine;
mod loader;
mod model;

pub use chat::format_user_message_engine;
pub use complete::{
    complete_chat_user_message, complete_from_prompt, CompleteOptions, CompleteResult,
};
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

pub fn complete_gguf_file_chat(
    path: &std::path::Path,
    user_message: &str,
    opts: &CompleteOptions,
) -> Result<CompleteResult> {
    let engine = GgufEngine::load(path)?;
    complete_chat_user_message(&engine, user_message, opts)
}
pub use loader::GgufModelLoader;
pub use model::{prompt_tensor, GgufLlmExecutor};
