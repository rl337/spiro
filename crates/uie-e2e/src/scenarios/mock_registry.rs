//! US-021: mock stub models discoverable via TestModelDetector + stub runtime load.

use crate::catalog::catalog_for_e2e;
use crate::harness::{E2eContext, E2eError, E2eTest};
use uie_catalog::{executor::load_executor_for_entry, ModelFormat, ModelRole};
use uie_core::CpuBackend;
use uie_core::InferenceRuntime;
use uie_stub_models::StubModelLoader;

#[derive(Debug, Default, Clone, Copy)]
pub struct MockRegistryDiscovery;

impl E2eTest for MockRegistryDiscovery {
    fn name(&self) -> &'static str {
        "mock_registry_discovery"
    }

    fn description(&self) -> &'static str {
        "Scan model roots with TestModelDetector overlay; list mock-lmstudio / mock-comfy stubs (US-021)."
    }

    fn run(&self, ctx: &mut E2eContext, args: &[String]) -> Result<(), E2eError> {
        let catalog =
            catalog_for_e2e(&ctx.repo_root).map_err(|e| E2eError::Failed(e.to_string()))?;
        ctx.phase("registry_scan");
        let entries = catalog.registry().entries();
        ctx.record(format!("registry_entries:{}", entries.len()));
        for e in entries {
            if e.root_id.starts_with("mock-") {
                eprintln!(
                    "mock-model\t{}\t{:?}\t{:?}\t{}",
                    e.id,
                    e.role,
                    e.format,
                    e.path.display()
                );
            }
        }
        let mock_llm = entries.iter().find(|e| {
            e.root_id == "mock-lmstudio"
                && e.format == ModelFormat::Stub
                && e.role == ModelRole::Llm
        });
        ctx.assert(mock_llm.is_some(), "missing mock-lmstudio LLM stub entry")?;
        let mock_diffusion = entries
            .iter()
            .find(|e| e.root_id == "mock-comfy" && e.role == ModelRole::DiffusionCheckpoint);
        ctx.assert(
            mock_diffusion.is_some(),
            "missing mock-comfy diffusion stub entry",
        )?;

        if args.iter().any(|a| a == "--load-stub") {
            ctx.phase("load_stub_executor");
            let entry = mock_llm.unwrap();
            let rt = InferenceRuntime::new(Box::new(CpuBackend::new()))
                .with_loader(Box::new(StubModelLoader));
            let model =
                load_executor_for_entry(&rt, entry).map_err(|e| E2eError::Failed(e.to_string()))?;
            ctx.assert(
                model.metadata().format == "stub",
                "expected stub executor from registry path",
            )?;
            ctx.record("stub_model_loaded"); // assert_story_events
        }
        Ok(())
    }
}
