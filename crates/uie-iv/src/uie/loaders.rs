//! Model loader sets — stub vs live wired through DI, not `#[cfg]` in tests.

use uie_core::model::ModelLoader;
use uie_stub_models::StubModelLoader;

use crate::uie::profile::ServeLoaderMode;

pub trait ModelLoaderSet: Send + Sync {
    fn id(&self) -> &'static str;
    fn loaders(&self) -> Vec<Box<dyn ModelLoader>>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct StubOnlyLoaderSet;

impl ModelLoaderSet for StubOnlyLoaderSet {
    fn id(&self) -> &'static str {
        "stub-only"
    }

    fn loaders(&self) -> Vec<Box<dyn ModelLoader>> {
        vec![Box::new(StubModelLoader)]
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct LiveLoaderSet;

impl ModelLoaderSet for LiveLoaderSet {
    fn id(&self) -> &'static str {
        "live-chain"
    }

    fn loaders(&self) -> Vec<Box<dyn ModelLoader>> {
        #[allow(unused_mut)]
        let mut chain: Vec<Box<dyn ModelLoader>> = vec![Box::new(StubModelLoader)];
        #[cfg(feature = "live-onnx")]
        {
            chain.push(Box::new(uie_loader_onnx::OnnxModelLoader::new()));
        }
        #[cfg(feature = "gguf")]
        {
            chain.push(Box::new(uie_loader_gguf::GgufModelLoader::default()));
        }
        #[cfg(feature = "safetensors")]
        {
            chain.push(Box::new(uie_loader_safetensors::SafetensorsModelLoader));
        }
        chain
    }
}

pub fn loader_set_for_serve(mode: ServeLoaderMode) -> Box<dyn ModelLoaderSet> {
    match mode {
        ServeLoaderMode::StubOnly => Box::new(StubOnlyLoaderSet),
        ServeLoaderMode::Live => Box::new(LiveLoaderSet),
    }
}
