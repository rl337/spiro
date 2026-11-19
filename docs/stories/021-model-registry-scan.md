# US-021: Model roots and runtime registry scan

**Status:** shipped (scan + Comfy family normalization; live scan optional)  
**Personas:** Jack, Liz  
**Crate / path:** `crates/uie-catalog`, `config/model_roots.example.yaml`

## User story

**As** Jack, with weights already under LM Studio and ComfyUI—and **as** Liz, wiring Spiro  
**I want** config to list **directories only** and the engine to **scan at runtime** into a model registry  
**So that** we do not duplicate paths or maintain two catalogs by hand.

## In plain terms

1. `model_roots.yaml` lists roots (`id`, `path`, `kind`: `lm_studio` | `comfy_ui` | `generic`).
2. `ModelRegistry::refresh` walks each root, classifies `.gguf`, `.safetensors`, `.onnx`, `.stub`, etc.
3. LM Studio layout yields `ModelRole::Llm`; Comfy folders map to `checkpoints`, `vae`, `text_encoders`, …
4. Each file gets a stable id: `{root_id}:{relative/path}`.
5. Spiro / `uie-serve` resolve `registry_id` → absolute path for [loaders](./007-models-loaders.md).

## Related stories

- [US-007 — Loaders](./007-models-loaders.md)
- [US-020 — Byte channel](./020-inference-byte-channel.md)
- [US-022 — Multi-hop pipeline](./022-byte-channel-pipeline.md)
- [US-010](./010-llm-text-completions.md) — LM Studio GGUF targets
- [US-012](./012-diffusion-text-to-image.md) — Comfy checkpoints

## Terms used

- [Model loader](../glossary.md#model-loader)
- [Inference executor](../glossary.md#inference-executor)

## Acceptance notes

- `cargo test -p uie-catalog`
- `cargo test -p uie-acceptance --test us_021_model_registry`

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_021_model_registry.rs` | `cargo test -p uie-acceptance --test us_021_model_registry` |
| Rust | `crates/uie-catalog/tests/scan.rs` | `cargo test -p uie-catalog` |
| Rust (live) | `crates/uie-acceptance/tests/live_models.rs` | `UIE_ACCEPTANCE_MODE=live cargo test -p uie-acceptance --test live_models live_us_021` |

## Observability

- **Events:** `registry_scan_entry`, `registry_entry_id_prefix`; crate test adds `registry_scan_multi_layout`
- **CI:** `us_021_model_registry.rs`, `uie-catalog/tests/scan.rs`
- **Live:** `live_us_021_*` records `registry_live_scan` events (skipped in CI)
