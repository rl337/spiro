//! US-007
mod support;

use uie_core::backend::cpu::CpuBackend;
use uie_core::InferenceRuntime;
use uie_stub_models::StubModelLoader;

#[test]
fn us_007_stub_loader_loads_executor() {
    let rt = InferenceRuntime::new(Box::new(CpuBackend::new()))
        .with_loader(Box::new(StubModelLoader::default()));
    let path = support::stubs_dir().join("llm.stub");
    let model = rt.load_model(&path).expect("load stub");
    assert!(!model.inputs().is_empty());
}