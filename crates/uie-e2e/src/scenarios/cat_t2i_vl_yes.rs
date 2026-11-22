//! US-012 + US-016: T2I emits cat fixture tensor → VL answers yes/no for cat presence.

use std::env;
use std::path::PathBuf;

use uie_catalog::ModelCatalog;
use uie_core::telemetry::ExecutionRecorder;
use uie_core::CpuBackend;
use uie_core::InferenceRuntime;
use uie_stub_models::{sample_input, ModalityKind, StubModelLoader};

#[cfg(feature = "safetensors")]
use uie_loader_safetensors::SafetensorsModelLoader;

use crate::cat_fixture_loader::CatFixtureDiffusionLoader;
use crate::catalog::catalog_for_e2e;
use crate::harness::{E2eContext, E2eError, E2eTest};
use crate::spiro_image::save_f32_nchw_rgb;

#[derive(Debug, Default, Clone, Copy)]
pub struct CatT2iVlYes;

impl E2eTest for CatT2iVlYes {
    fn name(&self) -> &'static str {
        "cat_t2i_vl_yes"
    }

    fn description(&self) -> &'static str {
        "T2I cat fixture stub → PNG → Qwen VL asked yes/no cat; asserts yes (proves both hops)."
    }

    fn run(&self, ctx: &mut E2eContext, args: &[String]) -> Result<(), E2eError> {
        let catalog =
            catalog_for_e2e(&ctx.repo_root).map_err(|e| E2eError::Failed(e.to_string()))?;

        ctx.phase("t2i_cat_fixture");
        let stub_path = ctx.repo_root.join("models/stubs/diffusion_t2i_cat.stub");
        ctx.record(format!("diffusion_path:{}", stub_path.display()));

        let rt = InferenceRuntime::new(Box::new(CpuBackend::new()))
            .with_loader(Box::new(CatFixtureDiffusionLoader::from_repo(
                &ctx.repo_root,
            )))
            .with_loader(Box::new(StubModelLoader));
        #[cfg(feature = "safetensors")]
        let rt = rt.with_loader(Box::new(SafetensorsModelLoader));

        let diff_model = rt
            .load_model(&stub_path)
            .map_err(|e| E2eError::Failed(format!("cat fixture load: {e:?}")))?;

        let t2i_in = sample_input(ModalityKind::DiffusionTextToImage)
            .map_err(|e| E2eError::Failed(e.to_string()))?;
        let mut session = rt.create_session();
        let t2i_out = rt
            .infer(
                diff_model.as_ref(),
                &t2i_in,
                &mut session,
                ExecutionRecorder::new("US-012"),
            )
            .map_err(|e| E2eError::Failed(format!("t2i infer: {e:?}")))?;
        ctx.assert(!t2i_out.outputs.is_empty(), "t2i produced no tensors")?;
        ctx.record("t2i_source:cat_fixture_stub");

        let out_dir = artifact_dir(args);
        let png = out_dir.join("cat_t2i_vl.png");
        save_f32_nchw_rgb(&t2i_out.outputs[0], &png, 512, 512)
            .map_err(|e| E2eError::Failed(e.to_string()))?;
        ctx.record(format!("wrote_image:{}", png.display()));

        if args.iter().any(|a| a == "--skip-llm") {
            ctx.record("skipped_vl_cat_check");
            return Ok(());
        }

        #[cfg(feature = "gguf")]
        {
            ctx.phase("vl_cat_yes_no");
            vl_assert_cat_yes(ctx, &catalog, &png, args)?;
        }
        #[cfg(not(feature = "gguf"))]
        {
            return Err(E2eError::Failed(
                "cat_t2i_vl_yes requires --features gguf (or spiro-chain)".into(),
            ));
        }

        Ok(())
    }
}

#[cfg(feature = "gguf")]
fn vl_assert_cat_yes(
    ctx: &mut E2eContext,
    catalog: &ModelCatalog,
    png: &std::path::Path,
    args: &[String],
) -> Result<(), E2eError> {
    use crate::gguf_resolve::{find_qwen_vl_2b_bundle, flag_value};
    use uie_loader_gguf::{describe_image_mmproj, VisionDescribeOptions};

    let bundle = find_qwen_vl_2b_bundle(catalog).map_err(|e| E2eError::Failed(e.to_string()))?;
    let prompt = flag_value(args, "--prompt")
        .unwrap_or_else(|| "Is there a cat in this image? Answer only yes or no.".into());
    let mut opts = VisionDescribeOptions::default();
    opts.max_new_tokens = flag_value(args, "--max-new")
        .and_then(|s| s.parse().ok())
        .unwrap_or(24);
    opts.temperature = 0.1;

    let out = describe_image_mmproj(&bundle.llm, &bundle.mmproj, png, &prompt, &opts)
        .map_err(|e| E2eError::Failed(format!("vl cat check: {e:?}")))?;

    let answer = out.text.trim().to_ascii_lowercase();
    eprintln!("vl_cat_answer: {}", out.text.trim());
    ctx.record(format!("vl_cat_answer:{}", out.text.trim()));
    let says_yes =
        answer.starts_with("yes") || answer.starts_with("yeah") || answer.contains(" yes");
    let says_no = answer.starts_with("no") || answer.starts_with("nope");
    ctx.assert(
        says_yes && !says_no,
        format!("expected yes for cat image, got: {}", out.text.trim()),
    )?;
    ctx.record("cat_vl_yes_ok");
    Ok(())
}

fn artifact_dir(args: &[String]) -> PathBuf {
    args.iter()
        .position(|a| a == "--out-dir")
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from)
        .or_else(|| env::var("UIE_ARTIFACT_DIR").ok().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("/tmp/uie-artifacts"))
}
