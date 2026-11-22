//! Load on-disk `.safetensors` through Spiro (mmap + executor contract).
//!
//! **Phase 1:** open weights, bind to modality I/O, run contract infer (same tensor shapes as stubs).
//! Full Qwen-Image denoise is US-012 follow-on; this path proves registry → loader → runtime.

mod executor;
mod loader;
mod role;

pub use executor::SafetensorsExecutor;
pub use loader::SafetensorsModelLoader;
pub use role::{guess_from_path, modality_for_catalog_entry};
