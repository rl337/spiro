# US-025: Bounded model admission and residency

**Status:** planned  
**Personas:** Jack, Liz  
**Crate / path:** `crates/uie-ipc` (host cache), `crates/uie-iv` (budget provider), loaders metadata  
**Inspiration:** EDGE-MEM-01 in [`docs/inspiration/trouble-edge-inference-2026-11-24.md`](../inspiration/trouble-edge-inference-2026-11-24.md)

## User story

**As a** Jack  
**I want** a model and its request state admitted only within a configured [memory budget](../glossary.md#memory-budget)  
**So that** one inference load cannot OOM the service or neighbouring container workloads.

## In plain terms

1. An injected **budget provider** (cgroup-backed live; fake in CI) exposes remaining capacity and peak usage.
2. Before load, the host **estimates** weights + workspace (+ unknown confidence) and **reserves** capacity.
3. If the next model would exceed the budget, the host **rejects / defers**, or **evicts an idle** cached executor per policy — never an in-use one.
4. Failed loads release reservations; concurrent admissions cannot double-spend the same bytes.
5. Telemetry / story events record model id, estimate, budget source, decision, and peak when known.

## Related stories

- [US-007](./007-models-loaders.md) — loaders / cache surface
- [US-020](./020-inference-byte-channel.md) — `InferenceEngineHost` residency
- [US-021](./021-model-registry-scan.md) — discovering candidates to load
- [US-024](./024-dependency-injection.md) — inject budget provider
- [US-027](./027-bounded-inference-state.md) — per-session state after admission
- [US-028](./028-constrained-benchmark-controls.md) — measure peak under cap

## Terms used

- [Memory budget](../glossary.md#memory-budget)
- [Model loader](../glossary.md#model-loader)
- [Inference executor](../glossary.md#inference-executor)
- [Composition root](../glossary.md#composition-root)

## Acceptance notes

- **Today:** planned; CI asserts story packaging + contract event names.
- **CI (future):** fake budget + fake loader — reserve / reject / release / concurrent failure with `StoryEventLog`.
- **Live (future):** Linux cgroup v2 `memory.max` / `current` / `peak` / `events` in a bounded container; no OOM kill under the controlled workload. Report process RSS separately from cgroup usage.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_025_bounded_admission.rs` | `cargo test -p uie-acceptance --test us_025_bounded_admission` |

## Observability

- **Events:** `StoryEventLog` contract markers (`admission_contract_*`); behavioral tests will assert `reserve` / `reject` / `release` / `evict_idle` once implemented.
- **CI test:** `us_025_bounded_admission.rs`
