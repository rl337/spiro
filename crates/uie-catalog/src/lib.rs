//! Directory-only [**model roots**](./roots.rs) and a [**model registry**](./registry.rs) built by scanning at runtime.

pub mod comfy;
pub mod detector;
pub mod executor;
mod lmstudio;
pub mod mock;
mod registry;
mod roots;
mod scan;

pub use comfy::{stack_candidates, ComfyStackRef};
pub use detector::{DirectoryModelDetector, ModelDetector, TestModelDetector};
pub use registry::{ModelCatalog, ModelEntry, ModelFormat, ModelRegistry, ModelRole, ScanStats};
pub use roots::{expand_home, ModelRootConfig, ModelRootKind, ModelRootsFile};
pub use scan::scan_roots;
