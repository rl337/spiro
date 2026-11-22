//! US-016/017: Qwen VL chat-style caption on a known cat image (mmproj + chat template).

use crate::catalog::catalog_for_e2e;
use crate::harness::{E2eContext, E2eError, E2eTest};

#[cfg(feature = "gguf")]
use crate::vl_helpers::describe_with_vl;

#[derive(Debug, Default, Clone, Copy)]
pub struct VlCatChat;

impl E2eTest for VlCatChat {
    fn name(&self) -> &'static str {
        "vl_cat_chat"
    }

    fn description(&self) -> &'static str {
        "Qwen VL chat: open-ended Q&A on fixtures/images/cat_512.png (conversation-style prompt)."
    }

    fn run(&self, ctx: &mut E2eContext, args: &[String]) -> Result<(), E2eError> {
        let catalog =
            catalog_for_e2e(&ctx.repo_root).map_err(|e| E2eError::Failed(e.to_string()))?;

        let cat_png = ctx.repo_root.join("fixtures/images/cat_512.png");
        ctx.phase("vl_chat_load_image");
        ctx.record(format!("image:{}", cat_png.display()));
        ctx.assert(
            cat_png.is_file(),
            format!("missing cat fixture: {}", cat_png.display()),
        )?;

        if args.iter().any(|a| a == "--skip-llm") {
            ctx.record("skipped_vl_chat");
            return Ok(());
        }

        #[cfg(feature = "gguf")]
        {
            ctx.phase("vl_chat_completion");
            let default_prompt = "Look at this image and chat with me: what animal do you see, \
                what color is it, and what is it doing? Answer in two short sentences like a helpful assistant.";
            let out = describe_with_vl(&catalog, &cat_png, args, default_prompt, 96, 0.3)?;
            let text = out.text.trim();
            eprintln!("vl_chat_reply: {text}");
            ctx.record(format!("vl_chat_reply:{text}"));
            ctx.record("vl_chat_completion_metrics");

            let lower = text.to_ascii_lowercase();
            ctx.assert(!text.is_empty(), "empty vl chat reply")?;
            ctx.assert(
                text.chars().filter(|c| c.is_alphanumeric()).count() >= 12,
                "vl chat reply too short",
            )?;
            ctx.assert(
                lower.contains("cat") || lower.contains("feline") || lower.contains("kitten"),
                format!("expected animal mention in chat reply, got: {text}"),
            )?;
            ctx.assert(
                out.metrics.completion_tokens > 0 || !out.token_ids.is_empty(),
                "vl chat produced no tokens",
            )?;
        }
        #[cfg(not(feature = "gguf"))]
        {
            return Err(E2eError::Failed(
                "vl_cat_chat requires --features gguf (or spiro-chain)".into(),
            ));
        }

        Ok(())
    }
}
