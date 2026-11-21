use crate::harness::E2eTest;
use crate::scenarios::mock_registry::MockRegistryDiscovery;

#[cfg(feature = "gguf")]
use crate::scenarios::qwen3_2b::Qwen3_2bChatCompletion;

pub fn all_tests() -> Vec<Box<dyn E2eTest>> {
    #[cfg(feature = "gguf")]
    {
        return vec![
            Box::new(MockRegistryDiscovery),
            Box::new(Qwen3_2bChatCompletion),
        ];
    }
    vec![Box::new(MockRegistryDiscovery)]
}
