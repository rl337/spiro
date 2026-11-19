# US-017: Multimodal chat (text + image → text)

**Status:** planned  
**Personas:** Liz  
**Crate / path:** future `uie-vlm` (not shipped)

## User story

**As** Liz, extending her chatbot from [US-010](./010-llm-text-completions.md)  
**I want** to send both a **prompt** and an image into one model  
**So that** users can ask “what’s in this picture?” inside the same UIE **session**.

## In plain terms

**Text + image → text** ([multimodal](../glossary.md#multimodal)):

1. Bind `prompt` text and `image` [tensor](./002-tensors.md) in one [session](./004-runtime-sessions.md).
2. Vision-language [inference executor](../glossary.md#inference-executor) consumes both [I/O specs](../glossary.md#io-spec).
3. Output is a text **completion** like US-010.

Combines [US-016](./016-image-to-text.md) encoding with [US-010](./010-llm-text-completions.md) dialogue patterns. Optional [US-011](./011-judge-system-one-models.md) on the answer.

## Related stories

- [US-010](./010-llm-text-completions.md) — chatbot base case
- [US-016](./016-image-to-text.md)
- [US-004](./004-runtime-sessions.md)
- [US-015](./015-modality-catalog.md)

## Terms used

- [Multimodal](../glossary.md#multimodal)
- [Prompt](../glossary.md#prompt)
- [Large language model](../glossary.md#large-language-model)
- [Modality](../glossary.md#modality)

## Acceptance notes

- **Today:** story only.
- **Future acceptance:** two-input `infer` test with fixture image + question string.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_010_019_modalities.rs` | `cargo test -p uie-acceptance --test us_010_019_modalities us_017_vlm_stub` |
