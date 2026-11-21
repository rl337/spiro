//! US-010–US-019: live CPU smoke for all nine stub modalities via the E2E harness.

use uie_core::backend::cpu::CpuBackend;
use uie_core::telemetry::ExecutionRecorder;
use uie_core::InferenceRuntime;
use uie_stub_models::telemetry_assert::assert_stub_modality_telemetry;
use uie_stub_models::{sample_input, ModalityKind, StubModelLoader};

use crate::harness::{E2eContext, E2eError, E2eTest};

#[derive(Debug, Default, Clone, Copy)]
pub struct ModalityLiveSmoke;

impl E2eTest for ModalityLiveSmoke {
    fn name(&self) -> &'static str {
        "modality_live_smoke"
    }

    fn description(&self) -> &'static str {
        "Load each of the nine modality stubs under models/stubs/, run CPU infer with \
         sample inputs, assert outputs and stub telemetry (US-010–US-019)."
    }

    fn run(&self, ctx: &mut E2eContext, _args: &[String]) -> Result<(), E2eError> {
        ctx.phase("modality_live_smoke");
        let stubs_dir = ctx.repo_root.join("models/stubs");
        let rt = InferenceRuntime::new(Box::new(CpuBackend::new()))
            .with_loader(Box::new(StubModelLoader));

        for kind in ModalityKind::ALL {
            let stub_name = format!("{}.stub", kind.id_str());
            ctx.phase(format!("infer_{}", kind.id_str()));
            let path = stubs_dir.join(&stub_name);
            ctx.assert(
                path.is_file(),
                format!("missing stub file: {}", path.display()),
            )?;

            let model = rt
                .load_model(&path)
                .map_err(|e| E2eError::Failed(format!("{}: {e}", kind.story_id())))?;
            let inputs = sample_input(kind)
                .map_err(|e| E2eError::Failed(format!("{} sample: {e}", kind.story_id())))?;
            let mut session = rt.create_session();
            let outcome = rt
                .infer(
                    model.as_ref(),
                    &inputs,
                    &mut session,
                    ExecutionRecorder::new(format!("e2e-{}", kind.story_id())),
                )
                .map_err(|e| E2eError::Failed(format!("{} infer: {e}", kind.story_id())))?;

            ctx.assert(
                !outcome.outputs.is_empty(),
                format!("{}: expected non-empty outputs", kind.story_id()),
            )?;
            // Panics on contract failure — same assert as acceptance us_010_019_modalities.
            assert_stub_modality_telemetry(&outcome.trace, kind);
            ctx.record(format!("modality_ok:{}:{}", kind.story_id(), kind.id_str()));
        }

        ctx.record(format!("modalities_passed:{}", ModalityKind::ALL.len()));
        Ok(())
    }
}
