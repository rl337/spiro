//! Bridge registry entries to [`InferenceRuntime::load_model`].

use uie_core::model::InferenceExecutor;
use uie_core::InferenceRuntime;
use uie_core::Result;

use crate::registry::ModelEntry;

/// Load the executor for a catalog entry using the runtime's registered loaders.
///
/// Stub (`.stub`) paths resolve to [`StubModelLoader`](../../uie-stub-models) when registered.
pub fn load_executor_for_entry(
    runtime: &InferenceRuntime,
    entry: &ModelEntry,
) -> Result<Box<dyn InferenceExecutor>> {
    runtime.load_model(&entry.path)
}
