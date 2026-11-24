//! Browser WASM inference facade (US-026).
//!
//! v1: CPU Tract ONNX (MNIST). `DevicePolicy::WebGpu` is reserved and fails loudly.

mod device;
mod engine;

pub use device::{DevicePolicy, DEVICE_CPU, DEVICE_WEBGPU};
pub use engine::WasmInferenceEngine;
