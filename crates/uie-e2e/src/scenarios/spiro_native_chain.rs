//! US-012 → US-016/017: Spiro-native chain (registry paths, safetensors mmap, Qwen2-VL GGUF), no Comfy/LM Studio HTTP.

use std::env;
use std::path::PathBuf;

use uie_catalog::{executor::load_executor_for_entry, ModelCatalog, ModelFormat, ModelRole};
use uie_core::telemetry::ExecutionRecorder;
use uie_core::CpuBackend;
use uie_core::InferenceRuntime;
use uie_loader_safetensors::{modality_for_catalog_entry, SafetensorsModelLoader};
use uie_stub_models::{sample_input, ModalityKind, StubModelLoader};

use crate::catalog::catalog_for_e2e;
use crate::harness::{E2eContext, E2eError, E2eTest};
use crate::spiro_image::save_f32_nchw_rgb;

#[derive(Debug, Default, Clone, Copy)]
pub struct SpiroNativeQwenChain;

impl E2eTest for SpiroNativeQwenChain {
    fn name(&self) -> &'static str {
        "spiro_native_qwen_chain"
    }

    fn description(&self) -> &'static str {
        "Registry → safetensors T2I hop → PNG artifact → Qwen VL ~2B (LM Studio mmproj) describes the image."
    }

    fn run(&self, ctx: &mut E2eContext, args: &[String]) -> Result<(), E2eError> {
        let catalog =
            catalog_for_e2e(&ctx.repo_root).map_err(|e| E2eError::Failed(e.to_string()))?;
        ctx.phase("pick_diffusion_path");
        let diff_path = pick_diffusion_path(&catalog, &ctx.repo_root);
        ctx.record(format!("diffusion_path:{}", diff_path.display()));

        ctx.phase("t2i_infer");
        let rt = InferenceRuntime::new(Box::new(CpuBackend::new()))
            .with_loader(Box::new(StubModelLoader))
            .with_loader(Box::new(SafetensorsModelLoader));
        let diff_entry = catalog
            .registry()
            .entries()
            .iter()
            .find(|e| e.path == diff_path)
            .cloned()
            .unwrap_or_else(|| stub_entry(diff_path.clone()));
        let diff_model = if diff_entry.format == uie_catalog::ModelFormat::Safetensors {
            SafetensorsModelLoader
                .load_catalog_entry(&diff_entry, rt.backend())
                .map_err(|e| E2eError::Failed(format!("safetensors load: {e:?}")))?
        } else {
            load_executor_for_entry(&rt, &diff_entry)
                .map_err(|e| E2eError::Failed(format!("stub load: {e:?}")))?
        };
        ctx.record(format!(
            "t2i_modality:{}",
            modality_for_catalog_entry(&diff_entry).id_str()
        ));
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
        let image_tensors = t2i_out.outputs;
        ctx.assert(!image_tensors.is_empty(), "t2i produced no tensors")?;
        if diff_entry.format == uie_catalog::ModelFormat::Safetensors {
            assert_safetensors_hop_telemetry(ctx, &t2i_out.trace)?;
        }

        let out_dir = artifact_dir(args);
        let png = out_dir.join("spiro_native_image.png");
        save_f32_nchw_rgb(&image_tensors[0], &png, 512, 512)
            .map_err(|e| E2eError::Failed(e.to_string()))?;
        ctx.record(format!("wrote_image:{}", png.display()));

        if args.iter().any(|a| a == "--skip-llm") {
            ctx.record("skipped_qwen_vl_describe");
            return Ok(());
        }

        #[cfg(feature = "gguf")]
        {
            ctx.phase("qwen_vl_describe");
            run_qwen_vl_on_image(ctx, &catalog, &png, args)?;
        }
        #[cfg(not(feature = "gguf"))]
        {
            return Err(E2eError::Failed(
                "spiro_native_qwen_chain requires --features spiro-chain (gguf vision)".into(),
            ));
        }

        Ok(())
    }
}

#[cfg(feature = "gguf")]
fn run_qwen_vl_on_image(
    ctx: &mut E2eContext,
    catalog: &ModelCatalog,
    png: &std::path::Path,
    args: &[String],
) -> Result<(), E2eError> {
    use crate::gguf_resolve::{find_qwen_vl_2b_bundle, flag_value};
    use uie_loader_gguf::{describe_image_mmproj, VisionDescribeOptions};

    let bundle = find_qwen_vl_2b_bundle(catalog).map_err(|e| E2eError::Failed(e.to_string()))?;
    ctx.record(format!("qwen_vl_llm:{}", bundle.llm.display()));
    ctx.record(format!("qwen_vl_mmproj:{}", bundle.mmproj.display()));

    let prompt = flag_value(args, "--prompt").unwrap_or_else(|| {
        "What is in this image? Describe the main subjects, colors, and layout in 2-3 sentences."
            .into()
    });
    let mut opts = VisionDescribeOptions::default();
    if let Some(n) = flag_value(args, "--max-new").and_then(|s| s.parse().ok()) {
        opts.max_new_tokens = n;
    }

    let out = describe_image_mmproj(&bundle.llm, &bundle.mmproj, png, &prompt, &opts)
        .map_err(|e| E2eError::Failed(format!("qwen vl describe: {e:?}")))?;

    ctx.record("qwen_vl_completion_metrics");
    ctx.assert(!out.text.trim().is_empty(), "empty vision description")?;
    ctx.assert(
        out.metrics.completion_tokens > 0,
        "expected vision completion_tokens > 0",
    )?;
    ctx.assert(
        out.metrics.total_ms >= 0.0,
        "expected non-negative vision total_ms",
    )?;
    eprintln!("image_description: {}", out.text.trim());
    Ok(())
}

fn assert_safetensors_hop_telemetry(
    ctx: &mut E2eContext,
    trace: &uie_telemetry::ExecutionTrace,
) -> Result<(), E2eError> {
    let mmap = trace
        .phases
        .iter()
        .any(|p| p.name.starts_with("safetensors_mmap_"));
    ctx.assert(mmap, "missing safetensors_mmap_* phase in t2i trace")?;
    ctx.assert(
        trace
            .phases
            .iter()
            .any(|p| p.name == "safetensors_weight_bind"),
        "missing safetensors_weight_bind phase in t2i trace",
    )?;
    Ok(())
}

fn pick_diffusion_path(catalog: &ModelCatalog, repo: &std::path::Path) -> PathBuf {
    catalog
        .registry()
        .entries()
        .iter()
        .find(|e| {
            e.format == ModelFormat::Safetensors
                && matches!(
                    e.role,
                    ModelRole::DiffusionCheckpoint | ModelRole::DiffusionUnet
                )
        })
        .map(|e| e.path.clone())
        .unwrap_or_else(|| repo.join("models/stubs/diffusion_t2i.stub"))
}

fn stub_entry(path: PathBuf) -> uie_catalog::ModelEntry {
    uie_catalog::ModelEntry {
        id: format!("stub:{}", path.display()),
        root_id: "stub".into(),
        path,
        format: ModelFormat::Stub,
        role: ModelRole::Unknown,
        family: None,
    }
}

fn artifact_dir(args: &[String]) -> PathBuf {
    args.iter()
        .position(|a| a == "--out-dir")
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from)
        .or_else(|| env::var("UIE_ARTIFACT_DIR").ok().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("/tmp/uie-artifacts"))
}
