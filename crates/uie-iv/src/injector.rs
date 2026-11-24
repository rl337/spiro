//! Built object graph (pyiv ``Injector``).

use std::sync::Arc;

use uie_core::model::ModelLoader;
use uie_core::resource::{
    InferenceStateBudget, MemoryBudget, UnlimitedInferenceStateBudget, UnlimitedMemoryBudget,
};

use crate::binder::Binder;
use crate::error::IvError;
use crate::module::{install_modules, Module};
use crate::uie::loaders::ModelLoaderSet;
use crate::uie::profile::AcceptanceProfile;

#[derive(Clone)]
pub struct Injector {
    acceptance_profile: Arc<dyn AcceptanceProfile>,
    loader_set: Arc<dyn ModelLoaderSet>,
    memory_budget: Arc<dyn MemoryBudget>,
    inference_state_budget: Arc<dyn InferenceStateBudget>,
}

impl Injector {
    pub fn from_modules(modules: &[&dyn Module]) -> Result<Self, IvError> {
        let mut binder = Binder::default();
        install_modules(modules, &mut binder);
        Self::from_binder(binder)
    }

    pub fn from_binder(binder: Binder) -> Result<Self, IvError> {
        Ok(Self {
            acceptance_profile: binder
                .acceptance_profile
                .ok_or(IvError::Unbound("acceptance_profile"))?,
            loader_set: binder.loader_set.ok_or(IvError::Unbound("loader_set"))?,
            memory_budget: binder
                .memory_budget
                .unwrap_or_else(|| Arc::new(UnlimitedMemoryBudget)),
            inference_state_budget: binder
                .inference_state_budget
                .unwrap_or_else(|| Arc::new(UnlimitedInferenceStateBudget)),
        })
    }

    pub fn acceptance_profile(&self) -> &dyn AcceptanceProfile {
        self.acceptance_profile.as_ref()
    }

    pub fn loaders(&self) -> Vec<Box<dyn ModelLoader>> {
        self.loader_set.loaders()
    }

    /// Injected memory budget (US-025); defaults to unlimited when unbound.
    pub fn memory_budget(&self) -> Arc<dyn MemoryBudget> {
        Arc::clone(&self.memory_budget)
    }

    /// Injected KV / context budget (US-027); defaults to unlimited when unbound.
    pub fn inference_state_budget(&self) -> Arc<dyn InferenceStateBudget> {
        Arc::clone(&self.inference_state_budget)
    }
}
