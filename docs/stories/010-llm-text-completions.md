# US-010: Large language models and text completions

**Status:** partial (GGUF / LM Studio paths; GPU backends later)  
**Personas:** Liz  
**Crate / path:** `crates/uie-loader-gguf`, `crates/uie-llm`, `config/model_roots.yaml`

## User story

**As** Liz, building a chatbot on top of the same engine that runs Spiro and MNIST  
**I want** to load a large language model and generate **text completions** from a **prompt**  
**So that** my bot replies in natural language through UIE—not a separate Python stack.

## In plain terms

Liz’s **concrete example:** a small chatbot CLI that:

1. Creates an [inference runtime](../glossary.md#inference-runtime) with a future LLM [model loader](./007-models-loaders.md).
2. Opens a [session](./004-runtime-sessions.md) and binds the user message as a text **prompt** [tensor](./002-tensors.md) (or a token sequence—see [token](../glossary.md#token)).
3. Calls `infer` on a **[large language model](../glossary.md#large-language-model)** to get the next **[completion](../glossary.md#completion)** (one turn of assistant text).
4. Appends the completion to chat history and repeats.

This is **text → text** [modality](./015-modality-catalog.md). It reuses the same path Jack wants for any [inference executor](../glossary.md#inference-executor); only the loader and I/O specs change.

## Related stories

- [US-001](./001-why-uie.md) — one engine for demos and models
- [US-004](./004-runtime-sessions.md) — session bindings for `prompt` / `completion`
- [US-007](./007-models-loaders.md) — pluggable loaders
- [US-015](./015-modality-catalog.md) — modality map
- [US-017](./017-multimodal-text-image-to-text.md) — chat with images later

## Terms used

- [Large language model](../glossary.md#large-language-model)
- [Completion](../glossary.md#completion)
- [Prompt](../glossary.md#prompt)
- [Chatbot](../glossary.md#chatbot)
- [Token](../glossary.md#token)
- [Modality](../glossary.md#modality)
- [Inference runtime](../glossary.md#inference-runtime)

## Acceptance notes

- **CI:** stub contract `us_010_llm_stub` (unchanged).
- **Live (LM Studio GGUF):**
  ```bash
  ./scripts/build-gguf.sh
  uie-llm list-gguf
  UIE_GGUF_MODEL=~/.lmstudio/models/.../model.gguf \
    cargo run -p uie-llm -- --prompt "The capital of France is"
  ```
- Registry id: `cargo run -p uie-llm -- --registry-id 'lmstudio:...' --prompt "Hello"`
- **Dependencies:** llama.cpp via `llama-cpp-2`; Metal/CUDA via crate features on a laptop with GPU.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_010_019_modalities.rs` | `cargo test -p uie-acceptance --test us_010_019_modalities us_010_llm_stub` |
| Rust (live) | `crates/uie-acceptance/tests/live_models.rs` | `UIE_ACCEPTANCE_MODE=live cargo test -p uie-acceptance --features gguf live_us_010_gguf_text_completion` |

## Observability

- **Trace:** `assert_stub_executor_telemetry` (`stub_validate_inputs`, `stub_policy_infer`, `runtime_session_bind`)
- **CI test:** `us_010_019_modalities.rs` → `us_010_llm_stub`
- **Live:** `live_us_010_gguf_text_completion` asserts `CompleteResult.metrics`
