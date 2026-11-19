//! # uie-iv
//!
//! [pyiv](https://github.com/rl337/pyiv)-style dependency injection for UIE: **modules** install
//! bindings; an **injector** resolves the CI vs live object graph without `if test` at call sites.
//!
//! ```ignore
//! use uie_iv::{Injector, uie::{AcceptanceEnvModule, ServeEnvModule}};
//!
//! let inj = Injector::from_modules(&[&AcceptanceEnvModule])?;
//! if inj.acceptance_profile().skip_live_test("live_us_010") { return; }
//! let host = InferenceEngineHost::new(inj.loaders());
//! ```

pub mod binder;
pub mod error;
pub mod injector;
pub mod key;
pub mod module;
pub mod uie;

pub use binder::Binder;
pub use error::IvError;
pub use injector::Injector;
pub use key::{Key, Named};
pub use module::{install_modules, Module};

pub fn acceptance_injector() -> Result<Injector, IvError> {
    Injector::from_modules(&[&uie::modules::AcceptanceEnvModule])
}

pub fn serve_injector() -> Result<Injector, IvError> {
    Injector::from_modules(&[&uie::modules::ServeEnvModule])
}

pub use uie::profile::serve_loader_mode_from_env;