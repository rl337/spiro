//! Preferred compute device for the browser facade (US-026).

use uie_core::error::{Error, Result};

pub const DEVICE_CPU: &str = "cpu";
pub const DEVICE_WEBGPU: &str = "webgpu";
pub const DEVICE_AUTO: &str = "auto";

/// Browser device preference. WebGPU is reserved for a future backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevicePolicy {
    Cpu,
    /// Reserved — v1 returns [`Error::UnsupportedOp`] with `webgpu_not_implemented`.
    WebGpu,
}

impl DevicePolicy {
    pub fn parse(preferred: &str) -> Result<Self> {
        match preferred.trim().to_ascii_lowercase().as_str() {
            DEVICE_CPU | DEVICE_AUTO | "" => Ok(Self::Cpu),
            DEVICE_WEBGPU => Ok(Self::WebGpu),
            other => Err(Error::UnsupportedOp(format!(
                "unknown preferred_device `{other}` (use auto|cpu|webgpu)"
            ))),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cpu => DEVICE_CPU,
            Self::WebGpu => DEVICE_WEBGPU,
        }
    }

    /// v1 only supports CPU WASM.
    pub fn ensure_implemented(self) -> Result<()> {
        match self {
            Self::Cpu => Ok(()),
            Self::WebGpu => Err(Error::UnsupportedOp(
                "webgpu_not_implemented: WebGPU backend reserved for future work (US-026)".into(),
            )),
        }
    }
}
