//! Built-in modules (pyiv ``Config`` subclasses).

use std::sync::Arc;

use crate::binder::Binder;
use crate::module::Module;
use crate::uie::loaders::{loader_set_for_serve, LiveLoaderSet, StubOnlyLoaderSet};
use crate::uie::profile::{
    acceptance_mode_from_env, profile_for_mode, serve_loader_mode_from_env, AcceptanceMode,
};

/// Binds CI vs live from `UIE_ACCEPTANCE_MODE` + matching loader chain.
pub struct AcceptanceEnvModule;

impl Module for AcceptanceEnvModule {
    fn install(&self, binder: &mut Binder) {
        let mode = acceptance_mode_from_env();
        let _ = binder.bind_acceptance_profile(profile_for_mode(mode));
        let set: Arc<dyn crate::uie::loaders::ModelLoaderSet> = match mode {
            AcceptanceMode::Ci => Arc::new(StubOnlyLoaderSet),
            AcceptanceMode::Live => Arc::new(LiveLoaderSet),
        };
        let _ = binder.bind_loader_set(set);
    }
}

/// Binds `uie-serve` loaders from `UIE_SERVE_PROFILE`.
pub struct ServeEnvModule;

impl Module for ServeEnvModule {
    fn install(&self, binder: &mut Binder) {
        let mode = serve_loader_mode_from_env();
        let set = loader_set_for_serve(mode);
        let _ = binder.bind_loader_set(Arc::from(set));
        // Default profile for serve is CI unless acceptance mode overrides in combined graphs.
        let _ = binder.bind_acceptance_profile(profile_for_mode(AcceptanceMode::Ci));
    }
}

/// Test override: force live profile without mutating env.
pub struct OverrideLiveModule;

impl Module for OverrideLiveModule {
    fn install(&self, binder: &mut Binder) {
        binder.override_acceptance_profile(profile_for_mode(AcceptanceMode::Live));
        binder.override_loader_set(Arc::new(LiveLoaderSet));
    }
}

/// Test override: force CI stub profile.
pub struct OverrideCiModule;

impl Module for OverrideCiModule {
    fn install(&self, binder: &mut Binder) {
        binder.override_acceptance_profile(profile_for_mode(AcceptanceMode::Ci));
        binder.override_loader_set(Arc::new(StubOnlyLoaderSet));
    }
}