//! CI (stub) vs live acceptance modes.

/// `ci` / `stub` — default in automation: stub models only.
/// `live` — load and run real weights from disk or registry (laptop / nightly).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcceptanceMode {
    Ci,
    Live,
}

/// Reads `UIE_ACCEPTANCE_MODE`. Unset or `ci`/`stub` → [`AcceptanceMode::Ci`].
pub fn acceptance_mode() -> AcceptanceMode {
    match std::env::var("UIE_ACCEPTANCE_MODE")
        .unwrap_or_else(|_| "ci".into())
        .to_ascii_lowercase()
        .as_str()
    {
        "live" | "real" | "local" => AcceptanceMode::Live,
        _ => AcceptanceMode::Ci,
    }
}

pub fn is_ci_mode() -> bool {
    matches!(acceptance_mode(), AcceptanceMode::Ci)
}

pub fn is_live_mode() -> bool {
    matches!(acceptance_mode(), AcceptanceMode::Live)
}