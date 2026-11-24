//! wasm-bindgen engine: load ONNX bytes, run MNIST-shaped F32 infer (US-026).

use js_sys::Uint8Array;
use serde::Serialize;
use uie_core::backend::cpu::CpuBackend;
use uie_core::telemetry::ExecutionRecorder;
use uie_core::tensor::{DataType, Shape, Tensor};
use uie_core::{InferenceExecutor, InferenceRuntime};
use uie_loader_onnx::OnnxModel;
use wasm_bindgen::prelude::*;

use crate::device::DevicePolicy;

const MNIST_ELEMS: usize = 28 * 28;

#[derive(Serialize)]
struct InferResultJs {
    digit: usize,
    logits: Vec<f32>,
    device: String,
    cpu_time_ms: f64,
    model: String,
}

/// In-browser inference engine (CPU Tract ONNX in v1).
#[wasm_bindgen]
pub struct WasmInferenceEngine {
    policy: DevicePolicy,
    runtime: InferenceRuntime,
    model: Option<OnnxModel>,
}

#[wasm_bindgen]
impl WasmInferenceEngine {
    /// Create an engine. `preferred_device`: `auto` | `cpu` | `webgpu`.
    /// `webgpu` fails with `webgpu_not_implemented` until a backend ships.
    #[wasm_bindgen(constructor)]
    pub fn new(preferred_device: &str) -> Result<WasmInferenceEngine, JsValue> {
        console_error_panic_hook::set_once();
        let policy = DevicePolicy::parse(preferred_device).map_err(to_js_err)?;
        policy.ensure_implemented().map_err(to_js_err)?;
        let runtime = InferenceRuntime::new(Box::new(CpuBackend::new()));
        Ok(Self {
            policy,
            runtime,
            model: None,
        })
    }

    /// Active device label (`cpu` in v1).
    #[wasm_bindgen(js_name = deviceLabel)]
    pub fn device_label(&self) -> String {
        self.policy.as_str().into()
    }

    /// Load an ONNX model from raw bytes (e.g. fetched `mnist-8.onnx`).
    #[wasm_bindgen(js_name = loadOnnx)]
    pub fn load_onnx(&mut self, name: &str, bytes: &Uint8Array) -> Result<(), JsValue> {
        self.policy.ensure_implemented().map_err(to_js_err)?;
        let data = bytes.to_vec();
        let model = OnnxModel::from_bytes(name, &data).map_err(to_js_err)?;
        self.model = Some(model);
        Ok(())
    }

    /// Run MNIST-shaped F32 input (`1×1×28×28` little-endian bytes) → JSON result.
    #[wasm_bindgen(js_name = inferMnistF32)]
    pub fn infer_mnist_f32(&self, input_bytes: &Uint8Array) -> Result<JsValue, JsValue> {
        self.policy.ensure_implemented().map_err(to_js_err)?;
        let model = self
            .model
            .as_ref()
            .ok_or_else(|| JsValue::from_str("model not loaded; call loadOnnx first"))?;

        let raw = input_bytes.to_vec();
        if raw.len() != MNIST_ELEMS * 4 {
            return Err(JsValue::from_str(&format!(
                "expected {} f32 bytes for MNIST, got {}",
                MNIST_ELEMS * 4,
                raw.len()
            )));
        }
        let mut floats = vec![0f32; MNIST_ELEMS];
        for (i, chunk) in raw.chunks_exact(4).enumerate() {
            floats[i] = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }
        let input =
            Tensor::from_f32_slice(Shape::new(vec![1, 1, 28, 28]), &floats).map_err(to_js_err)?;
        debug_assert_eq!(input.dtype(), DataType::F32);

        let mut session = self.runtime.create_session();
        let outcome = self
            .runtime
            .infer(
                model,
                &[input],
                &mut session,
                ExecutionRecorder::new("us-026-wasm-mnist"),
            )
            .map_err(to_js_err)?;

        let logits = outcome.outputs[0]
            .as_f32_slice()
            .map_err(to_js_err)?
            .to_vec();
        let digit = logits
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0);

        let result = InferResultJs {
            digit,
            logits,
            device: self.policy.as_str().into(),
            cpu_time_ms: outcome.trace.metrics.cpu_time_ms,
            model: model.metadata().name.clone(),
        };
        serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

fn to_js_err(e: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&e.to_string())
}

#[cfg(test)]
mod native_tests {
    use super::*;
    use std::path::PathBuf;

    fn repo_file(rel: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(rel)
    }

    #[test]
    fn us_026_from_bytes_digit_one_native() {
        let model_bytes = std::fs::read(repo_file("models/mnist-8.onnx")).unwrap();
        let fixture = std::fs::read(repo_file("models/fixtures/digit_one_f32.bin")).unwrap();
        let model = OnnxModel::from_bytes("mnist-8", &model_bytes).unwrap();
        let runtime = InferenceRuntime::new(Box::new(CpuBackend::new()));
        let mut floats = vec![0f32; MNIST_ELEMS];
        for (i, chunk) in fixture.chunks_exact(4).enumerate() {
            floats[i] = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }
        let input = Tensor::from_f32_slice(Shape::new(vec![1, 1, 28, 28]), &floats).unwrap();
        let mut session = runtime.create_session();
        let outcome = runtime
            .infer(
                &model,
                &[input],
                &mut session,
                ExecutionRecorder::new("us-026-native-bytes"),
            )
            .unwrap();
        uie_core::telemetry::assert_phases(
            &outcome.trace,
            &["onnx_validate_inputs", "onnx_tract_run"],
        );
        let logits = outcome.outputs[0].as_f32_slice().unwrap();
        let digit = logits
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        assert_eq!(digit, 1);
    }

    #[test]
    fn us_026_webgpu_rejected() {
        let err = DevicePolicy::parse("webgpu")
            .unwrap()
            .ensure_implemented()
            .unwrap_err();
        assert!(err.to_string().contains("webgpu_not_implemented"), "{err}");
    }
}
