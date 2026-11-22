//! US-012 → US-016/010: Spiro-native chain (registry paths, safetensors mmap, stub/GGUF), no Comfy/LM Studio HTTP.

use std::env;
use std::path::PathBuf;

use uie_catalog::{executor::load_executor_for_entry, ModelCatalog, ModelFormat, ModelRole};
use uie_core::telemetry::ExecutionRecorder;
use uie_core::CpuBackend;
use uie_core::InferenceRuntime;
use uie_loader_safetensors::{modality_for_catalog_entry, SafetensorsModelLoader};
use uie_stub_models::telemetry_assert::assert_stub_modality_telemetry;
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
        "Load weights via Spiro: safetensors T2I hop → image_to_text → optional GGUF caption (registry roots)."
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

        let out_dir = artifact_dir(args);
        let png = out_dir.join("spiro_native_image.png");
        save_f32_nchw_rgb(&image_tensors[0], &png, 512, 512)
            .map_err(|e| E2eError::Failed(e.to_string()))?;
        ctx.record(format!("wrote_image:{}", png.display()));

        ctx.phase("image_to_text");
        let i2t_path = ctx.repo_root.join("models/stubs/image_to_text.stub");
        let i2t = load_executor_for_entry(&rt, &stub_entry(i2t_path))
            .map_err(|e| E2eError::Failed(e.to_string()))?;
        session = rt.create_session();
        let i2t_in = if image_tensors[0].shape().dims() == &[1, 3, 224, 224][..] {
            vec![image_tensors[0].clone()]
        } else {
            sample_input(ModalityKind::ImageToText).map_err(|e| E2eError::Failed(e.to_string()))?
        };
        let cap = rt
            .infer(
                i2t.as_ref(),
                &i2t_in,
                &mut session,
                ExecutionRecorder::new("US-016"),
            )
            .map_err(|e| E2eError::Failed(e.to_string()))?;
        assert_stub_modality_telemetry(&cap.trace, ModalityKind::ImageToText);
        ctx.record("image_to_text_stub"); // assert_story_events

        #[cfg(feature = "gguf")]
        if !args.iter().any(|a| a == "--skip-llm") {
            ctx.phase("gguf_summarize");
            run_gguf_followup(ctx, &catalog, &cap.trace)?;
        }
        #[cfg(not(feature = "gguf"))]
        let _ = &catalog;

        Ok(())
    }
}

#[cfg(feature = "gguf")]
fn run_gguf_followup(
    ctx: &mut E2eContext,
    catalog: &ModelCatalog,
    _cap: &uie_telemetry::ExecutionTrace,
) -> Result<(), E2eError> {
    use crate::gguf_resolve::find_qwen3_2b_path;
    use uie_loader_gguf::{complete_gguf_file_chat, CompleteOptions};

    let model = find_qwen3_2b_path(catalog).map_err(|e| E2eError::Failed(e.to_string()))?;
    let prompt = "Summarize what the vision stub said about the generated image in one sentence.";
    let out = complete_gguf_file_chat(&model, prompt, &CompleteOptions::default())
        .map_err(|e| E2eError::Failed(e.to_string()))?;
    ctx.record("gguf_completion_metrics");
    ctx.assert(!out.text.is_empty(), "empty gguf summary")?;
    eprintln!("llm_summary: {}", out.text);
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
