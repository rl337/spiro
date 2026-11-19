use std::path::{Path, PathBuf};
use std::sync::Mutex;

use uie_core::error::{Error, Result};
use uie_core::model::{InferenceExecutor, IoSpec, ModelMetadata};
use uie_core::tensor::{DataType, Shape, Tensor};
use uie_telemetry::{ComputeDevice, ExecutionRecorder};
const MAX_CONTEXT_TOKENS: usize = 2048;
const MAX_NEW_TOKENS: usize = 128;
const PAD_TOKEN_ID: i32 = 0;

use crate::complete::{complete_from_prompt, CompleteOptions};

pub struct GgufLlmExecutor {
    path: PathBuf,
    max_new_tokens: usize,
    metadata: ModelMetadata,
    inputs: Vec<IoSpec>,
    outputs: Vec<IoSpec>,
    /// Serialized load: gguf + llama are not Sync-friendly; one completion at a time.
    gate: Mutex<()>,
}

impl GgufLlmExecutor {
    pub fn from_path(path: &Path, max_new_tokens: usize) -> Result<Self> {
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("gguf")
            .to_string();
        Ok(Self {
            path: path.to_path_buf(),
            max_new_tokens,
            metadata: ModelMetadata {
                name,
                format: "gguf".into(),
                ops: 0,
            },
            inputs: vec![IoSpec {
                name: "prompt".into(),
                shape: Shape::new([1, MAX_CONTEXT_TOKENS]),
                dtype: DataType::I32,
            }],
            outputs: vec![
                IoSpec {
                    name: "completion_text".into(),
                    shape: Shape::new([MAX_NEW_TOKENS]),
                    dtype: DataType::U8,
                },
                IoSpec {
                    name: "output_ids".into(),
                    shape: Shape::new([1, MAX_NEW_TOKENS]),
                    dtype: DataType::I32,
                },
            ],
            gate: Mutex::new(()),
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl InferenceExecutor for GgufLlmExecutor {
    fn metadata(&self) -> &ModelMetadata {
        &self.metadata
    }

    fn inputs(&self) -> &[IoSpec] {
        &self.inputs
    }

    fn outputs(&self) -> &[IoSpec] {
        &self.outputs
    }

    fn infer(&self, inputs: &[Tensor], telemetry: &mut ExecutionRecorder) -> Result<Vec<Tensor>> {
        let _guard = self
            .gate
            .lock()
            .map_err(|_| Error::Runtime("gguf infer lock".into()))?;

        if inputs.is_empty() {
            return Err(Error::Runtime("expected prompt tensor".into()));
        }

        let device = if std::env::var("UIE_GGUF_GPU").is_ok_and(|v| v == "1") {
            ComputeDevice::Gpu
        } else {
            ComputeDevice::Cpu
        };

        telemetry.phase("gguf_complete", device, || {
            let prompt = prompt_from_tensor(&inputs[0])?;
            let opts = CompleteOptions {
                max_new_tokens: self.max_new_tokens as u32,
                ..CompleteOptions::default()
            };
            let result = complete_from_prompt(&self.path, &prompt, &opts)?;
            merge_gguf_metrics(telemetry, &result);

            let text_bytes = result.text.as_bytes().to_vec();
            let mut text_buf = vec![0u8; MAX_NEW_TOKENS];
            let copy_len = text_bytes.len().min(MAX_NEW_TOKENS);
            text_buf[..copy_len].copy_from_slice(&text_bytes[..copy_len]);

            let mut out_ids = vec![PAD_TOKEN_ID; MAX_NEW_TOKENS];
            let gen = result.token_ids;
            let copy_ids = gen.len().min(MAX_NEW_TOKENS);
            for (i, t) in gen.iter().take(copy_ids).enumerate() {
                out_ids[i] = *t;
            }

            Ok(vec![
                Tensor::from_bytes(DataType::U8, Shape::new([MAX_NEW_TOKENS]), text_buf)?,
                Tensor::from_i32_slice(Shape::new([1, MAX_NEW_TOKENS]), &out_ids)?,
            ])
        })
    }
}

fn merge_gguf_metrics(telemetry: &mut ExecutionRecorder, result: &crate::complete::CompleteResult) {
    let m = telemetry.metrics_mut();
    *m = result.metrics.clone();
}

fn prompt_from_tensor(t: &Tensor) -> Result<String> {
    match t.dtype() {
        DataType::U8 => {
            let bytes = t.as_bytes();
            let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
            String::from_utf8(bytes[..end].to_vec())
                .map_err(|e| Error::Runtime(format!("prompt utf8: {e}")))
        }
        _ => Err(Error::Runtime(
            "GGUF text completion expects UTF-8 prompt in a U8 tensor".into(),
        )),
    }
}

/// Build a fixed-size U8 prompt tensor (NUL-padded).
pub fn prompt_tensor(text: &str, max_len: usize) -> Result<Tensor> {
    let mut buf = vec![0u8; max_len];
    let bytes = text.as_bytes();
    let n = bytes.len().min(max_len);
    buf[..n].copy_from_slice(&bytes[..n]);
    Tensor::from_bytes(DataType::U8, Shape::new([max_len]), buf)
}
