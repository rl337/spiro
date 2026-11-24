# US-027: Bounded inference state

**Status:** partial (CI fake context budget; GGUF live wiring later)  
**Personas:** Liz, Pete  
**Crate / path:** `crates/uie-core` (`InferenceStateBudget`), `uie-loader-gguf` (`CompleteOptions.context_tokens` / `n_ctx`)  
**Inspiration:** EDGE-STATE-02 in [`docs/inspiration/trouble-edge-inference-2026-11-24.md`](../inspiration/trouble-edge-inference-2026-11-24.md)

## User story

**As a** Liz  
**I want** a declared context and [inference state](../glossary.md#inference-state) budget  
**So that** long conversations cannot silently exhaust a small container after a model is admitted.

## In plain terms

1. Model metadata plus a configured **state limit** define how much KV / scratch may exist.
2. Short requests succeed; over-budget requests are **rejected** or explicitly **negotiate a shorter context** — never silent semantic truncation.
3. Cancellation releases state; unrelated sessions stay isolated.
4. Optional **prefix reuse** is keyed by exact tokens, model version, position, and relevant execution settings.
5. Prefer documenting / wiring llama.cpp `n_ctx`, KV types, and batch controls before inventing a second cache.

## Related stories

- [US-004](./004-runtime-sessions.md) — session binding
- [US-010](./010-llm-text-completions.md) — GGUF completions / context
- [US-023](./023-inference-telemetry.md) — phase timing around prefill/decode
- [US-025](./025-bounded-model-admission.md) — weight residency before state
- [US-028](./028-constrained-benchmark-controls.md) — measure peak state under caps

## Terms used

- [Inference state](../glossary.md#inference-state)
- [Session](../glossary.md#session)
- [Token](../glossary.md#token)
- [Memory budget](../glossary.md#memory-budget)

## Acceptance notes

- **Today (CI):** `FakeInferenceStateBudget` — accept within limit + release; hard reject when over; negotiate offers shorter `n_ctx` without silent truncation. GGUF already exposes `CompleteOptions.context_tokens` → llama `n_ctx`; callers should admit via the state budget first.
- **Live (future):** wire admit into `uie-llm` / complete path; measure peak vs `n_ctx`; same prompt/tokenizer.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_027_bounded_state.rs` | `cargo test -p uie-acceptance --test us_027_bounded_state` |

## Observability

- **Events:** `StoryEventLog` (`state_contract_*`) + `StateEvent` (`Accept` / `Reject` / `Negotiate` / `Release`).
- **CI test:** `us_027_bounded_state.rs`
