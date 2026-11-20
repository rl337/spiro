//! US-010: Qwen3 ~2B GGUF chat completion end-to-end.

use std::env;
use std::path::PathBuf;

use uie_catalog::{expand_home, ModelCatalog};
use uie_loader_gguf::{complete_gguf_file_chat, CompleteOptions, CompleteResult};

use crate::gguf_resolve::{find_qwen3_2b_path, flag_value, roots_file};
use crate::harness::{E2eContext, E2eError, E2eTest};

#[derive(Debug, Default, Clone, Copy)]
pub struct Qwen3_2bChatCompletion;

impl E2eTest for Qwen3_2bChatCompletion {
    fn name(&self) -> &'static str {
        "qwen3_2b_chat_completion"
    }

    fn description(&self) -> &'static str {
        "Load a Qwen3 ~2B GGUF from model roots (or env path), apply chat template, \
         generate text, assert tokens and telemetry (US-010)."
    }

    fn run(&self, ctx: &mut E2eContext, args: &[String]) -> Result<(), E2eError> {
        ctx.phase("resolve_model_roots");
        let roots = roots_file(&ctx.repo_root).map_err(|e| E2eError::Failed(e.to_string()))?;
        let catalog = ModelCatalog::from_roots(roots);

        ctx.phase("list_qwen_gguf");
        let qwen_count = catalog
            .registry()
            .entries()
            .iter()
            .filter(|e| {
                e.path.extension().is_some_and(|x| x == "gguf")
                    && format!("{} {}", e.id, e.path.display())
                        .to_lowercase()
                        .contains("qwen")
            })
            .count();
        ctx.record(format!("qwen_gguf_candidates:{qwen_count}"));
        ctx.assert(
            qwen_count > 0,
            "no Qwen .gguf under SPIRO_MODEL_ROOTS — download Qwen3 2B in LM Studio",
        )?;

        ctx.phase("resolve_model_path");
        let model_path = resolve_model_path(args, &catalog)?;
        ctx.record(format!("model:{}", model_path.display()));
        ctx.assert(
            model_path.is_file(),
            format!("model not found: {}", model_path.display()),
        )?;

        let prompt = flag_value(args, "--prompt")
            .or_else(|| env::var("UIE_LLM_PROMPT").ok())
            .unwrap_or_else(|| "What is 2+2? Answer in one short sentence.".into());
        let max_new: u32 = flag_value(args, "--max-new")
            .and_then(|s| s.parse().ok())
            .or_else(|| env::var("UIE_LLM_MAX_NEW").ok()?.parse().ok())
            .unwrap_or(64);

        let opts = CompleteOptions {
            max_new_tokens: max_new,
            ..CompleteOptions::default()
        };

        ctx.phase("gguf_chat_complete");
        let out: CompleteResult = complete_gguf_file_chat(&model_path, &prompt, &opts)
            .map_err(|e| E2eError::Failed(e.to_string()))?;

        eprintln!("completion: {}", out.text);
        // assert_story_events: gguf_completion_metrics
        ctx.record("gguf_completion_metrics");
        ctx.assert(!out.token_ids.is_empty(), "expected non-empty token_ids")?;
        ctx.assert(
            out.metrics.total_ms >= 0.0,
            "expected non-negative total_ms",
        )?;
        ctx.assert(
            out.metrics.completion_tokens > 0,
            "expected completion_tokens > 0",
        )?;
        Ok(())
    }
}

fn resolve_model_path(args: &[String], catalog: &ModelCatalog) -> Result<PathBuf, E2eError> {
    if let Some(p) = flag_value(args, "--model") {
        return Ok(expand_home(PathBuf::from(p)));
    }
    if let Some(id) = flag_value(args, "--registry-id") {
        let entry = catalog
            .registry()
            .get(&id)
            .ok_or_else(|| E2eError::Failed(format!("registry id not found: {id}")))?;
        return Ok(entry.path.clone());
    }
    if let Ok(id) = env::var("UIE_QWEN3_2B_REGISTRY_ID") {
        let entry = catalog
            .registry()
            .get(&id)
            .ok_or_else(|| E2eError::Failed(format!("UIE_QWEN3_2B_REGISTRY_ID not found: {id}")))?;
        return Ok(entry.path.clone());
    }
    if let Ok(p) = env::var("UIE_GGUF_MODEL") {
        return Ok(expand_home(PathBuf::from(p)));
    }
    if let Ok(p) = env::var("UIE_LIVE_MODEL_PATH") {
        return Ok(expand_home(PathBuf::from(p)));
    }
    if args.iter().any(|a| a == "--auto-qwen3-2b") {
        return find_qwen3_2b_path(catalog).map_err(|e| E2eError::Failed(e.to_string()));
    }
    find_qwen3_2b_path(catalog).map_err(|e| E2eError::Failed(e.to_string()))
}
