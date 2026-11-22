#[cfg(feature = "gguf")]
use std::path::Path;

#[cfg(feature = "gguf")]
use uie_catalog::ModelCatalog;

#[cfg(feature = "gguf")]
use crate::gguf_resolve::{find_qwen_vl_2b_bundle, flag_value};
#[cfg(feature = "gguf")]
use crate::harness::E2eError;
#[cfg(feature = "gguf")]
use uie_loader_gguf::{describe_image_mmproj, CompleteResult, VisionDescribeOptions};

#[cfg(feature = "gguf")]
pub fn describe_with_vl(
    catalog: &ModelCatalog,
    png: &Path,
    args: &[String],
    default_prompt: &str,
    max_new_default: u32,
    temperature: f32,
) -> Result<CompleteResult, E2eError> {
    let bundle = find_qwen_vl_2b_bundle(catalog).map_err(|e| E2eError::Failed(e.to_string()))?;
    let prompt = flag_value(args, "--prompt").unwrap_or_else(|| default_prompt.to_string());
    let mut opts = VisionDescribeOptions::default();
    opts.temperature = temperature;
    opts.max_new_tokens = flag_value(args, "--max-new")
        .and_then(|s| s.parse().ok())
        .unwrap_or(max_new_default);
    describe_image_mmproj(&bundle.llm, &bundle.mmproj, png, &prompt, &opts)
        .map_err(|e| E2eError::Failed(format!("vl describe: {e:?}")))
}
