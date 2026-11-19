//! CI vs live **acceptance profile** (replaces `if live` gates).

use std::sync::Arc;

/// Like pyiv test doubles vs production: one binding per process.
pub trait AcceptanceProfile: Send + Sync {
    fn id(&self) -> &'static str;

    /// When false, live integration tests record `live_skipped_ci` and return.
    fn run_live_integration(&self) -> bool;

    fn skip_live_test(&self, test_name: &str) -> bool {
        if self.run_live_integration() {
            return false;
        }
        tracing::info!(
            target: "uie_iv",
            test = test_name,
            profile = self.id(),
            "skip live test (CI profile)"
        );
        true
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CiAcceptanceProfile;

impl AcceptanceProfile for CiAcceptanceProfile {
    fn id(&self) -> &'static str {
        "ci"
    }

    fn run_live_integration(&self) -> bool {
        false
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct LiveAcceptanceProfile;

impl AcceptanceProfile for LiveAcceptanceProfile {
    fn id(&self) -> &'static str {
        "live"
    }

    fn run_live_integration(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcceptanceMode {
    Ci,
    Live,
}

/// Reads `UIE_ACCEPTANCE_MODE` once when building the injector.
pub fn acceptance_mode_from_env() -> AcceptanceMode {
    match std::env::var("UIE_ACCEPTANCE_MODE")
        .unwrap_or_else(|_| "ci".into())
        .to_ascii_lowercase()
        .as_str()
    {
        "live" | "real" | "local" => AcceptanceMode::Live,
        _ => AcceptanceMode::Ci,
    }
}

pub fn profile_for_mode(mode: AcceptanceMode) -> Arc<dyn AcceptanceProfile> {
    match mode {
        AcceptanceMode::Ci => Arc::new(CiAcceptanceProfile),
        AcceptanceMode::Live => Arc::new(LiveAcceptanceProfile),
    }
}

/// Serve / daemon profile from `UIE_SERVE_PROFILE` (`stub` default, `live` adds optional loaders).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServeLoaderMode {
    StubOnly,
    Live,
}

pub fn serve_loader_mode_from_env() -> ServeLoaderMode {
    match std::env::var("UIE_SERVE_PROFILE")
        .unwrap_or_else(|_| "stub".into())
        .to_ascii_lowercase()
        .as_str()
    {
        "live" | "all" => ServeLoaderMode::Live,
        _ => ServeLoaderMode::StubOnly,
    }
}
