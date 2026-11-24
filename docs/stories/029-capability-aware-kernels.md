# US-029: Capability-aware optimized execution

**Status:** planned  
**Personas:** Pete, Toofer  
**Crate / path:** `crates/uie-core` backends / ops; optional ARM NEON paths; prefer llama.cpp / ONNX delegation first  
**Inspiration:** EDGE-ARM-04 in [`docs/inspiration/trouble-edge-inference-2026-11-24.md`](../inspiration/trouble-edge-inference-2026-11-24.md)

## User story

**As a** Pete  
**I want** supported ARM (and other) kernel paths selected safely  
**So that** small CPUs benefit without illegal instructions or silent numerical wrongness.

## In plain terms

1. For an op with scalar and optimized backends, compare deterministic fixtures under declared tolerance (odd shapes / tails included).
2. Dispatch records backend / kernel / precision / reason; missing features **fall back** or return **explicit unsupported** — never SIGILL.
3. Integer paths validate exactly where the math contract allows; float tolerances are per op and precision.
4. Investigate existing Tract / llama.cpp / ONNX Runtime delegation before writing new Rust kernels.
5. Live Pi 4/5 measurements are optional and include thermal / memory effects.

## Related stories

- [US-003](./003-backends-cpu.md) — CPU backend
- [US-005](./005-execution-plans.md) / [US-006](./006-operations.md) — graph ops
- [US-009](./009-onnx-mnist.md) — Tract ONNX path already used in WASM ([US-026](./026-browser-wasm-inference.md))
- [US-028](./028-constrained-benchmark-controls.md) — prove a fast path is a real win
- [US-030](./030-pi-gpu-backend.md) / [US-031](./031-npu-backend.md) — accelerator backends after probes exist

## Terms used

- [Backend](../glossary.md#backend)
- [Operation](../glossary.md#operation)
- [Device](../glossary.md#device)
- [Capability probe](../glossary.md#capability-probe)

## Acceptance notes

- **Today:** planned; CI asserts contract events.
- **CI (future):** scalar vs optimized fixture equality / tolerance; dispatch event; forced-unsupported path falls back cleanly.
- **Live (future):** Pi 4/5 kernel and end-to-end cost; feature detection for NEON / dotprod rather than assume.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_029_capability_kernels.rs` | `cargo test -p uie-acceptance --test us_029_capability_kernels` |

## Observability

- **Events:** `StoryEventLog` contract markers (`kernel_contract_*`); later assert `dispatch` / `fallback` / `unsupported`.
- **CI test:** `us_029_capability_kernels.rs`
