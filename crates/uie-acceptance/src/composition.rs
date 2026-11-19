//! Composition root for acceptance tests (pyiv `get_injector`).

use std::sync::OnceLock;

use uie_iv::Injector;

fn build() -> Injector {
    uie_iv::acceptance_injector().expect("acceptance DI graph")
}

/// Process-wide injector from [`uie_iv::AcceptanceEnvModule`].
pub fn injector() -> &'static Injector {
    static INJ: OnceLock<Injector> = OnceLock::new();
    INJ.get_or_init(build)
}

/// Build an [`InferenceRuntime`] from the injected loader multibinder.
pub fn inference_runtime() -> uie_core::InferenceRuntime {
    use uie_core::backend::cpu::CpuBackend;
    use uie_core::InferenceRuntime;

    let mut rt = InferenceRuntime::new(Box::new(CpuBackend::new()));
    for loader in injector().loaders() {
        rt.register_loader(loader);
    }
    rt
}

/// Loader format ids present in the active profile (e.g. `stub`, `onnx`).
pub fn loader_format_ids() -> Vec<String> {
    injector()
        .loaders()
        .into_iter()
        .map(|l| l.format_id().to_string())
        .collect()
}