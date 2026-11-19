//! CI (stub) vs live acceptance modes — resolved via the composition root ([`crate::composition`]).

pub use uie_iv::uie::profile::{
    AcceptanceMode, AcceptanceProfile, CiAcceptanceProfile, LiveAcceptanceProfile,
};

/// Active acceptance mode from the injected profile (not a direct env read).
pub fn acceptance_mode() -> AcceptanceMode {
    match crate::composition::injector().acceptance_profile().id() {
        "live" => AcceptanceMode::Live,
        _ => AcceptanceMode::Ci,
    }
}

pub fn is_ci_mode() -> bool {
    matches!(acceptance_mode(), AcceptanceMode::Ci)
}

pub fn is_live_mode() -> bool {
    matches!(acceptance_mode(), AcceptanceMode::Live)
}
