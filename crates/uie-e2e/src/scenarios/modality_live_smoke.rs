//! US-010–019: trivial **live** infer on every stub modality (cloud-safe, no GPU).

use std::path::{Path, PathBuf};

use uie_catalog::{executor::load_executor_for_entry, ModelFormat};
use uie_core::telemetry::ExecutionRecorder;
use uie_core::CpuBackend;
use uie_core::InferenceRuntime;
use uie_stub_models::telemetry_assert::assert_stub_modality_telemetry;
use uie_stub_models::{sample_input, ModalityKind, StubModelLoader};

use crate::catalog::catalog_for_e2e;
use crate::harness::{E2eContext, E2eError, E2eTest};

#[derive(Debug, Default, Clone, Copy)]
pub struct ModalityLiveSmoke;

impl E2eTest for ModalityLiveSmoke {
    fn name(&self) -> &'static str {
        "modality_live_smoke"
    }

    fn description(&self) -> &'static str {
        "Load each stub modality, run sample_input → infer via InferenceRuntime (live CPU path, US-010–019)."
    }

    fn run(&self, ctx: &mut E2eContext, args: &[String]) -> Result<(), E2eError> {
        let via_registry = !args.iter().any(|a| a == "--stubs-only");
        let rt = InferenceRuntime::new(Box::new(CpuBackend::new()))
            .with_loader(Box::new(StubModelLoader));

        let catalog = if via_registry {
            ctx.phase("registry_resolve");
            Some(catalog_for_e2e(&ctx.repo_root).map_err(|e| E2eError::Failed(e.to_string()))?)
        } else {
            None
        };

        let stubs_root = ctx.repo_root.join("models/stubs");
        for kind in ModalityKind::ALL {
            ctx.phase(format!("modality_{}", kind.id_str()));
            let path = resolve_stub_path(kind, &stubs_root, catalog.as_ref(), via_registry)?;
            ctx.record(format!("modality_path:{}", kind.id_str()));

            let model = load_executor_for_entry(&rt, &stub_entry_path(&path))
                .map_err(|e| E2eError::Failed(format!("{}: {e}", kind.id_str())))?;
            let inputs = sample_input(kind).map_err(|e| E2eError::Failed(e.to_string()))?;
            let mut session = rt.create_session();
            let outcome = rt
                .infer(
                    model.as_ref(),
                    &inputs,
                    &mut session,
                    ExecutionRecorder::new(kind.story_id()),
                )
                .map_err(|e| E2eError::Failed(format!("{} infer: {e}", kind.id_str())))?;
            ctx.assert(
                !outcome.outputs.is_empty(),
                format!("{}: empty outputs", kind.id_str()),
            )?;
            assert_stub_modality_telemetry(&outcome.trace, kind);
            ctx.record(format!("modality_live_ok:{}", kind.id_str())); // assert_story_events
        }
        Ok(())
    }
}

fn stub_entry_path(path: &Path) -> uie_catalog::ModelEntry {
    uie_catalog::ModelEntry {
        id: format!("path:{}", path.display()),
        root_id: "e2e".into(),
        path: path.to_path_buf(),
        format: ModelFormat::Stub,
        role: uie_catalog::ModelRole::Unknown,
        family: None,
    }
}

fn resolve_stub_path(
    kind: ModalityKind,
    stubs_root: &Path,
    catalog: Option<&uie_catalog::ModelCatalog>,
    try_registry: bool,
) -> Result<PathBuf, E2eError> {
    let file = format!("{}.stub", kind.id_str());
    let direct = stubs_root.join(&file);
    if try_registry {
        if let Some(cat) = catalog {
            if let Some(entry) = cat.registry().entries().iter().find(|e| {
                e.format == ModelFormat::Stub
                    && e.path.file_name().is_some_and(|n| n == file.as_str())
            }) {
                return Ok(entry.path.clone());
            }
        }
    }
    if !direct.is_file() {
        return Err(E2eError::Failed(format!(
            "stub missing: {}",
            direct.display()
        )));
    }
    Ok(direct)
}
