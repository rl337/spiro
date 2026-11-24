# US-001: Why UIE exists

**Status:** shipped  
**Personas:** Liz, Jack  
**Crate / path:** workspace root, `README.md`

## User story

**As** Liz, who inherited a Spirograph art script—and **as** Jack, who wants one platform for demos and models  
**I want** a single Rust engine that can run both toy workloads and real models **at any scale**, with an emphasis on edge and embedded hosts  
**So that** Liz is not maintaining parallel scripts and Jack can ship the same engine from a Pi or browser WASM build up to larger machines.

## In plain terms

This repo began as `spiro.py`, which fills a big pixel grid and saves a JPEG. UIE keeps that idea—**compute writes into a buffer**—and generalizes it into a small engine:

1. Data lives in [tensors](./002-tensors.md).
2. A [backend](../glossary.md#backend) runs the math.
3. A [runtime](./004-runtime-sessions.md) ties sessions, models, and plans together.
4. Demos like the [Spirograph](./008-spiro-demo.md) prove the plumbing; [model loaders](./007-models-loaders.md) extend it to ONNX and beyond.
5. The [project mandate](../mandate.md) prioritizes **edge and embedded** (WASM, Raspberry Pi–class ARM) without abandoning larger-scale backends.

You do not need to know Rust internals to follow the stories—start with [US-002](./002-tensors.md) if you care how data is stored.

## Related stories

- [US-002 — Tensors](./002-tensors.md) (foundation)
- [US-008 — Spirograph demo](./008-spiro-demo.md) (original motivation)
- [US-026 — Browser WASM](./026-browser-wasm-inference.md) (embedded WASM gate)
- [US-025](./025-bounded-model-admission.md)–[US-029](./029-capability-aware-kernels.md) (edge resource & measurement)

## Terms used

- [UIE](../glossary.md#uie)
- [Edge compute](../glossary.md#edge-compute)
- [Tensor](../glossary.md#tensor)
- [Workload](../glossary.md#workload)

## Acceptance notes

- Workspace `Cargo.toml` lists `uie-core` and demo crates.
- Root `README.md` and [`docs/mandate.md`](../mandate.md) state the any-scale / edge-emphasis goal.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_001_why_uie.rs` | `cargo test -p uie-acceptance --test us_001_why_uie` |

## Observability

- **Events (`StoryEventLog`):** `uie_core_present`, `story_doc_present`, `mandate_edge_embedded`
- **CI test:** `us_001_why_uie.rs`
