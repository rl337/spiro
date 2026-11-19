use uie_core::error::{Error, Result};
use uie_core::tensor::{DataType, Tensor};

pub fn expect_dtype(tensor: &Tensor, dtype: DataType, name: &str) -> Result<()> {
    if tensor.dtype() != dtype {
        return Err(Error::DtypeMismatch {
            expected: dtype,
            got: tensor.dtype(),
        });
    }
    let _ = name;
    Ok(())
}

pub fn expect_rank(tensor: &Tensor, rank: usize, name: &str) -> Result<()> {
    if tensor.shape().rank() != rank {
        return Err(Error::InvalidShape(format!(
            "{name}: expected rank {rank}, got {:?}",
            tensor.shape().dims()
        )));
    }
    Ok(())
}

pub fn expect_batch_one(tensor: &Tensor, name: &str) -> Result<()> {
    let dims = tensor.shape().dims();
    if dims.first() != Some(&1) {
        return Err(Error::InvalidShape(format!(
            "{name}: batch size must be 1, got {:?}",
            dims
        )));
    }
    Ok(())
}

pub fn expect_token_batch_seq(tensor: &Tensor, name: &str, max_seq: usize) -> Result<()> {
    expect_dtype(tensor, DataType::I32, name)?;
    expect_rank(tensor, 2, name)?;
    expect_batch_one(tensor, name)?;
    let dims = tensor.shape().dims();
    let seq = dims[1];
    if seq == 0 || seq > max_seq {
        return Err(Error::InvalidShape(format!(
            "{name}: seq len {seq} out of range 1..={max_seq}"
        )));
    }
    Ok(())
}

pub fn expect_audio_waveform(tensor: &Tensor, name: &str, max_samples: usize) -> Result<()> {
    expect_dtype(tensor, DataType::F32, name)?;
    expect_rank(tensor, 2, name)?;
    expect_batch_one(tensor, name)?;
    let n = tensor.shape().dims()[1];
    if n == 0 || n > max_samples {
        return Err(Error::InvalidShape(format!(
            "{name}: expected 1..={max_samples} samples at 16 kHz, got {n}"
        )));
    }
    Ok(())
}

pub fn expect_nchw_image(
    tensor: &Tensor,
    name: &str,
    channels: usize,
    height: usize,
    width: usize,
) -> Result<()> {
    expect_dtype(tensor, DataType::F32, name)?;
    expect_rank(tensor, 4, name)?;
    expect_batch_one(tensor, name)?;
    let d = tensor.shape().dims();
    if d[1] != channels || d[2] != height || d[3] != width {
        return Err(Error::InvalidShape(format!(
            "{name}: expected [1, {channels}, {height}, {width}], got {d:?}"
        )));
    }
    Ok(())
}

pub fn expect_clip_embeddings(tensor: &Tensor, name: &str) -> Result<()> {
    expect_dtype(tensor, DataType::F32, name)?;
    expect_rank(tensor, 3, name)?;
    expect_batch_one(tensor, name)?;
    let d = tensor.shape().dims();
    if d[1] != super::conventions::CLIP_SEQ_LEN || d[2] != super::conventions::CLIP_HIDDEN_DIM {
        return Err(Error::InvalidShape(format!(
            "{name}: expected [1, {}, {}], got {:?}",
            super::conventions::CLIP_SEQ_LEN,
            super::conventions::CLIP_HIDDEN_DIM,
            d
        )));
    }
    Ok(())
}
