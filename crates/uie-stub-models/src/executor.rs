use std::sync::Arc;
use std::time::Instant;

use tracing::info;
use uie_core::error::{Error, Result};
use uie_core::model::{InferenceExecutor, IoSpec, ModelMetadata};
use uie_core::tensor::{DataType, Shape, Tensor};
use uie_telemetry::{ComputeDevice, ExecutionRecorder};

use crate::stub_infer_metrics::apply_stub_infer_metrics;
use crate::stub_input_validate::validate_stub_inputs;
use crate::telemetry::{phase_modality, PHASE_POLICY_INFER, PHASE_VALIDATE_INPUTS};

use crate::conventions::{
    CLIP_HIDDEN_DIM, CLIP_IMAGE_SIZE, CLIP_SEQ_LEN, DIFFUSION_HEIGHT, DIFFUSION_WIDTH,
    MAX_ASR_TEXT_TOKENS, MAX_CONTEXT_TOKENS, MAX_NEW_TOKENS, SAMPLE_RATE_ASR, TTS_OUTPUT_SAMPLES,
    VLM_IMAGE_SIZE,
};
use crate::modality::ModalityKind;
use crate::policy::{BlankPolicy, StubPolicy};

pub struct StubExecutor {
    kind: ModalityKind,
    metadata: ModelMetadata,
    inputs: Vec<IoSpec>,
    outputs: Vec<IoSpec>,
    policy: Arc<dyn StubPolicy>,
}

impl StubExecutor {
    pub fn new(kind: ModalityKind) -> Self {
        Self::with_policy(kind, Arc::new(BlankPolicy))
    }

    pub fn with_policy(kind: ModalityKind, policy: Arc<dyn StubPolicy>) -> Self {
        let (inputs, outputs) = io_specs(kind);
        Self {
            metadata: ModelMetadata {
                name: format!("stub-{}", kind.id_str()),
                format: "stub".into(),
                ops: 0,
            },
            kind,
            inputs,
            outputs,
            policy,
        }
    }

    pub fn kind(&self) -> ModalityKind {
        self.kind
    }

    pub fn policy(&self) -> Arc<dyn StubPolicy> {
        Arc::clone(&self.policy)
    }
}

fn io_specs(kind: ModalityKind) -> (Vec<IoSpec>, Vec<IoSpec>) {
    let input_ids = IoSpec {
        name: "input_ids".into(),
        shape: Shape::new([1, MAX_CONTEXT_TOKENS]),
        dtype: DataType::I32,
    };
    let output_ids = IoSpec {
        name: "output_ids".into(),
        shape: Shape::new([1, MAX_NEW_TOKENS]),
        dtype: DataType::I32,
    };
    let encoder_hidden = IoSpec {
        name: "encoder_hidden_states".into(),
        shape: Shape::new([1, CLIP_SEQ_LEN, CLIP_HIDDEN_DIM]),
        dtype: DataType::F32,
    };
    let pixel_values_clip = IoSpec {
        name: "pixel_values".into(),
        shape: Shape::new([1, 3, CLIP_IMAGE_SIZE, CLIP_IMAGE_SIZE]),
        dtype: DataType::F32,
    };
    let pixel_values_vlm = IoSpec {
        name: "pixel_values".into(),
        shape: Shape::new([1, 3, VLM_IMAGE_SIZE, VLM_IMAGE_SIZE]),
        dtype: DataType::F32,
    };
    let pixel_values_sd = IoSpec {
        name: "pixel_values".into(),
        shape: Shape::new([1, 3, DIFFUSION_HEIGHT, DIFFUSION_WIDTH]),
        dtype: DataType::F32,
    };
    let logits = IoSpec {
        name: "logits".into(),
        shape: Shape::new([1, 2]),
        dtype: DataType::F32,
    };
    let label_ids = IoSpec {
        name: "label_ids".into(),
        shape: Shape::new([1, 1]),
        dtype: DataType::I32,
    };
    let waveform_in = IoSpec {
        name: "input_features".into(),
        shape: Shape::new([1, SAMPLE_RATE_ASR]),
        dtype: DataType::F32,
    };
    let waveform_out = IoSpec {
        name: "waveform".into(),
        shape: Shape::new([1, TTS_OUTPUT_SAMPLES]),
        dtype: DataType::F32,
    };
    let decoded_ids = IoSpec {
        name: "sequences".into(),
        shape: Shape::new([1, MAX_ASR_TEXT_TOKENS]),
        dtype: DataType::I32,
    };
    let image_out = IoSpec {
        name: "sample".into(),
        shape: Shape::new([1, 3, DIFFUSION_HEIGHT, DIFFUSION_WIDTH]),
        dtype: DataType::F32,
    };

    match kind {
        ModalityKind::LlmTextCompletion => (vec![input_ids], vec![output_ids]),
        ModalityKind::Judge => (vec![input_ids], vec![logits, label_ids]),
        ModalityKind::DiffusionTextToImage => (vec![encoder_hidden], vec![image_out]),
        ModalityKind::SpeechToText => (vec![waveform_in.clone()], vec![decoded_ids]),
        ModalityKind::TextToSpeech => (vec![input_ids], vec![waveform_out]),
        ModalityKind::ImageToText => (vec![pixel_values_clip], vec![output_ids]),
        ModalityKind::MultimodalTextImageToText => {
            (vec![input_ids, pixel_values_vlm], vec![output_ids])
        }
        ModalityKind::AudioToAudio => (vec![waveform_in.clone()], vec![waveform_in]),
        ModalityKind::ImageToImage => (vec![pixel_values_sd.clone()], vec![pixel_values_sd]),
        ModalityKind::ImageFilter => {
            let filter_json = IoSpec {
                name: "filter_json".into(),
                shape: Shape::new([4096]),
                dtype: DataType::U8,
            };
            (
                vec![pixel_values_sd.clone(), filter_json],
                vec![image_out],
            )
        }
    }
}

impl InferenceExecutor for StubExecutor {
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
        telemetry.tag_stub_executor(self.metadata.name.clone(), self.kind.story_id());
        telemetry.record_phase_ms(phase_modality(self.kind), ComputeDevice::Cpu, 0.0);

        if inputs.len() != self.inputs.len() {
            return Err(Error::Runtime(format!(
                "stub `{}` expects {} inputs, got {}",
                self.kind.id_str(),
                self.inputs.len(),
                inputs.len()
            )));
        }

        let t_validate = Instant::now();
        telemetry.phase(PHASE_VALIDATE_INPUTS, ComputeDevice::Cpu, || {
            validate_stub_inputs(self.kind, &self.inputs, inputs)
        })?;
        let prefill_ms = t_validate.elapsed().as_secs_f64() * 1000.0;

        let t_policy = Instant::now();
        let outputs = telemetry.phase(PHASE_POLICY_INFER, ComputeDevice::Cpu, || {
            self.policy.infer(self.kind, inputs)
        })?;
        let decode_ms = t_policy.elapsed().as_secs_f64() * 1000.0;

        apply_stub_infer_metrics(
            self.kind,
            inputs,
            &outputs,
            telemetry.metrics_mut(),
            prefill_ms,
            decode_ms,
        );

        let m = telemetry.metrics_mut();
        info!(
            target: "uie_stub",
            modality = %self.kind.id_str(),
            story = self.kind.story_id(),
            prompt_tokens = m.prompt_tokens,
            completion_tokens = m.completion_tokens,
            ttft_ms = ?m.ttft_ms,
            prefill_ms = m.prefill_ms,
            decode_ms = m.decode_ms,
            cpu_time_ms = m.cpu_time_ms,
            "stub infer complete"
        );

        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::samples::sample_input;
    use crate::telemetry::phase_modality;
    use crate::telemetry_assert::assert_stub_executor_telemetry;
    use uie_telemetry::ExecutionRecorder;

    #[test]
    fn canonical_output_shapes() {
        for kind in ModalityKind::ALL {
            if kind == ModalityKind::AudioToAudio {
                continue;
            }
            let stub = StubExecutor::new(kind);
            let inputs = sample_input(kind).expect("sample");
            let mut telem = ExecutionRecorder::new("unit-test");
            let outs = stub.infer(&inputs, &mut telem).expect("infer");
            assert_eq!(outs.len(), stub.outputs().len(), "{:?}", kind);
            for (o, spec) in outs.iter().zip(stub.outputs()) {
                assert_eq!(o.dtype(), spec.dtype, "{:?}", kind);
                assert_eq!(o.shape().dims(), spec.shape.dims(), "{:?}", kind);
            }
            let trace = telem.finish();
            assert_stub_executor_telemetry(&trace);
            assert_eq!(
                trace.user_story.as_deref(),
                Some(kind.story_id()),
                "{:?}",
                kind
            );
            assert!(
                trace.phases.iter().any(|p| p.name == phase_modality(kind)),
                "{:?}",
                kind
            );
        }
    }

    #[test]
    fn llm_stub_emits_token_metrics() {
        let stub = StubExecutor::new(ModalityKind::LlmTextCompletion);
        let inputs = sample_input(ModalityKind::LlmTextCompletion).unwrap();
        let mut telem = ExecutionRecorder::new("unit-llm-metrics");
        stub.infer(&inputs, &mut telem).unwrap();
        let trace = telem.finish();
        assert!(trace.metrics.prompt_tokens > 0);
        assert!(trace.metrics.completion_tokens > 0);
        assert!(trace.metrics.ttft_ms.is_some());
    }

    #[test]
    fn audio_passthrough_matches_input_length() {
        let stub = StubExecutor::new(ModalityKind::AudioToAudio);
        let inputs = sample_input(ModalityKind::AudioToAudio).unwrap();
        let mut telem = ExecutionRecorder::new("unit-test");
        let out = stub.infer(&inputs, &mut telem).unwrap();
        assert_eq!(out[0].shape().dims(), inputs[0].shape().dims(),);
    }
}
