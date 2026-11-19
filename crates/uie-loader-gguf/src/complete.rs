//! Text completion via llama.cpp (prompt in, token ids + text out).

use std::num::NonZeroU32;
use std::path::Path;

use encoding_rs::UTF_8;
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::sampling::LlamaSampler;
use llama_cpp_2::token::LlamaToken;
use uie_core::error::{Error, Result};
use uie_telemetry::{ComputeDevice, InferMetrics};
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct CompleteOptions {
    pub max_new_tokens: u32,
    pub context_tokens: u32,
    pub temperature: f32,
}

impl Default for CompleteOptions {
    fn default() -> Self {
        Self {
            max_new_tokens: 64,
            context_tokens: 2048,
            temperature: 0.7,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompleteResult {
    pub text: String,
    pub token_ids: Vec<i32>,
    pub metrics: InferMetrics,
}

pub fn complete_from_prompt(
    model_path: &Path,
    prompt: &str,
    opts: &CompleteOptions,
) -> Result<CompleteResult> {
    let backend = LlamaBackend::init().map_err(|e| Error::ModelLoad(format!("llama backend: {e}")))?;
    let model_params = LlamaModelParams::default();
    let model = LlamaModel::load_from_file(&backend, model_path, &model_params)
        .map_err(|e| Error::ModelLoad(format!("load gguf {}: {e}", model_path.display())))?;

    let vocab = model.vocab();
    let tokens = vocab.tokenize(prompt.as_bytes(), true, true);
    run_generation(&backend, &model, tokens, opts)
}

fn run_generation(
    backend: &LlamaBackend,
    model: &LlamaModel,
    tokens: Vec<LlamaToken>,
    opts: &CompleteOptions,
) -> Result<CompleteResult> {
    let run_started = Instant::now();
    let device = if std::env::var("UIE_GGUF_GPU").is_ok_and(|v| v == "1") {
        ComputeDevice::Gpu
    } else {
        ComputeDevice::Cpu
    };

    let ctx_tokens = NonZeroU32::new(opts.context_tokens.max(256)).ok_or_else(|| {
        Error::InvalidShape("context_tokens must be > 0".into())
    })?;
    let ctx_params = LlamaContextParams::default().with_n_ctx(Some(ctx_tokens));
    let mut ctx = model
        .new_context(backend, ctx_params)
        .map_err(|e| Error::Runtime(format!("llama context: {e}")))?;

    let max_gen = opts.max_new_tokens.min(opts.context_tokens.saturating_sub(64));
    if tokens.is_empty() {
        return Err(Error::Runtime("empty prompt tokens".into()));
    }
    let prompt_len = tokens.len() as i32;

    let batch_cap = (prompt_len as usize).max(64);
    let mut batch = LlamaBatch::new(batch_cap, 1);
    let last_index = prompt_len - 1;
    for (i, token) in (0_i32..).zip(tokens.iter().copied()) {
        let is_last = i == last_index;
        batch
            .add(token, i, &[0], is_last)
            .map_err(|e| Error::Runtime(format!("batch add: {e}")))?;
    }
    let prefill_t0 = Instant::now();
    ctx.decode(&mut batch)
        .map_err(|e| Error::Runtime(format!("decode prompt: {e}")))?;
    let prefill_ms = prefill_t0.elapsed().as_secs_f64() * 1000.0;

    let mut sampler = LlamaSampler::greedy();
    let vocab = model.vocab();

    let prompt_tokens = tokens.len() as u32;
    let mut generated: Vec<i32> = tokens.iter().map(|t| t.0).collect();
    let mut output_text = String::new();
    let mut n_cur = batch.n_tokens();
    let mut ttft_ms = None;
    let decode_t0 = Instant::now();
    let mut completion_tokens = 0u32;

    for _ in 0..max_gen {
        let token = sampler.sample(&ctx, batch.n_tokens() - 1);
        sampler.accept(token);

        if vocab.is_eog(token) {
            break;
        }

        let token_bytes = vocab.token_to_piece(token, true, None);
        let mut piece = String::with_capacity(32);
        let mut decoder = UTF_8.new_decoder();
        let _ = decoder.decode_to_string(&token_bytes, &mut piece, false);
        output_text.push_str(&piece);

        generated.push(token.0);
        completion_tokens += 1;
        if ttft_ms.is_none() {
            ttft_ms = Some(run_started.elapsed().as_secs_f64() * 1000.0);
        }

        batch.clear();
        batch
            .add(token, n_cur, &[0], true)
            .map_err(|e| Error::Runtime(format!("batch add: {e}")))?;
        ctx.decode(&mut batch)
            .map_err(|e| Error::Runtime(format!("decode token: {e}")))?;
        n_cur += 1;
    }

    let decode_ms = decode_t0.elapsed().as_secs_f64() * 1000.0;
    let total_ms = run_started.elapsed().as_secs_f64() * 1000.0;
    let mut metrics = InferMetrics {
        prompt_tokens,
        completion_tokens,
        ttft_ms,
        prefill_ms,
        decode_ms,
        total_ms,
        cpu_time_ms: 0.0,
        gpu_time_ms: None,
        decode_tokens_per_sec: None,
        overall_tokens_per_sec: None,
    };
    metrics.add_phase_time(device, prefill_ms);
    metrics.add_phase_time(device, decode_ms);
    metrics.recompute_rates();

    Ok(CompleteResult {
        text: output_text,
        token_ids: generated,
        metrics,
    })
}