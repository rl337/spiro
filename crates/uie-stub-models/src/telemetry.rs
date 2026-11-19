//! Required telemetry phase names for stub executors (US-023 acceptance).

use crate::modality::ModalityKind;

pub const PHASE_VALIDATE_INPUTS: &str = "stub_validate_inputs";
pub const PHASE_POLICY_INFER: &str = "stub_policy_infer";
pub const PHASE_MODALITY_PREFIX: &str = "stub_modality_";

/// Opening phase tagging the stub modality (e.g. `stub_modality_llm`).
pub fn phase_modality(kind: ModalityKind) -> String {
    format!("{PHASE_MODALITY_PREFIX}{}", kind.id_str())
}
