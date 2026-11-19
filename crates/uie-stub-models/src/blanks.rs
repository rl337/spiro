use uie_core::error::Result;
use uie_core::tensor::{Shape, Tensor};

use crate::conventions::{
    DIFFUSION_HEIGHT, DIFFUSION_WIDTH, JUDGE_NUM_LABELS, PAD_TOKEN_ID,
};

pub fn token_ids_blank(seq_len: usize) -> Result<Tensor> {
    let ids = vec![PAD_TOKEN_ID; seq_len];
    Tensor::from_i32_slice(Shape::new([1, seq_len]), &ids)
}

pub fn judge_logits_blank() -> Result<Tensor> {
    Tensor::from_f32_slice(Shape::new([1, JUDGE_NUM_LABELS]), &[0.0, 0.0])
}

pub fn judge_label_blank() -> Result<Tensor> {
    Tensor::from_i32_slice(Shape::new([1, 1]), &[PAD_TOKEN_ID])
}

pub fn diffusion_image_blank_nchw() -> Result<Tensor> {
    let n = 3 * DIFFUSION_HEIGHT * DIFFUSION_WIDTH;
    Tensor::from_f32_slice(
        Shape::new([1, 3, DIFFUSION_HEIGHT, DIFFUSION_WIDTH]),
        &vec![0.0; n],
    )
}

pub fn silent_audio_n_samples(samples: usize) -> Result<Tensor> {
    Tensor::from_f32_slice(Shape::new([1, samples]), &vec![0.0; samples])
}

pub fn like_zeros_f32(input: &Tensor) -> Result<Tensor> {
    Tensor::like_zeros(input)
}
