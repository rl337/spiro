use crate::harness::E2eTest;
use crate::scenarios::mock_registry::MockRegistryDiscovery;
use crate::scenarios::modality_live_smoke::ModalityLiveSmoke;

#[cfg(feature = "gguf")]
use crate::scenarios::qwen3_2b::Qwen3_2bChatCompletion;

pub fn all_tests() -> Vec<Box<dyn E2eTest>> {
    vec![
        Box::new(MockRegistryDiscovery),
        Box::new(ModalityLiveSmoke),
        #[cfg(feature = "gguf")]
        Box::new(Qwen3_2bChatCompletion),
    ]
}