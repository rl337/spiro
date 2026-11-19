use std::path::Path;

use crate::registry::{ModelEntry, ModelRole};
use crate::scan::{extension_format, id_from_rel};

/// Shared stack key: checkpoint stem; for other roles, strip common role suffixes.
fn comfy_family(role_folder: &str, path: &Path) -> Option<String> {
    let stem = path.file_stem()?.to_str()?;
    let stem = stem.strip_suffix(".fp16").unwrap_or(stem);
    match role_folder {
        "checkpoints" | "diffusion_models" | "unet" => Some(stem.to_string()),
        "text_encoders" | "clip" | "vae" | "loras" | "controlnet" => {
            let stripped = stem
                .strip_suffix("_vae")
                .or_else(|| stem.strip_suffix("-vae"))
                .or_else(|| stem.strip_suffix("_clip"))
                .or_else(|| stem.strip_suffix("-clip"))
                .unwrap_or(stem);
            Some(stripped.to_string())
        }
        _ => Some(stem.to_string()),
    }
}

/// Map ComfyUI folder names to roles (see Comfy `folder_paths.py`).
pub fn classify_comfy_path(
    root_id: &str,
    _base: &Path,
    path: &Path,
    rel_str: &str,
) -> Option<ModelEntry> {
    let format = extension_format(path)?;
    let top = rel_str.split('/').next().unwrap_or("");
    let role = match top {
        "checkpoints" => ModelRole::DiffusionCheckpoint,
        "diffusion_models" | "unet" => ModelRole::DiffusionUnet,
        "text_encoders" | "clip" => ModelRole::TextEncoder,
        "vae" => ModelRole::Vae,
        "loras" => ModelRole::Lora,
        "controlnet" => ModelRole::ControlNet,
        "upscale_models" | "latent_upscale_models" => ModelRole::Upscaler,
        _ => ModelRole::Unknown,
    };
    let family = comfy_family(top, path);
    Some(ModelEntry {
        id: id_from_rel(root_id, rel_str),
        root_id: root_id.into(),
        path: path.to_path_buf(),
        format,
        role,
        family,
    })
}

/// Group Comfy **roles** that share a `family` (checkpoint base name) for pipeline wiring.
pub fn stack_candidates<'a>(entries: &'a [ModelEntry]) -> Vec<ComfyStackRef<'a>> {
    use std::collections::HashMap;
    let mut by_family: HashMap<&str, ComfyStackRef<'a>> = HashMap::new();

    for e in entries {
        let Some(fam) = e.family.as_deref() else {
            continue;
        };
        let slot = by_family.entry(fam).or_insert_with(|| ComfyStackRef {
            family: fam,
            checkpoint: None,
            text_encoder: None,
            unet: None,
            vae: None,
        });
        match e.role {
            ModelRole::DiffusionCheckpoint => slot.checkpoint = Some(e),
            ModelRole::TextEncoder => slot.text_encoder = Some(e),
            ModelRole::DiffusionUnet => slot.unet = Some(e),
            ModelRole::Vae => slot.vae = Some(e),
            _ => {}
        }
    }
    by_family
        .into_values()
        .filter(|s| s.checkpoint.is_some() || s.unet.is_some())
        .collect()
}

#[derive(Debug, Clone)]
pub struct ComfyStackRef<'a> {
    pub family: &'a str,
    pub checkpoint: Option<&'a ModelEntry>,
    pub text_encoder: Option<&'a ModelEntry>,
    pub unet: Option<&'a ModelEntry>,
    pub vae: Option<&'a ModelEntry>,
}
