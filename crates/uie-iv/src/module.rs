//! ``Module`` installs bindings (pyiv ``Config.configure``).

use crate::binder::Binder;

/// Contributes bindings into a shared injector graph.
pub trait Module: Send + Sync {
    fn install(&self, binder: &mut Binder);
}

/// Install several modules in order (later overrides allowed via binder policy).
pub fn install_modules(modules: &[&dyn Module], binder: &mut Binder) {
    for m in modules {
        m.install(binder);
    }
}