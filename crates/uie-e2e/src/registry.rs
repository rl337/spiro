use crate::harness::E2eTest;

#[cfg(feature = "gguf")]
use crate::scenarios::qwen3_2b::Qwen3_2bChatCompletion;

pub fn all_tests() -> Vec<Box<dyn E2eTest>> {
    #[cfg(feature = "gguf")]
    {
        vec![Box::new(Qwen3_2bChatCompletion)]
    }
    #[cfg(not(feature = "gguf"))]
    {
        Vec::new()
    }
}
