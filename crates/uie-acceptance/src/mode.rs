//! CI (stub) vs live acceptance modes — resolved via [`crate::composition`].

pub use uie_iv::uie::profile::{
    acceptance_mode_from_env, AcceptanceMode, AcceptanceProfile, CiAcceptanceProfile,
    LiveAcceptanceProfile,
};

/// Reads `UIE_ACCEPTANCE_MODE`. Unset or `ci`/`stub` → [`AcceptanceMode::Ci`].
pub fn acceptance_mode() -> AcceptanceMode {
    acceptance_mode_from_env()
}

pub fn is_ci_mode() -> bool {
    matches!(acceptance_mode(), AcceptanceMode::Ci)
}

pub fn is_live_mode() -> bool {
    matches!(acceptance_mode(), AcceptanceMode::Live)
}
