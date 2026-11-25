//! Structured JSON image filters for US-032 (CPU stub; Pi GPU later).

use serde::Deserialize;
use uie_core::error::{Error, Result};
use uie_core::tensor::{Shape, Tensor};


/// Request body carried in the `filter_json` U8 tensor.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct FilterRequest {
    #[serde(default)]
    pub filters: Vec<FilterOp>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum FilterOp {
    Invert,
    Grayscale,
    Brightness {
        #[serde(default = "default_brightness")]
        amount: f32,
    },
    Contrast {
        #[serde(default = "default_contrast")]
        amount: f32,
    },
}

fn default_brightness() -> f32 {
    0.1
}

fn default_contrast() -> f32 {
    1.2
}

pub fn parse_filter_json(bytes: &[u8]) -> Result<FilterRequest> {
    let trimmed = if bytes == [0] { &[][..] } else { bytes };
    if trimmed.is_empty() {
        return Ok(FilterRequest { filters: vec![] });
    }
    serde_json::from_slice(trimmed).map_err(|e| {
        Error::InvalidShape(format!("filter_json: {e}"))
    })
}

/// Apply filters to NCHW f32 image `[1, 3, H, W]` in \[0, 1\].
pub fn apply_filters_nchw(image: &Tensor, req: &FilterRequest) -> Result<Tensor> {
    let shape = image.shape().dims().to_vec();
    if shape.len() != 4 || shape[0] != 1 || shape[1] != 3 {
        return Err(Error::InvalidShape(format!(
            "filter expects NCHW [1,3,H,W], got {shape:?}"
        )));
    }
    let h = shape[2];
    let w = shape[3];
    let plane = h * w;
    let mut px = image.as_f32_slice()?.to_vec();

    for op in &req.filters {
        match op {
            FilterOp::Invert => {
                for v in &mut px {
                    *v = (1.0 - *v).clamp(0.0, 1.0);
                }
            }
            FilterOp::Grayscale => {
                for i in 0..plane {
                    let r = px[i];
                    let g = px[plane + i];
                    let b = px[2 * plane + i];
                    let y = 0.299 * r + 0.587 * g + 0.114 * b;
                    px[i] = y;
                    px[plane + i] = y;
                    px[2 * plane + i] = y;
                }
            }
            FilterOp::Brightness { amount } => {
                for v in &mut px {
                    *v = (*v + amount).clamp(0.0, 1.0);
                }
            }
            FilterOp::Contrast { amount } => {
                for v in &mut px {
                    *v = ((*v - 0.5) * amount + 0.5).clamp(0.0, 1.0);
                }
            }
        }
    }

    Tensor::from_f32_slice(Shape::new([1, 3, h, w]), &px)
}

pub fn sample_filter_json() -> Result<Tensor> {
    let body = r#"{"filters":[{"op":"invert"}]}"#;
    Tensor::text_tensor(body)
}

pub fn expect_filter_json(t: &Tensor) -> Result<FilterRequest> {
    if t.dtype() != uie_core::tensor::DataType::U8 {
        return Err(Error::DtypeMismatch {
            expected: uie_core::tensor::DataType::U8,
            got: t.dtype(),
        });
    }
    parse_filter_json(t.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conventions::{DIFFUSION_HEIGHT, DIFFUSION_WIDTH};

    #[test]
    fn invert_flips_pixel() {
        let n = 3 * DIFFUSION_HEIGHT * DIFFUSION_WIDTH;
        let vals = vec![0.25f32; n];
        let img =
            Tensor::from_f32_slice(Shape::new([1, 3, DIFFUSION_HEIGHT, DIFFUSION_WIDTH]), &vals)
                .unwrap();
        let req = FilterRequest {
            filters: vec![FilterOp::Invert],
        };
        let out = apply_filters_nchw(&img, &req).unwrap();
        assert!((out.as_f32_slice().unwrap()[0] - 0.75).abs() < 1e-5);
    }

    #[test]
    fn parse_chain() {
        let raw = br#"{"filters":[{"op":"grayscale"},{"op":"brightness","amount":0.2}]}"#;
        let req = parse_filter_json(raw).unwrap();
        assert_eq!(req.filters.len(), 2);
    }
}
