use crate::harness::E2eTest;
use crate::scenarios::mock_registry::MockRegistryDiscovery;
use crate::scenarios::modality_live_smoke::ModalityLiveSmoke;

#[cfg(feature = "gguf")]
use crate::scenarios::cat_t2i_vl_yes::CatT2iVlYes;
#[cfg(feature = "gguf")]
use crate::scenarios::qwen3_2b::Qwen3_2bChatCompletion;
#[cfg(feature = "safetensors")]
use crate::scenarios::spiro_native_chain::SpiroNativeQwenChain;
#[cfg(feature = "gguf")]
use crate::scenarios::vl_cat_chat::VlCatChat;

pub fn all_tests() -> Vec<Box<dyn E2eTest>> {
    #[cfg_attr(not(any(feature = "safetensors", feature = "gguf")), allow(unused_mut))]
    let mut v: Vec<Box<dyn E2eTest>> =
        vec![Box::new(MockRegistryDiscovery), Box::new(ModalityLiveSmoke)];
    #[cfg(feature = "gguf")]
    {
        v.push(Box::new(CatT2iVlYes));
        v.push(Box::new(VlCatChat));
    }
    #[cfg(feature = "safetensors")]
    v.push(Box::new(SpiroNativeQwenChain));
    #[cfg(feature = "gguf")]
    v.push(Box::new(Qwen3_2bChatCompletion));
    v
}
