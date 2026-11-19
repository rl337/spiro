# US-016: Image-to-text (vision captioning)

**Status:** planned  
**Personas:** Frank  
**Crate / path:** future `uie-vision` (not shipped)

## User story

**As** Frank, who trusts pictures more than prompts  
**I want** a model that takes an image and returns a text description  
**So that** UIE supports **image-to-text** for captioning, alt text, and QA.

## In plain terms

**Image → text**:

1. Load PNG/JPEG bytes into an image [tensor](./002-tensors.md) (like [US-008](./008-spiro-demo.md) output).
2. Vision encoder + decoder (CLIP+LM head, ViT classifier with label map, etc.) runs through `infer`.
3. Text lands in the [session](./004-runtime-sessions.md) for downstream [US-010](./010-llm-text-completions.md) or logging.

Differs from [US-009](./009-onnx-mnist.md): output is language, not a single digit class.

## Related stories

- [US-002](./002-tensors.md)
- [US-012](./012-diffusion-text-to-image.md) — opposite direction
- [US-017](./017-multimodal-text-image-to-text.md)
- [US-015](./015-modality-catalog.md)

## Terms used

- [Image-to-text](../glossary.md#image-to-text)
- [Modality](../glossary.md#modality)
- [Tensor](../glossary.md#tensor)

## Acceptance notes

- **Today:** story only.
- **Future acceptance:** fixture image → caption contains expected keyword in test.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_010_019_modalities.rs` | `cargo test -p uie-acceptance --test us_010_019_modalities us_016_image_to_text_stub` |

## Observability

- **Trace:** stub executor phases via `us_016_image_to_text_stub`
- **CI test:** `us_010_019_modalities.rs`
