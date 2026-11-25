//! Load LM Studio–style `.gguf` weights as [US-010](https://github.com) text→text executors.

use uie_core::error::Result;
use uie_core::resource::InferenceStateBudget;

mod chat;
mod complete;
mod engine;
mod gpu;
mod loader;
mod model;
mod vision;

pub use chat::format_user_message_engine;
pub use complete::{
    complete_chat_user_message, complete_chat_user_message_budgeted, complete_from_prompt,
    complete_from_prompt_budgeted, complete_with_arc_budget, complete_with_engine_budgeted,
    CompleteOptions, CompleteResult,
};
pub use engine::{GgufEngine, GgufLoadOptions};
pub use gpu::{classify_ggml_devices, GgufGpuOffload, LiveGgmlAcceleratorProbe};

/// One-shot CLI helper: load GGUF then complete (still loads once per call).
pub fn complete_gguf_file(
    path: &std::path::Path,
    prompt: &str,
    opts: &CompleteOptions,
) -> Result<CompleteResult> {
    let engine = GgufEngine::load(path)?;
    complete_from_prompt(&engine, prompt, opts)
}

pub fn complete_gguf_file_budgeted(
    path: &std::path::Path,
    prompt: &str,
    opts: &CompleteOptions,
    budget: &dyn InferenceStateBudget,
) -> Result<CompleteResult> {
    complete_gguf_file_budgeted_offload(path, prompt, opts, budget, GgufGpuOffload::cpu_only())
}

pub fn complete_gguf_file_budgeted_offload(
    path: &std::path::Path,
    prompt: &str,
    opts: &CompleteOptions,
    budget: &dyn InferenceStateBudget,
    offload: GgufGpuOffload,
) -> Result<CompleteResult> {
    let engine = GgufEngine::load_with_offload(path, offload)?;
    complete_from_prompt_budgeted(&engine, prompt, opts, budget)
}

pub fn complete_gguf_file_chat(
    path: &std::path::Path,
    user_message: &str,
    opts: &CompleteOptions,
) -> Result<CompleteResult> {
    let engine = GgufEngine::load(path)?;
    complete_chat_user_message(&engine, user_message, opts)
}

pub fn complete_gguf_file_chat_budgeted(
    path: &std::path::Path,
    user_message: &str,
    opts: &CompleteOptions,
    budget: &dyn InferenceStateBudget,
) -> Result<CompleteResult> {
    complete_gguf_file_chat_budgeted_offload(
        path,
        user_message,
        opts,
        budget,
        GgufGpuOffload::cpu_only(),
    )
}

pub fn complete_gguf_file_chat_budgeted_offload(
    path: &std::path::Path,
    user_message: &str,
    opts: &CompleteOptions,
    budget: &dyn InferenceStateBudget,
    offload: GgufGpuOffload,
) -> Result<CompleteResult> {
    let engine = GgufEngine::load_with_offload(path, offload)?;
    complete_chat_user_message_budgeted(&engine, user_message, opts, budget)
}

pub use loader::GgufModelLoader;
pub use model::{prompt_tensor, GgufLlmExecutor};
pub use vision::{describe_image_mmproj, VisionDescribeOptions};
