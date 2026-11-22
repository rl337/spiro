use std::path::Path;

use uie_catalog::{ModelEntry, ModelFormat, ModelRole};
use uie_stub_models::ModalityKind;

/// Map registry metadata to stub modality contracts until format-specific kernels ship.
pub fn modality_for_catalog_entry(entry: &ModelEntry) -> ModalityKind {
    match (entry.format, entry.role) {
        (ModelFormat::Stub, _) => {
            ModalityKind::parse_file(&entry.path).unwrap_or(ModalityKind::LlmTextCompletion)
        }
        (ModelFormat::Safetensors, ModelRole::DiffusionCheckpoint)
        | (ModelFormat::Safetensors, ModelRole::DiffusionUnet) => {
            ModalityKind::DiffusionTextToImage
        }
        (ModelFormat::Safetensors, ModelRole::TextEncoder) => ModalityKind::Judge,
        (ModelFormat::Safetensors, ModelRole::Vae) => ModalityKind::ImageToImage,
        (ModelFormat::Safetensors, ModelRole::Llm) => ModalityKind::LlmTextCompletion,
        (ModelFormat::Gguf, ModelRole::Llm) => ModalityKind::LlmTextCompletion,
        _ => guess_from_path(&entry.path),
    }
}

pub fn guess_from_path(path: &Path) -> ModalityKind {
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if stem.contains("vae") {
        return ModalityKind::ImageToImage;
    }
    if stem.contains("clip") || stem.contains("encoder") {
        return ModalityKind::Judge;
    }
    if stem.contains("qwen") || stem.contains("diffusion") || stem.contains("unet") {
        return ModalityKind::DiffusionTextToImage;
    }
    ModalityKind::DiffusionTextToImage
}
