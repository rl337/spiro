//! Spirograph curve tracer — logic migrated from `spiro.py`.
//!
//! The original script maintains a 2048×2048 RGB buffer and iteratively plots
//! hypocycloid/epicycloid points until the path repeats. Here that buffer is a
//! [`uie_core::Tensor`] (H × W × 3, U8), executed on whatever backend allocates it
//! (typically [`CpuBackend`] via the demo binary).

use std::f64::consts::PI;

use uie_core::tensor::{DataType, Shape, Tensor};

pub const IMG_WIDTH: usize = 2048;
pub const IMG_HEIGHT: usize = 2048;

#[derive(Debug, Clone, Copy)]
pub struct Rgb(pub u8, pub u8, pub u8);

/// Parameters matching the Python `spiro(...)` function.
#[derive(Debug, Clone, Copy)]
pub struct SpiroParams {
    pub center_x: f64,
    pub center_y: f64,
    pub frame_radius: f64,
    pub shape_radius: f64,
    pub hole_height: f64,
    pub color: Rgb,
}

impl SpiroParams {
    pub fn demo_layers(center_x: f64, center_y: f64) -> Vec<Self> {
        vec![
            Self {
                center_x,
                center_y,
                frame_radius: 600.0,
                shape_radius: 360.0,
                hole_height: 31.0,
                color: Rgb(0xFF, 0xFF, 0x00),
            },
            Self {
                center_x,
                center_y,
                frame_radius: 600.0,
                shape_radius: 235.0,
                hole_height: 1.0,
                color: Rgb(0x80, 0xFF, 0x80),
            },
            Self {
                center_x,
                center_y,
                frame_radius: 600.0,
                shape_radius: 105.0,
                hole_height: 17.0,
                color: Rgb(0x00, 0xFF, 0xFF),
            },
            Self {
                center_x,
                center_y,
                frame_radius: 400.0,
                shape_radius: 250.0,
                hole_height: 31.0,
                color: Rgb(0xFF, 0xFF, 0x80),
            },
            Self {
                center_x,
                center_y,
                frame_radius: 400.0,
                shape_radius: 150.0,
                hole_height: 1.0,
                color: Rgb(0xFF, 0x8F, 0x80),
            },
            Self {
                center_x,
                center_y,
                frame_radius: 400.0,
                shape_radius: 35.0,
                hole_height: 15.0,
                color: Rgb(0xFF, 0xFF, 0xFF),
            },
        ]
    }
}

pub fn canvas_shape() -> Shape {
    Shape::new([IMG_HEIGHT, IMG_WIDTH, 3])
}

pub fn new_canvas_tensor() -> uie_core::Result<Tensor> {
    Tensor::empty(DataType::U8, canvas_shape())
}

/// Plot one spiro layer into `buffer` (row-major RGB bytes).
pub fn spiro_trace(buffer: &mut [u8], params: SpiroParams) {
    let SpiroParams {
        center_x: x,
        center_y: y,
        frame_radius,
        shape_radius,
        hole_height,
        color,
    } = params;

    let mut prefix = Vec::new();
    let mut last_n = Vec::new();
    let stride = IMG_WIDTH * 3;

    let mut i: i64 = -1;
    loop {
        i += 1;
        let iu = i as f64;
        let outer_angle = iu * PI / 180.0 / 10.0;
        let inner_angle = frame_radius / shape_radius * iu * PI / 180.0 / 10.0;

        let hole_range_min = hole_height / 100.0 * shape_radius;
        let hole_range_max = shape_radius - hole_range_min;
        let hole_range = hole_range_max - hole_range_min;

        let outer_x = frame_radius * outer_angle.cos();
        let outer_y = frame_radius * outer_angle.sin();
        let inner_x = hole_range * inner_angle.cos();
        let inner_y = hole_range * inner_angle.sin();

        let nx = (x + outer_x + inner_x) as i32;
        let ny = (y + outer_y + inner_y) as i32;
        let coord = (nx, ny);

        last_n.push(coord);
        if i < 20 {
            prefix.push(coord);
        } else {
            last_n.remove(0);
        }

        if i > 20 {
            let tail = &last_n[last_n.len() - prefix.len()..];
            if prefix == tail {
                break;
            }
        }

        if ny >= 0 && ny < IMG_HEIGHT as i32 && nx >= 0 && nx < IMG_WIDTH as i32 {
            let base = ny as usize * stride + nx as usize * 3;
            buffer[base] = color.0;
            buffer[base + 1] = color.1;
            buffer[base + 2] = color.2;
        }
    }
}

pub fn render_demo_layers(tensor: &mut Tensor) -> uie_core::Result<()> {
    let mut data = tensor.as_bytes().to_vec();
    let cx = IMG_WIDTH as f64 / 2.0;
    let cy = IMG_HEIGHT as f64 / 2.0;
    for layer in SpiroParams::demo_layers(cx, cy) {
        spiro_trace(&mut data, layer);
    }
    *tensor = Tensor::from_bytes(DataType::U8, canvas_shape(), data)?;
    Ok(())
}
