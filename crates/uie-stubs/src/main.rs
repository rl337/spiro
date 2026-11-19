use std::env;
use std::path::{Path, PathBuf};

use uie_core::backend::cpu::CpuBackend;
use uie_core::telemetry::ExecutionRecorder;
use uie_core::InferenceRuntime;
use uie_stub_models::{sample_input, ModalityKind, StubModelLoader};

fn main() -> uie_core::Result<()> {
    tracing_subscriber::fmt::init();
    let path = resolve_stub_path()?;

    let backend = Box::new(CpuBackend::new());
    let runtime = InferenceRuntime::new(backend).with_loader(Box::new(StubModelLoader::default()));
    let model = runtime.load_model(&path)?;
    let kind = infer_kind_from_path(&path)?;

    println!("story: {}", kind.story_id());
    println!("modality: {}", kind.id_str());
    println!("model: {}", model.metadata().name);
    print_io(model.as_ref());

    let inputs = sample_input(kind)?;
    let mut session = runtime.create_session();
    let outcome = runtime.infer(
        model.as_ref(),
        &inputs,
        &mut session,
        ExecutionRecorder::new("uie-stubs"),
    )?;
    let outputs = outcome.outputs;

    for (i, out) in outputs.iter().enumerate() {
        let spec = &model.outputs()[i];
        println!(
            "output `{}`: dtype={:?} shape={:?} nbytes={}",
            spec.name,
            out.dtype(),
            out.shape().dims(),
            out.nbytes()
        );
        describe_payload(out, spec.name.as_str());
    }

    Ok(())
}

fn print_io(model: &dyn uie_core::InferenceExecutor) {
    for spec in model.inputs() {
        println!(
            "input spec `{}`: {:?} {:?}",
            spec.name,
            spec.dtype,
            spec.shape.dims()
        );
    }
    for spec in model.outputs() {
        println!(
            "output spec `{}`: {:?} {:?}",
            spec.name,
            spec.dtype,
            spec.shape.dims()
        );
    }
}

fn describe_payload(tensor: &uie_core::Tensor, name: &str) {
    match tensor.dtype() {
        uie_core::DataType::I32 => {
            if let Ok(ids) = tensor.as_i32_slice() {
                let preview = ids.iter().take(8).copied().collect::<Vec<_>>();
                println!("  {name} token preview: {preview:?}");
            }
        }
        uie_core::DataType::F32 => {
            if let Ok(slice) = tensor.as_f32_slice() {
                let energy: f32 = slice.iter().map(|v| v * v).sum();
                println!("  {name} f32 energy: {energy:.6}");
            }
        }
        _ => {}
    }
}

fn resolve_stub_path() -> uie_core::Result<PathBuf> {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        return Ok(PathBuf::from(&args[1]));
    }
    Ok(workspace_root().join("models/stubs/llm.stub"))
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap_or_else(|_| Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
}

fn infer_kind_from_path(path: &Path) -> uie_core::Result<ModalityKind> {
    ModalityKind::parse_file(path)
}
