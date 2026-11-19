use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComputeDevice {
    Cpu,
    Gpu,
    /// e.g. NPU, TPU — label in telemetry only until backends report hardware.
    Accelerator,
    Unknown,
}