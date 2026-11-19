use uie_core::error::Result;
use uie_core::tensor::Tensor;

use crate::blanks::{
    diffusion_image_blank_nchw, judge_label_blank, judge_logits_blank, like_zeros_f32,
    silent_audio_n_samples, token_ids_blank,
};
use crate::conventions::{
    CLIP_IMAGE_SIZE, DIFFUSION_HEIGHT, DIFFUSION_WIDTH, MAX_ASR_TEXT_TOKENS, MAX_CONTEXT_TOKENS,
    MAX_NEW_TOKENS, SAMPLE_RATE_ASR, TTS_OUTPUT_SAMPLES, VLM_IMAGE_SIZE,
};
use crate::modality::ModalityKind;
use crate::validate::{
    expect_audio_waveform, expect_clip_embeddings, expect_nchw_image, expect_token_batch_seq,
};

/// Default blank outputs after input validation (production stub behavior).
pub fn infer_blank(kind: ModalityKind, inputs: &[Tensor]) -> Result<Vec<Tensor>> {
    match kind {
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