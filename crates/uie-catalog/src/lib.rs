//! Directory-only [**model roots**](./roots.rs) and a [**model registry**](./registry.rs) built by scanning at runtime.

pub mod comfy;
mod lmstudio;
mod registry;
mod roots;
mod scan;

pub use comfy::{stack_candidates, ComfyStackRef};
pub use registry::{ModelCatalog, ModelEntry, ModelFormat, ModelRegistry, ModelRole, ScanStats};
pub use roots::{expand_home, ModelRootConfig, ModelRootKind, ModelRootsFile};
pub use scan::scan_roots;
