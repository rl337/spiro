use uie_core::error::Result;
use uie_core::model::IoSpec;
use uie_core::tensor::Tensor;

use crate::conventions::{
    CLIP_IMAGE_SIZE, DIFFUSION_HEIGHT, DIFFUSION_WIDTH, MAX_ASR_SAMPLES, MAX_CONTEXT_TOKENS,
    SAMPLE_RATE_ASR, VLM_IMAGE_SIZE,
};
use crate::modality::ModalityKind;
use crate::validate::{
    expect_audio_waveform, expect_clip_embeddings, expect_nchw_image, expect_token_batch_seq,
};

/// Modality-aware validation run inside `stub_validate_inputs` (recorded as a trace phase).
pub fn validate_stub_inputs(kind: ModalityKind, specs: &[IoSpec], inputs: &[Tensor]) -> Result<()> {
    for (tensor, spec) in inputs.iter().zip(specs) {
        if tensor.dtype() != spec.dtype {
            return Err(uie_core::error::Error::DtypeMismatch {
                expected: spec.dtype,
                got: tensor.dtype(),
            });
        }
    }

    match kind {
        ModalityKind::LlmTextCompletion | ModalityKind::TextToSpeech => {
            expect_token_batch_seq(&inputs[0], "input_ids", MAX_CONTEXT_TOKENS)?;
        }
        ModalityKind::Judge => {
            expect_token_batch_seq(&inputs[0], "input_ids", MAX_CONTEXT_TOKENS)?;
        }
        ModalityKind::DiffusionTextToImage => {
            expect_clip_embeddings(&inputs[0], "encoder_hidden_states")?;
        }
        ModalityKind::SpeechToText => {
            expect_audio_waveform(&inputs[0], "input_features", MAX_ASR_SAMPLES)?;
        }
        ModalityKind::ImageToText => {
            expect_nchw_image(
                &inputs[0],
                "pixel_values",
                3,
                CLIP_IMAGE_SIZE,
                CLIP_IMAGE_SIZE,
            )?;
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
        }
        ModalityKind::AudioToAudio => {
            expect_audio_waveform(&inputs[0], "input_features", SAMPLE_RATE_ASR)?;
        }
        ModalityKind::ImageToImage => {
            expect_nchw_image(
                &inputs[0],
                "pixel_values",
                3,
                DIFFUSION_HEIGHT,
                DIFFUSION_WIDTH,
            )?;
        }
    }

    Ok(())
}