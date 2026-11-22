//! US-016/017: Image + Qwen2-VL (or compatible) via llama.cpp MTMD (mmproj).

use std::num::NonZeroU32;
use std::path::Path;
use std::time::Instant;

use encoding_rs::UTF_8;
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::context::LlamaContext;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::mtmd::{
    mtmd_default_marker, MtmdBitmap, MtmdContext, MtmdContextParams, MtmdInputText,
};
use llama_cpp_2::sampling::LlamaSampler;
use uie_core::error::{Error, Result};
use uie_telemetry::{ComputeDevice, InferMetrics};

use crate::chat::format_user_message_engine;
use crate::complete::CompleteResult;
use crate::engine::GgufEngine;

#[derive(Debug, Clone)]
pub struct VisionDescribeOptions {
    pub max_new_tokens: u32,
    pub context_tokens: u32,
    pub temperature: f32,
    pub n_batch: i32,
}

impl Default for VisionDescribeOptions {
    fn default() -> Self {
        Self {
            max_new_tokens: 128,
            context_tokens: 4096,
            temperature: 0.2,
            n_batch: 512,
        }
    }
}

/// Load LLM + mmproj, attach a PNG/JPEG path, generate a short description.
pub fn describe_image_mmproj(
    model_path: &Path,
    mmproj_path: &Path,
    image_path: &Path,
    user_prompt: &str,
    opts: &VisionDescribeOptions,
) -> Result<CompleteResult> {
    let image_str = image_path
        .to_str()
        .ok_or_else(|| Error::Runtime("image path is not UTF-8".into()))?;
    let mmproj_str = mmproj_path
        .to_str()
        .ok_or_else(|| Error::Runtime("mmproj path is not UTF-8".into()))?;

    let run_started = Instant::now();
    let engine = GgufEngine::load(model_path)?;

    let mtmd_params = MtmdContextParams::default();
    let mut mtmd = MtmdContext::init_from_file(mmproj_str, &engine.model, &mtmd_params)
        .map_err(|e| Error::ModelLoad(format!("mmproj init: {e}")))?;
    if !mtmd.support_vision() {
        return Err(Error::ModelLoad(
            "mmproj does not report vision support — use Qwen2-VL (or similar) + matching mmproj"
                .into(),
        ));
    }

    let ctx_tokens = NonZeroU32::new(opts.context_tokens.max(512))
        .ok_or_else(|| Error::InvalidShape("context_tokens must be > 0".into()))?;
    let ctx_params = LlamaContextParams::default().with_n_ctx(Some(ctx_tokens));
    let mut llama_ctx = engine
        .model
        .new_context(&engine.backend, ctx_params)
        .map_err(|e| Error::Runtime(format!("llama context: {e}")))?;

    let bitmap = MtmdBitmap::from_file(&mtmd, image_str, false)
        .map_err(|e| Error::Runtime(format!("load image for mtmd: {e}")))?;

    let marker = mtmd_default_marker();
    let user_body = if user_prompt.contains(marker) {
        user_prompt.to_string()
    } else {
        format!("{marker}\n{user_prompt}")
    };
    let text = format_user_message_engine(&engine, &user_body).unwrap_or(user_body);

    let chunks = mtmd
        .tokenize(
            MtmdInputText {
                text,
                add_special: false,
                parse_special: true,
            },
            &[&bitmap],
        )
        .map_err(|e| Error::Runtime(format!("mtmd tokenize: {e}")))?;

    let n_past = chunks
        .eval_chunks(&mut mtmd, &mut llama_ctx, 0, 0, opts.n_batch, true)
        .map_err(|e| Error::Runtime(format!("mtmd eval: {e}")))?;

    let _ = &mtmd;

    run_decode_loop(&engine, &mut llama_ctx, opts, run_started, n_past)
}

fn build_sampler(temperature: f32) -> LlamaSampler {
    if temperature <= 0.0 {
        LlamaSampler::greedy()
    } else {
        LlamaSampler::chain_simple([LlamaSampler::temp(temperature), LlamaSampler::dist(0)])
    }
}

fn run_decode_loop(
    engine: &GgufEngine,
    llama_ctx: &mut LlamaContext,
    opts: &VisionDescribeOptions,
    run_started: Instant,
    mut n_cur: i32,
) -> Result<CompleteResult> {
    let device = if std::env::var("UIE_GGUF_GPU").is_ok_and(|v| v == "1") {
        ComputeDevice::Gpu
    } else {
        ComputeDevice::Cpu
    };

    let max_gen = opts
        .max_new_tokens
        .min(opts.context_tokens.saturating_sub(128));
    let mut sampler = build_sampler(opts.temperature);
    let vocab = engine.model.vocab();

    let decode_t0 = Instant::now();
    let mut new_token_ids: Vec<i32> = Vec::new();
    let mut output_text = String::new();
    let mut ttft_ms = None;
    let mut completion_tokens = 0u32;
    let mut batch = LlamaBatch::new(8, 1);

    for _ in 0..max_gen {
        let token = sampler.sample(llama_ctx, -1);
        sampler.accept(token);

        if vocab.is_eog(token) {
            break;
        }

        let token_bytes = vocab.token_to_piece(token, true, None);
        let mut piece = String::with_capacity(32);
        let mut decoder = UTF_8.new_decoder();
        let _ = decoder.decode_to_string(&token_bytes, &mut piece, false);
        output_text.push_str(&piece);

        new_token_ids.push(token.0);
        completion_tokens += 1;
        if ttft_ms.is_none() {
            ttft_ms = Some(run_started.elapsed().as_secs_f64() * 1000.0);
        }

        batch.clear();
        batch
            .add(token, n_cur, &[0], true)
            .map_err(|e| Error::Runtime(format!("batch add: {e}")))?;
        llama_ctx
            .decode(&mut batch)
            .map_err(|e| Error::Runtime(format!("decode token: {e}")))?;
        n_cur += 1;
    }

    let decode_ms = decode_t0.elapsed().as_secs_f64() * 1000.0;
    let total_ms = run_started.elapsed().as_secs_f64() * 1000.0;
    let prefill_ms = ttft_ms.unwrap_or(total_ms);
    let mut metrics = InferMetrics {
        prompt_tokens: 0,
        completion_tokens,
        ttft_ms,
        prefill_ms,
        decode_ms,
        total_ms,
        cpu_time_ms: prefill_ms + decode_ms,
        gpu_time_ms: if matches!(device, ComputeDevice::Gpu) {
            Some(prefill_ms + decode_ms)
        } else {
            None
        },
        decode_tokens_per_sec: None,
        overall_tokens_per_sec: None,
    };
    metrics.recompute_rates();

    Ok(CompleteResult {
        text: output_text,
        token_ids: new_token_ids,
        metrics,
    })
}
