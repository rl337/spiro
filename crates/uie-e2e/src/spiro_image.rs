//! Save diffusion stub / safetensors contract output as PNG for inspection.

use std::fs;
use std::path::Path;

use image::{ImageBuffer, RgbImage};
use uie_core::error::{Error, Result};
use uie_core::tensor::Tensor;

pub fn save_f32_nchw_rgb(tensor: &Tensor, path: &Path, width: u32, height: u32) -> Result<()> {
    let slice = tensor
        .as_f32_slice()
        .map_err(|e| Error::Runtime(e.to_string()))?;
    let ch = 3usize;
    let expected = (width as usize) * (height as usize) * ch;
    if slice.len() < expected {
        return Err(Error::Runtime(format!(
            "tensor too small for {width}x{height} rgb: {}",
            slice.len()
        )));
    }
    let img: RgbImage = ImageBuffer::from_fn(width, height, |x, y| {
        let base = (y as usize * width as usize + x as usize) * ch;
        let r = (slice[base].clamp(0.0, 1.0) * 255.0) as u8;
        let g = (slice[base + 1].clamp(0.0, 1.0) * 255.0) as u8;
        let b = (slice[base + 2].clamp(0.0, 1.0) * 255.0) as u8;
        image::Rgb([r, g, b])
    });
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).map_err(|e| Error::Runtime(e.to_string()))?;
        }
    }
    img.save(path).map_err(|e| Error::Runtime(e.to_string()))
}
