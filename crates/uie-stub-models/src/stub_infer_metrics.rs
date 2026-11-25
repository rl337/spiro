//! Fill [`InferMetrics`] for stub runs so CI and JSONL traces match real-loader shape.

use uie_core::tensor::{DataType, Tensor};
use uie_telemetry::InferMetrics;

use crate::conventions::{MAX_CONTEXT_TOKENS, MAX_NEW_TOKENS};
use crate::modality::ModalityKind;

pub fn apply_stub_infer_metrics(
    kind: ModalityKind,
    inputs: &[Tensor],
    outputs: &[Tensor],
    metrics: &mut InferMetrics,
    prefill_ms: f64,
    decode_ms: f64,
) {
    metrics.prefill_ms = prefill_ms;
    metrics.decode_ms = decode_ms;

    match kind {
        ModalityKind::LlmTextCompletion
        | ModalityKind::ImageToText
        | ModalityKind::MultimodalTextImageToText
        | ModalityKind::TextToSpeech => {
            if let Some(t) = inputs.first() {
                metrics.prompt_tokens = token_seq_len(t);
            }
            if let Some(t) = outputs.first().filter(|t| t.dtype() == DataType::I32) {
                metrics.completion_tokens = token_seq_len(t);
            }
            metrics.ttft_ms = Some(decode_ms.max(0.0));
        }
        ModalityKind::Judge => {
            if let Some(t) = inputs.first() {
                metrics.prompt_tokens = token_seq_len(t);
            }
            metrics.completion_tokens = 1;
            metrics.ttft_ms = Some(decode_ms.max(0.0));
        }
        ModalityKind::SpeechToText => {
            metrics.prompt_tokens = audio_samples(inputs.first());
            if let Some(t) = outputs.first().filter(|t| t.dtype() == DataType::I32) {
                metrics.completion_tokens = token_seq_len(t);
            }
        }
        ModalityKind::DiffusionTextToImage
        | ModalityKind::ImageToImage
        | ModalityKind::ImageFilter => {
            metrics.prompt_tokens = 1;
            metrics.completion_tokens = 1;
        }
        ModalityKind::AudioToAudio => {
            metrics.prompt_tokens = audio_samples(inputs.first());
            metrics.completion_tokens = metrics.prompt_tokens;
        }
    }

    metrics.recompute_rates();
    let _ = MAX_CONTEXT_TOKENS;
    let _ = MAX_NEW_TOKENS;
}

fn token_seq_len(t: &Tensor) -> u32 {
    let dims = t.shape().dims();
    if dims.len() >= 2 {
        return dims[1] as u32;
    }
    dims.iter().product::<usize>() as u32
}

fn audio_samples(t: Option<&Tensor>) -> u32 {
    t.and_then(|t| t.shape().dims().get(1).copied())
        .unwrap_or(0) as u32
}
