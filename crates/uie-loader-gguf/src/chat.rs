//! Apply GGUF-baked chat templates (Qwen3, ChatML, etc.) via llama.cpp.

use llama_cpp_2::model::{LlamaChatMessage, LlamaModel};
use uie_core::error::{Error, Result};

use crate::engine::GgufEngine;

/// Format a single user turn using the model's default chat template (`add_ass` = assistant header).
pub fn format_user_message(model: &LlamaModel, user_message: &str) -> Result<String> {
    let tmpl = model
        .chat_template(None)
        .map_err(|e| Error::Runtime(format!("model has no chat template: {e:?}")))?;
    let msg = LlamaChatMessage::new("user".into(), user_message.into())
        .map_err(|e| Error::Runtime(format!("chat message: {e:?}")))?;
    model
        .apply_chat_template(&tmpl, &[msg], true)
        .map_err(|e| Error::Runtime(format!("apply_chat_template: {e:?}")))
}

/// Same as [`format_user_message`] but takes a loaded [`GgufEngine`].
pub fn format_user_message_engine(engine: &GgufEngine, user_message: &str) -> Result<String> {
    format_user_message(&engine.model, user_message)
}

/// Vision / short-caption path: Qwen3 templates default to thinking; disable when possible.
pub fn format_user_message_engine_vision(
    engine: &GgufEngine,
    user_message: &str,
) -> Result<String> {
    let hay = engine
        .path
        .to_string_lossy()
        .to_ascii_lowercase();
    let mut msg = user_message.to_string();
    if hay.contains("qwen3") && !msg.contains("/no_think") {
        msg.push_str(" /no_think");
    }
    format_user_message_engine(engine, &msg)
}
