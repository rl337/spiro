//! Registration surface (pyiv ``Binder`` / ``Config.register``).

use std::sync::Arc;

use uie_core::resource::MemoryBudget;

use crate::error::IvError;
use crate::uie::loaders::ModelLoaderSet;
use crate::uie::profile::AcceptanceProfile;

#[derive(Default)]
pub struct Binder {
    pub(crate) acceptance_profile: Option<Arc<dyn AcceptanceProfile>>,
    pub(crate) loader_set: Option<Arc<dyn ModelLoaderSet>>,
    pub(crate) memory_budget: Option<Arc<dyn MemoryBudget>>,
}

impl Binder {
    pub fn bind_acceptance_profile(
        &mut self,
        profile: Arc<dyn AcceptanceProfile>,
    ) -> Result<(), IvError> {
        if self.acceptance_profile.is_some() {
            return Err(IvError::Duplicate("acceptance_profile"));
        }
        self.acceptance_profile = Some(profile);
        Ok(())
    }

    pub fn bind_loader_set(&mut self, set: Arc<dyn ModelLoaderSet>) -> Result<(), IvError> {
        if self.loader_set.is_some() {
            return Err(IvError::Duplicate("loader_set"));
        }
        self.loader_set = Some(set);
        Ok(())
    }

    pub fn bind_memory_budget(&mut self, budget: Arc<dyn MemoryBudget>) -> Result<(), IvError> {
        if self.memory_budget.is_some() {
            return Err(IvError::Duplicate("memory_budget"));
        }
        self.memory_budget = Some(budget);
        Ok(())
    }

    pub fn override_acceptance_profile(&mut self, profile: Arc<dyn AcceptanceProfile>) {
        self.acceptance_profile = Some(profile);
    }

    pub fn override_loader_set(&mut self, set: Arc<dyn ModelLoaderSet>) {
        self.loader_set = Some(set);
    }

    pub fn override_memory_budget(&mut self, budget: Arc<dyn MemoryBudget>) {
        self.memory_budget = Some(budget);
    }
}
