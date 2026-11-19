use uie_core::error::Result;
use uie_core::tensor::{Shape, Tensor};

use crate::conventions::{
    CLIP_HIDDEN_DIM, CLIP_IMAGE_SIZE, CLIP_SEQ_LEN, DIFFUSION_HEIGHT, MAX_ASR_SAMPLES,
    MAX_PROMPT_TOKENS, PAD_TOKEN_ID, SAMPLE_RATE_ASR, VLM_IMAGE_SIZE, VOCAB_SIZE,
};
use crate::modality::ModalityKind;

fn fake_token_ids(seq_len: usize) -> Result<Tensor> {
    let ids: Vec<i32> = (0..seq_len)
        .map(|i| ((i as i32 * 13) % VOCAB_SIZE as i32).max(PAD_TOKEN_ID + 1))
        .collect();
    Tensor::from_i32_slice(Shape::new([1, seq_len]), &ids)
}

fn pixel_values_nchw(size: usize) -> Result<Tensor> {
    let n = 3 * size * size;
    let vals: Vec<f32> = (0..n).map(|i| (i % 256) as f32 / 255.0).collect();
    Tensor::from_f32_slice(Shape::new([1, 3, size, size]), &vals)
}

fn clip_encoder_hidden_states() -> Result<Tensor> {
    let n = CLIP_SEQ_LEN * CLIP_HIDDEN_DIM;
    let vals: Vec<f32> = (0..n).map(|i| (i as f32 * 1e-4).sin()).collect();
    Tensor::from_f32_slice(Shape::new([1, CLIP_SEQ_LEN, CLIP_HIDDEN_DIM]), &vals)
}

fn audio_16k_one_second() -> Result<Tensor> {
    let n = SAMPLE_RATE_ASR;
    let wave: Vec<f32> = (0..n).map(|i| (i as f32 * 0.01).sin() * 0.1).collect();
    Tensor::from_f32_slice(Shape::new([1, n]), &wave)
}

fn audio_16k_thirty_sec_cap() -> Result<Tensor> {
    let n = MAX_ASR_SAMPLES.min(SAMPLE_RATE_ASR * 2);
    Tensor::from_f32_slice(Shape::new([1, n]), &vec![0.0; n])
}

/// Sample inputs matching real-model tensor contracts (see `conventions.rs`).
pub fn sample_input(kind: ModalityKind) -> Result<Vec<Tensor>> {
    match kind {
        ModalityKind::LlmTextCompletion => Ok(vec![fake_token_ids(16)?]),
        ModalityKind::Judge => Ok(vec![fake_token_ids(48)?]),
        ModalityKind::DiffusionTextToImage => Ok(vec![clip_encoder_hidden_states()?]),
        ModalityKind::SpeechToText => Ok(vec![audio_16k_thirty_sec_cap()?]),
        ModalityKind::TextToSpeech => Ok(vec![fake_token_ids(24)?]),
        ModalityKind::ImageToText => Ok(vec![pixel_values_nchw(CLIP_IMAGE_SIZE)?]),
        ModalityKind::MultimodalTextImageToText => Ok(vec![
            fake_token_ids(MAX_PROMPT_TOKENS)?,
            pixel_values_nchw(VLM_IMAGE_SIZE)?,
        ]),
        ModalityKind::AudioToAudio => Ok(vec![audio_16k_one_second()?]),
        ModalityKind::ImageToImage => Ok(vec![pixel_values_nchw(DIFFUSION_HEIGHT)?]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uie_core::tensor::DataType;

    #[test]
    fn samples_respect_canonical_dtypes() {
        for kind in ModalityKind::ALL {
            let inputs = sample_input(kind).unwrap();
            assert!(!inputs.is_empty());
            for t in &inputs {
                assert!(matches!(t.dtype(), DataType::F32 | DataType::I32));
            }
        }
    }
}
