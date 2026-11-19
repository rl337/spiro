//! Built object graph (pyiv ``Injector``).

use std::sync::Arc;

use uie_core::model::ModelLoader;

use crate::binder::Binder;
use crate::error::IvError;
use crate::module::{install_modules, Module};
use crate::uie::loaders::ModelLoaderSet;
use crate::uie::profile::AcceptanceProfile;

#[derive(Clone)]
pub struct Injector {
    acceptance_profile: Arc<dyn AcceptanceProfile>,
    loader_set: Arc<dyn ModelLoaderSet>,
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
        })
    }

    pub fn acceptance_profile(&self) -> &dyn AcceptanceProfile {
        self.acceptance_profile.as_ref()
    }

    pub fn loaders(&self) -> Vec<Box<dyn ModelLoader>> {
        self.loader_set.loaders()
    }
}
