# US-028: Constrained benchmarks and trustworthy comparisons

**Status:** planned  
**Personas:** Pete, Jack  
**Crate / path:** `crates/uie-telemetry`, `crates/uie-e2e`, `crates/uie-acceptance`  
**Inspiration:** EDGE-BENCH-03 + measurement amendment in [`docs/inspiration/trouble-edge-inference-2026-11-24.md`](../inspiration/trouble-edge-inference-2026-11-24.md) and [`docs/inspiration/trouble-measurement-controls-2026-11-24.md`](../inspiration/trouble-measurement-controls-2026-11-24.md)

## User story

**As a** Pete  
**I want** repeatable, apples-to-apples measurements of model variants and runtime knobs — with **control brackets** that reject measurement artifacts  
**So that** I can choose smaller / faster configs by quality and cost, not folklore or chunk-count tok/s.

## In plain terms

1. Freeze workload id, model/tokenizer/quant hashes, backend revision, sampling, generation cap, CPU quota/threads, memory cap, and cold/warm policy. Only declared variables may change.
2. Record load time, TTFT, decode throughput, peak RSS / cgroup usage, error/OOM counts, and task quality (labels/F1 or fixed assertions — not “same tokens across quants”).
3. **Control / candidate / control** brackets (injected clocks in CI) mark **drift / inconclusive** vs eligible wins under a predeclared margin; missing counters are **unknown**, not zero.
4. Count generated tokens from tokenizer / server usage — **never** transport chunk counts. Warm cache hits are not cold prefill.
5. Negative controls (loops, wrong retrieval, over-budget) must not promote; retain failed data. Live Pi 4/5 runs are opt-in and report conditions (thermal / throttle) when available.

## Related stories

- [US-009](./009-onnx-mnist.md) — tiny classifier panel member
- [US-010](./010-llm-text-completions.md) — GGUF panel member
- [US-023](./023-inference-telemetry.md) — trace / metrics schema to extend
- [US-024](./024-dependency-injection.md) — injected clocks and resource providers
- [US-025](./025-bounded-model-admission.md) — admission under the same caps
- [US-026](./026-browser-wasm-inference.md) — browser CPU gate (separate CI)

## Terms used

- [Execution trace](../glossary.md#execution-trace)
- [Infer metrics](../glossary.md#infer-metrics)
- [Benchmark bracket](../glossary.md#benchmark-bracket)
- [Workload](../glossary.md#workload)

## Acceptance notes

- **Today:** planned; CI asserts contract events (extends US-023, does not replace it).
- **CI (future):** fixture rates e.g. baseline 100 / candidate 108 / control 110 → drift; fake stream 3 chunks / 9 tokens → usage-based counts; incomplete bracket rejected.
- **Live (future):** interleaved A/B/A on ARM64 containers; median + spread; no universal speed threshold until baselines exist.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_028_benchmark_controls.rs` | `cargo test -p uie-acceptance --test us_028_benchmark_controls` |

## Observability

- **Events / traces:** `StoryEventLog` contract markers (`bench_contract_*`); later assert bracket ids, reason codes, and US-023 phase fields with cold/warm tags.
- **CI test:** `us_028_benchmark_controls.rs`
