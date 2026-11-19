use std::path::{Path, PathBuf};

use tracing::info;
use uie_core::backend::cpu::CpuBackend;
use uie_core::telemetry::ExecutionRecorder;
use uie_core::tensor::{DataType, Shape, Tensor};
use uie_core::InferenceRuntime;
use uie_loader_onnx::OnnxModelLoader;

const MNIST_SHAPE: [usize; 4] = [1, 1, 28, 28];

fn main() -> uie_core::Result<()> {
    tracing_subscriber::fmt::init();

    let model_path = resolve_model_path();
    let fixture_path = resolve_fixture_path();

    let backend = Box::new(CpuBackend::new());
    let runtime = InferenceRuntime::new(backend).with_loader(Box::new(OnnxModelLoader::new()));

    let model = runtime.load_model(&model_path)?;
    info!(
        model = %model.metadata().name,
        format = %model.metadata().format,
        ops = model.metadata().ops,
        "loaded model"
    );

    let input = load_input_tensor(&fixture_path)?;
    let mut session = runtime.create_session();
    session.bind("input", input.clone());

    let outcome = runtime.infer(
        model.as_ref(),
        &[input],
        &mut session,
        ExecutionRecorder::new("uie-mnist"),
    )?;
    let outputs = outcome.outputs;
    let logits = outputs[0].as_f32_slice()?;
    let (digit, score) = logits
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(d, s)| (d, *s))
        .expect("logits");

    println!("model: {}", model_path.display());
    println!("fixture: {}", fixture_path.display());
    println!("predicted digit: {digit} (score={score:.4})");
    println!("logits: {logits:?}");

    Ok(())
}

fn resolve_model_path() -> PathBuf {
    if let Ok(p) = std::env::var("UIE_MNIST_MODEL") {
        return PathBuf::from(p);
    }
    workspace_relative("models/mnist-8.onnx")
}

fn resolve_fixture_path() -> PathBuf {
    if let Ok(p) = std::env::var("UIE_MNIST_FIXTURE") {
        return PathBuf::from(p);
    }
    workspace_relative("models/fixtures/digit_one_f32.bin")
}

fn workspace_relative(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn load_input_tensor(path: &Path) -> uie_core::Result<Tensor> {
    let bytes = std::fs::read(path).map_err(|e| uie_core::Error::Runtime(e.to_string()))?;
    if bytes.len() != MNIST_SHAPE.iter().product::<usize>() * 4 {
        return Err(uie_core::Error::InvalidShape(format!(
            "fixture {}: expected {} f32 bytes, got {}",
            path.display(),
            MNIST_SHAPE.iter().product::<usize>() * 4,
            bytes.len()
        )));
    }
    Tensor::from_bytes(DataType::F32, Shape::new(MNIST_SHAPE), bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mnist_inference_digit_one() -> uie_core::Result<()> {
        let model_path = resolve_model_path();
        if !model_path.exists() {
            eprintln!("skip: model not at {}", model_path.display());
            return Ok(());
        }

        let backend = Box::new(CpuBackend::new());
        let runtime = InferenceRuntime::new(backend).with_loader(Box::new(OnnxModelLoader::new()));
        let model = runtime.load_model(&model_path)?;
        let input = load_input_tensor(&resolve_fixture_path())?;
        let mut session = runtime.create_session();
        let outcome = runtime.infer(
            model.as_ref(),
            &[input],
            &mut session,
            ExecutionRecorder::new("uie-mnist"),
        )?;
        let outputs = outcome.outputs;
        let logits = outputs[0].as_f32_slice()?;
        let digit = logits
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;

        assert_eq!(digit, 1, "vertical stroke fixture should classify as 1");
        Ok(())
    }
}
