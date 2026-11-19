mod spiro;

use std::path::PathBuf;

use image::{ImageBuffer, RgbImage};
use tracing::info;
use uie_core::backend::cpu::CpuBackend;
use uie_core::InferenceRuntime;

use crate::spiro::{canvas_shape, new_canvas_tensor, render_demo_layers, IMG_HEIGHT, IMG_WIDTH};

fn main() -> uie_core::Result<()> {
    tracing_subscriber::fmt::init();

    let backend = Box::new(CpuBackend::new());
    let runtime = InferenceRuntime::new(backend);
    let mut session = runtime.create_session();

    let mut canvas = new_canvas_tensor()?;
    session.bind("canvas", canvas.clone());

    info!(
        backend = %runtime.backend().id(),
        shape = ?canvas_shape().dims(),
        "allocating canvas tensor"
    );

    render_demo_layers(&mut canvas)?;
    session.bind("canvas", canvas.clone());

    let out = PathBuf::from("output.jpg");
    save_rgb_jpeg(&canvas, &out)?;
    info!(path = %out.display(), "wrote spiro output");

    Ok(())
}

fn save_rgb_jpeg(tensor: &uie_core::Tensor, path: &std::path::Path) -> uie_core::Result<()> {
    let bytes = tensor.as_bytes();
    let img: RgbImage = ImageBuffer::from_fn(IMG_WIDTH as u32, IMG_HEIGHT as u32, |x, y| {
        let base = (y as usize * IMG_WIDTH + x as usize) * 3;
        image::Rgb([bytes[base], bytes[base + 1], bytes[base + 2]])
    });
    img.save(path)
        .map_err(|e| uie_core::Error::Runtime(e.to_string()))?;
    Ok(())
}
