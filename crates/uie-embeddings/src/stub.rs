//! Deterministic CI stub (EMB-09). Not the EMB-06 reference model id.

use uie_core::error::{Error, Result};
use uie_telemetry::{ComputeDevice, ExecutionRecorder};

use crate::config::TextEmbedderConfig;
use crate::telemetry::{PHASE_EMBED_ENCODE, PHASE_EMBED_VALIDATE};
use crate::traits::TextEmbedder;

/// Fixed stub identity — must not collide with live BGE / MiniLM ids.
pub const STUB_MODEL_ID: &str = "uie-stub-text-embed-v1";
pub const STUB_DIMENSIONS: usize = 384;

#[derive(Debug, Clone)]
pub struct StubTextEmbedder {
    config: TextEmbedderConfig,
}

impl StubTextEmbedder {
    pub fn new(config: TextEmbedderConfig) -> Result<Self> {
        if config.expected_dimensions == 0 {
            return Err(Error::ModelLoad(
                "text embedder expected_dimensions must be > 0".into(),
            ));
        }
        if config.expected_dimensions != STUB_DIMENSIONS {
            return Err(Error::ModelLoad(format!(
                "stub embedder is {STUB_DIMENSIONS}-d; config asked for {}",
                config.expected_dimensions
            )));
        }
        Ok(Self { config })
    }

    pub fn with_defaults() -> Self {
        Self::new(TextEmbedderConfig::stub()).expect("default stub config")
    }
}

impl TextEmbedder for StubTextEmbedder {
    fn model_id(&self) -> &str {
        STUB_MODEL_ID
    }

    fn dimensions(&self) -> usize {
        STUB_DIMENSIONS
    }

    fn embed(
        &self,
        texts: &[String],
        telemetry: &mut ExecutionRecorder,
    ) -> Result<Vec<Vec<f32>>> {
        telemetry.tag_stub_executor(STUB_MODEL_ID, "US-033");

        if texts.is_empty() {
            telemetry.record_phase_ms(PHASE_EMBED_VALIDATE, ComputeDevice::Cpu, 0.0);
            telemetry.record_phase_ms(PHASE_EMBED_ENCODE, ComputeDevice::Cpu, 0.0);
            return Ok(Vec::new());
        }

        if texts.len() > 64 {
            return Err(Error::Runtime(format!(
                "embed batch length {} exceeds max 64",
                texts.len()
            )));
        }

        telemetry.phase(PHASE_EMBED_VALIDATE, ComputeDevice::Cpu, || {
            // Strings are UTF-8 by construction; reject only pathological oversized entries.
            for (i, t) in texts.iter().enumerate() {
                if t.len() > 1_000_000 {
                    return Err(Error::Runtime(format!(
                        "embed input {i} exceeds 1MB byte budget"
                    )));
                }
            }
            Ok(())
        })?;

        let max_chars = self.config.max_chars.max(1);
        let dim = STUB_DIMENSIONS;
        let out = telemetry.phase(PHASE_EMBED_ENCODE, ComputeDevice::Cpu, || {
            let mut vectors = Vec::with_capacity(texts.len());
            for text in texts {
                let truncated = truncate_chars(text, max_chars);
                vectors.push(hash_embed(truncated, dim));
            }
            if vectors.len() != texts.len() {
                return Err(Error::Runtime(
                    "embed batch length differs from input length".into(),
                ));
            }
            let expect = vectors[0].len();
            if expect == 0 || expect != dim {
                return Err(Error::Runtime("embed produced empty or wrong-dim vector".into()));
            }
            for v in &vectors {
                if v.len() != expect {
                    return Err(Error::Runtime(
                        "embed vectors have inconsistent dimensions".into(),
                    ));
                }
            }
            Ok(vectors)
        })?;

        let m = telemetry.metrics_mut();
        m.prompt_tokens = texts.len() as u32;
        m.completion_tokens = texts.len() as u32;

        Ok(out)
    }
}

fn truncate_chars(text: &str, max_chars: usize) -> &str {
    if text.chars().count() <= max_chars {
        return text;
    }
    let mut end = 0;
    for (i, (byte_idx, _)) in text.char_indices().enumerate() {
        if i == max_chars {
            end = byte_idx;
            break;
        }
    }
    if end == 0 {
        text
    } else {
        &text[..end]
    }
}

/// Deterministic bag-of-tokens hash → L2 unit vector (CI only).
fn hash_embed(text: &str, dim: usize) -> Vec<f32> {
    let mut values = vec![0f32; dim];
    let tokens: Vec<String> = text
        .split(|ch: char| !ch.is_alphanumeric())
        .filter(|token| !token.is_empty())
        .map(|token| token.to_lowercase())
        .collect();
    if tokens.is_empty() {
        values[0] = 1.0;
        return values;
    }
    for token in &tokens {
        let hash = fnv(token);
        let index = (hash as usize) % dim;
        let sign = if (hash >> 31) & 1 == 1 { 1.0 } else { -1.0 };
        values[index] += sign;
    }
    for window in tokens.windows(2) {
        let hash = fnv(&format!("{} {}", window[0], window[1]));
        let index = (hash as usize) % dim;
        let sign = if (hash >> 17) & 1 == 1 { 1.0 } else { -1.0 };
        values[index] += sign * 0.5;
    }
    l2_normalize(&mut values);
    values
}

fn l2_normalize(values: &mut [f32]) {
    let norm = values.iter().map(|v| v * v).sum::<f32>().sqrt();
    if norm > 0.0 {
        for v in values {
            *v /= norm;
        }
    }
}

fn fnv(text: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in text.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_batch() {
        let stub = StubTextEmbedder::with_defaults();
        let mut telem = ExecutionRecorder::new("t");
        let out = stub.embed(&[], &mut telem).unwrap();
        assert!(out.is_empty());
    }

    #[test]
    fn l2_unit_and_stable() {
        let stub = StubTextEmbedder::with_defaults();
        let mut a = ExecutionRecorder::new("a");
        let mut b = ExecutionRecorder::new("b");
        let texts = vec!["hello weirdness hub".into()];
        let v1 = stub.embed(&texts, &mut a).unwrap();
        let v2 = stub.embed(&texts, &mut b).unwrap();
        assert_eq!(v1, v2);
        let n: f32 = v1[0].iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((n - 1.0).abs() < 1e-5);
        assert_eq!(v1[0].len(), STUB_DIMENSIONS);
    }

    #[test]
    fn batch_too_large_errors() {
        let stub = StubTextEmbedder::with_defaults();
        let texts: Vec<String> = (0..65).map(|i| format!("t{i}")).collect();
        let mut telem = ExecutionRecorder::new("t");
        assert!(stub.embed(&texts, &mut telem).is_err());
    }
}
