# US-033: In-process text embeddings

**Status:** partial (CI stub; live sentence-transformer weights later)  
**Personas:** Liz, Pete  
**Crate / path:** `crates/uie-embeddings`, DI via `uie-iv` (`text_embedder`)

## User story

**As a** Liz (weirdness hub)  
**I want** an in-process Rust library that turns text batches into L2-normalized vectors  
**So that** the hub can embed chunks without an embeddings HTTP API or a second process.

## In plain terms

1. Host depends on `uie-embeddings` and calls `TextEmbedder::embed` with UTF-8 strings.
2. Output is one equal-length `Vec<f32>` per input, L2-normalized, tagged with a stable `model_id`.
3. CI uses `StubTextEmbedder` (`uie-stub-text-embed-v1`, 384-d) with deterministic vectors + telemetry.
4. Live reference weights (BGE-small / MiniLM on Pi 5 CPU) are a later enablement under the same trait.
5. Normative contract: [text-embeddings.md](../requirements/text-embeddings.md).

## Related stories

- [US-015](./015-modality-catalog.md) — modality hub (this is library-first, not byte-channel)
- [US-021](./021-model-registry-scan.md) — future weight+tokenizer bundle admission
- [US-023](./023-inference-telemetry.md) — execution recorder phases
- [US-024](./024-dependency-injection.md) — composition root binds the embedder
- [US-025](./025-bounded-model-admission.md) — live load must fit memory budget

## Terms used

- [Text embedding](../glossary.md#text-embedding)
- [Composition root](../glossary.md#composition-root)
- [Execution trace](../glossary.md#execution-trace)
- [Edge compute](../glossary.md#edge-compute)

## Acceptance notes

- **Today (CI):** stub embedder via injector; empty batch → empty output; batch ≤ 64; telemetry phases `embed_validate` + `embed_encode`.
- **Live:** optional weight directory + tokenizer bundle; not required for CI green.
- Stub `model_id` must **not** be advertised as the EMB-06 reference model.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_033_text_embeddings.rs` | `cargo test -p uie-acceptance --test us_033_text_embeddings` |

## Observability

- **Trace:** `assert_embed_telemetry` (`embed_validate`, `embed_encode`, format=stub, user_story=US-033).
- **CI test:** `us_033_text_embeddings.rs`
