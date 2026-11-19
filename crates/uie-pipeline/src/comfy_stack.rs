use std::path::{Path, PathBuf};

use uie_catalog::{ModelCatalog, ModelEntry, ModelRole};

use crate::byte_pipeline::PipelineHop;

/// Ordered stages for a Comfy-style stack (text → latent → image stubs today).
#[derive(Debug, Clone)]
pub struct ComfyPipelinePlan {
    pub family: String,
    pub hops: Vec<PipelineHop>,
}

/// Build hops from scanned registry entries; missing roles fall back to repo stubs for tests.
pub fn build_comfy_byte_pipeline(
    catalog: &ModelCatalog,
    family: &str,
    stub_fallback_dir: &Path,
) -> Option<ComfyPipelinePlan> {
    let entries: Vec<_> = catalog.registry().entries().to_vec();
    let stacks = uie_catalog::comfy::stack_candidates(&entries);
    let stack = stacks.into_iter().find(|s| s.family == family)?;

    let mut hops = Vec::new();

    push_hop(
        &mut hops,
        "text_encoder",
        stack.text_encoder,
        stub_fallback("judge.stub", stub_fallback_dir),
    );
    push_hop(
        &mut hops,
        "diffusion",
        stack.unet.or(stack.checkpoint),
        stub_fallback("diffusion_t2i.stub", stub_fallback_dir),
    );
    push_hop(
        &mut hops,
        "vae",
        stack.vae,
        stub_fallback("diffusion_t2i.stub", stub_fallback_dir),
    );

    if hops.is_empty() {
        return None;
    }

    Some(ComfyPipelinePlan {
        family: family.into(),
        hops,
    })
}

fn push_hop(
    hops: &mut Vec<PipelineHop>,
    label: &str,
    entry: Option<&ModelEntry>,
    fallback: PathBuf,
) {
    let path = entry.map(|e| e.path.clone()).unwrap_or(fallback);
    hops.push(PipelineHop {
        label: label.into(),
        model_path: path,
    });
}

fn stub_fallback(name: &str, dir: &Path) -> PathBuf {
    dir.join(name)
}

/// Pick the first checkpoint family in the registry (for demos).
#[allow(dead_code)]
pub fn first_comfy_family(catalog: &ModelCatalog) -> Option<String> {
    catalog
        .registry()
        .entries()
        .iter()
        .find(|e| e.role == ModelRole::DiffusionCheckpoint)
        .and_then(|e| e.family.clone())
}
