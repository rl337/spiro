//! Built-in modules (pyiv ``Config`` subclasses).

use std::sync::Arc;

use crate::binder::Binder;
use crate::module::Module;
use crate::uie::loaders::{loader_set_for_serve, LiveLoaderSet, StubOnlyLoaderSet};
use uie_core::accelerator::{AcceleratorKind, FakeAcceleratorProbe};
use uie_core::resource::{
    FakeInferenceStateBudget, FakeMemoryBudget, UnlimitedInferenceStateBudget,
    UnlimitedMemoryBudget,
};
use uie_embeddings::StubTextEmbedder;

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
        // Unlimited until a live cgroup provider ships; tests override with FakeMemoryBudget.
        let _ = binder.bind_memory_budget(Arc::new(UnlimitedMemoryBudget));
        let _ = binder.bind_inference_state_budget(Arc::new(UnlimitedInferenceStateBudget));
        let _ = binder.bind_accelerator_probe(FakeAcceleratorProbe::empty());
        let _ = binder.bind_text_embedder(Arc::new(StubTextEmbedder::with_defaults()));
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
        let _ = binder.bind_memory_budget(Arc::new(UnlimitedMemoryBudget));
        let _ = binder.bind_inference_state_budget(Arc::new(UnlimitedInferenceStateBudget));
        let _ = binder.bind_accelerator_probe(FakeAcceleratorProbe::empty());
        let _ = binder.bind_text_embedder(Arc::new(StubTextEmbedder::with_defaults()));
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
        binder.override_text_embedder(Arc::new(StubTextEmbedder::with_defaults()));
    }
}

/// Test override: fixed-capacity fake memory budget (US-025).
pub struct OverrideFakeMemoryBudgetModule {
    pub capacity: u64,
}

impl Module for OverrideFakeMemoryBudgetModule {
    fn install(&self, binder: &mut Binder) {
        binder.override_memory_budget(FakeMemoryBudget::with_capacity(self.capacity));
    }
}

/// Test override: fixed context-token fake state budget (US-027).
pub struct OverrideFakeInferenceStateBudgetModule {
    pub max_context_tokens: u32,
    pub allow_negotiate: bool,
}

impl Module for OverrideFakeInferenceStateBudgetModule {
    fn install(&self, binder: &mut Binder) {
        let budget = if self.allow_negotiate {
            FakeInferenceStateBudget::with_negotiate(self.max_context_tokens)
        } else {
            FakeInferenceStateBudget::with_max_context(self.max_context_tokens)
        };
        binder.override_inference_state_budget(budget);
    }
}

/// Test override: fake accelerator presence (US-030 / US-031).
pub struct OverrideFakeAcceleratorProbeModule {
    pub kinds: Vec<AcceleratorKind>,
}

impl Module for OverrideFakeAcceleratorProbeModule {
    fn install(&self, binder: &mut Binder) {
        binder.override_accelerator_probe(FakeAcceleratorProbe::with_kinds(self.kinds.clone()));
    }
}
