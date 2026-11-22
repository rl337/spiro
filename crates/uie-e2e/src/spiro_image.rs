//! Save diffusion stub / safetensors contract output as PNG for inspection.

use std::fs;
use std::path::Path;

use image::imageops::FilterType;
use image::{ImageBuffer, RgbImage};
use uie_core::error::{Error, Result};
use uie_core::tensor::{Shape, Tensor};

/// Load or resize a PNG/JPEG to `[1, 3, H, W]` f32 in 0..1 (NCHW).
pub fn load_png_nchw_rgb(path: &Path, width: u32, height: u32) -> Result<Tensor> {
    let img = image::open(path).map_err(|e| Error::Runtime(format!("open image: {e}")))?;
    let rgb = img.to_rgb8();
    let resized = if rgb.width() != width || rgb.height() != height {
        image::imageops::resize(&rgb, width, height, FilterType::Lanczos3)
    } else {
        rgb
    };
    let mut data = Vec::with_capacity((width * height * 3) as usize);
    for y in 0..height {
        for x in 0..width {
            let p = resized.get_pixel(x, y);
            data.push(p[0] as f32 / 255.0);
            data.push(p[1] as f32 / 255.0);
            data.push(p[2] as f32 / 255.0);
        }
    }
    // HWC planar -> NCHW
    let plane = width as usize * height as usize;
    let mut nchw = vec![0.0f32; plane * 3];
    for c in 0..3 {
        for i in 0..plane {
            nchw[c * plane + i] = data[i * 3 + c];
        }
    }
    Tensor::from_f32_slice(Shape::new([1, 3, height as usize, width as usize]), &nchw)
}

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
