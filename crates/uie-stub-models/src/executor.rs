use uie_core::error::{Error, Result};
use uie_core::model::{InferenceExecutor, IoSpec, ModelMetadata};
use uie_core::tensor::{DataType, Shape, Tensor};

use crate::blanks::{
    diffusion_image_blank_nchw, judge_label_blank, judge_logits_blank, like_zeros_f32,
    silent_audio_n_samples, token_ids_blank,
};
use crate::conventions::{
    CLIP_HIDDEN_DIM, CLIP_IMAGE_SIZE, CLIP_SEQ_LEN, DIFFUSION_HEIGHT, DIFFUSION_WIDTH,
    MAX_ASR_TEXT_TOKENS, MAX_CONTEXT_TOKENS, MAX_NEW_TOKENS, SAMPLE_RATE_ASR, TTS_OUTPUT_SAMPLES,
    VLM_IMAGE_SIZE,
};
use crate::modality::ModalityKind;
use crate::validate::{
    expect_audio_waveform, expect_clip_embeddings, expect_nchw_image, expect_token_batch_seq,
};

pub struct StubExecutor {
    kind: ModalityKind,
    metadata: ModelMetadata,
    inputs: Vec<IoSpec>,
    outputs: Vec<IoSpec>,
}

impl StubExecutor {
    pub fn new(kind: ModalityKind) -> Self {
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
        }
    }

    pub fn kind(&self) -> ModalityKind {
        self.kind
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
        ModalityKind::AudioToAudio => (
            vec![waveform_in.clone()],
            vec![waveform_in],
        ),
        ModalityKind::ImageToImage => (vec![pixel_values_sd.clone()], vec![pixel_values_sd]),
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

    fn infer(&self, inputs: &[Tensor]) -> Result<Vec<Tensor>> {
        if inputs.len() != self.inputs.len() {
            return Err(Error::Runtime(format!(
                "stub `{}` expects {} inputs, got {}",
                self.kind.id_str(),
                self.inputs.len(),
                inputs.len()
            )));
        }

        match self.kind {
            ModalityKind::LlmTextCompletion => {
                expect_token_batch_seq(&inputs[0], "input_ids", MAX_CONTEXT_TOKENS)?;
                Ok(vec![token_ids_blank(MAX_NEW_TOKENS)?])
            }
            ModalityKind::Judge => {
                expect_token_batch_seq(&inputs[0], "input_ids", MAX_CONTEXT_TOKENS)?;
                Ok(vec![judge_logits_blank()?, judge_label_blank()?])
            }
            ModalityKind::DiffusionTextToImage => {
                expect_clip_embeddings(&inputs[0], "encoder_hidden_states")?;
                Ok(vec![diffusion_image_blank_nchw()?])
            }
            ModalityKind::SpeechToText => {
                expect_audio_waveform(&inputs[0], "input_features", SAMPLE_RATE_ASR * 30)?;
                Ok(vec![token_ids_blank(MAX_ASR_TEXT_TOKENS)?])
            }
            ModalityKind::TextToSpeech => {
                expect_token_batch_seq(&inputs[0], "input_ids", MAX_CONTEXT_TOKENS)?;
                Ok(vec![silent_audio_n_samples(TTS_OUTPUT_SAMPLES)?])
            }
            ModalityKind::ImageToText => {
                expect_nchw_image(
                    &inputs[0],
                    "pixel_values",
                    3,
                    CLIP_IMAGE_SIZE,
                    CLIP_IMAGE_SIZE,
                )?;
                Ok(vec![token_ids_blank(MAX_NEW_TOKENS)?])
            }
            ModalityKind::MultimodalTextImageToText => {
                expect_token_batch_seq(&inputs[0], "input_ids", MAX_CONTEXT_TOKENS)?;
                expect_nchw_image(
                    &inputs[1],
                    "pixel_values",
                    3,
                    VLM_IMAGE_SIZE,
                    VLM_IMAGE_SIZE,
                )?;
                Ok(vec![token_ids_blank(MAX_NEW_TOKENS)?])
            }
            ModalityKind::AudioToAudio => {
                expect_audio_waveform(&inputs[0], "input_features", SAMPLE_RATE_ASR * 30)?;
                Ok(vec![like_zeros_f32(&inputs[0])?])
            }
            ModalityKind::ImageToImage => {
                expect_nchw_image(
                    &inputs[0],
                    "pixel_values",
                    3,
                    DIFFUSION_HEIGHT,
                    DIFFUSION_WIDTH,
                )?;
                Ok(vec![diffusion_image_blank_nchw()?])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::samples::sample_input;

    #[test]
    fn canonical_output_shapes() {
        for kind in ModalityKind::ALL {
            if kind == ModalityKind::AudioToAudio {
                continue;
            }
            let stub = StubExecutor::new(kind);
            let inputs = sample_input(kind).expect("sample");
            let outs = stub.infer(&inputs).expect("infer");
            assert_eq!(outs.len(), stub.outputs().len(), "{:?}", kind);
            for (o, spec) in outs.iter().zip(stub.outputs()) {
                assert_eq!(o.dtype(), spec.dtype, "{:?}", kind);
                assert_eq!(o.shape().dims(), spec.shape.dims(), "{:?}", kind);
            }
        }
    }

    #[test]
    fn audio_passthrough_matches_input_length() {
        let stub = StubExecutor::new(ModalityKind::AudioToAudio);
        let inputs = sample_input(ModalityKind::AudioToAudio).unwrap();
        let out = stub.infer(&inputs).unwrap();
        assert_eq!(out[0].shape().dims(), inputs[0].shape().dims(),);
    }
}
