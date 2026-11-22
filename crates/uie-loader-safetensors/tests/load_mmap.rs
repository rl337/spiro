use uie_catalog::{ModelEntry, ModelFormat, ModelRole};
use uie_core::backend::cpu::CpuBackend;
use uie_loader_safetensors::SafetensorsModelLoader;
use uie_stub_models::{sample_input, ModalityKind};

#[test]
fn loads_real_safetensors_when_present() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/comfy-shared/models/checkpoints/tiny_random_vit.safetensors");
    if !path.is_file() {
        eprintln!("skip: no fixture {}", path.display());
        return;
    }
    let loader = SafetensorsModelLoader;
    let backend = CpuBackend::new();
    let entry = ModelEntry {
        id: "t:vit".into(),
        root_id: "t".into(),
        path: path.clone(),
        format: ModelFormat::Safetensors,
        role: ModelRole::DiffusionCheckpoint,
        family: Some("tiny_random_vit".into()),
    };
    let exec = loader.load_catalog_entry(&entry, &backend).expect("load");
    assert!(exec.metadata().format.contains("safetensors"));
    let inputs = sample_input(ModalityKind::DiffusionTextToImage).unwrap();
    let mut rec = uie_telemetry::ExecutionRecorder::new("US-012");
    let outs = exec.infer(&inputs, &mut rec).unwrap();
    assert!(!outs.is_empty());
}
