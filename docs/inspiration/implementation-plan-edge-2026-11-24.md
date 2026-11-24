# Implementation plan: edge resource + measurement stories

**Branch / PR:** work on `inspiration/trouble-measurement-controls-2026-11-24` (#22).  
**Status:** planning — stories are `planned`; no production behavior yet.  
**Mandate:** [`docs/mandate.md`](../mandate.md) — any-scale inference; when compromising, prefer edge/embedded.  
**Inputs:** [`trouble-edge-inference-2026-11-24.md`](./trouble-edge-inference-2026-11-24.md), [`trouble-measurement-controls-2026-11-24.md`](./trouble-measurement-controls-2026-11-24.md), [`inference-optimizations.md`](./inference-optimizations.md).

## Story map (EDGE → US)

| Provisional | Story | One-line |
|-------------|-------|----------|
| EDGE-MEM-01 | [US-025](../stories/025-bounded-model-admission.md) | Admit / reject / evict idle under injected memory budget |
| EDGE-STATE-02 | [US-027](../stories/027-bounded-inference-state.md) | Bound KV/context; no silent truncation |
| EDGE-BENCH-03 + controls | [US-028](../stories/028-constrained-benchmark-controls.md) | Frozen workloads + control/candidate/control brackets |
| EDGE-ARM-04 | [US-029](../stories/029-capability-aware-kernels.md) | Probe → dispatch / fallback; no illegal insn |

US-026 (WASM MNIST) stays the small Chromium gate; it is not replaced by these stories.

## Principles

1. **DI-first** ([US-024](../stories/024-dependency-injection.md)): `MemoryBudget`, clocks, and capability probes are injectables owned by modules — not call-site `UIE_*` branches.
2. **Extend, don’t fork:** US-028 grows [US-023](../stories/023-inference-telemetry.md) / e2e JSONL; US-025 grows host cache in `uie-ipc`; US-027 prefers llama.cpp controls before a second KV stack.
3. **CI before Pi:** every story ships deterministic fake providers first; live cgroup / Pi 4–5 is opt-in.
4. **Technique backlog is not scope:** MoE SSD offload, speculative decode, WebGPU kernels, etc. stay in [`inference-optimizations.md`](./inference-optimizations.md) until a story claims them.

## Phased delivery

### Phase 0 — Docs & contracts (this change set)

- [x] Inspiration tree on #22’s branch
- [x] US-025 / 027 / 028 / 029 stories + glossary terms
- [x] Acceptance files with contract `StoryEventLog` + `#[ignore]` behavioral stubs
- [x] Governance + acceptance index rows

### Phase 1 — US-025 admission (first product cut)

**Why first:** inspected gap — path-keyed host cache with no capacity / eviction; prevents live bench chaos.

1. [x] Define `MemoryBudget` / `Reservation` in `uie-core::resource`; bind via `uie-iv`.
2. [x] Fake provider for CI: fixed capacity, deterministic events.
3. [x] Wire `InferenceEngineHost::with_budget` / `load_cached`: estimate → reserve → load → commit / release on failure.
4. [x] Policy: hard reject when `evict_idle_on_pressure=false`; idle eviction when `true`.
5. [x] Acceptance: reject / reserve+evict / failed-load release with `StoryEventLog`.
6. [ ] Live: cgroup v2 reader — follow-up when Linux lab access is ready.

**Exit:** CI green with fake budget; US-025 marked `partial` for the fake path.

### Phase 2 — US-028 trustworthy comparison (extends US-023)

**Why second:** makes Phase 1 / later opts measurable without importing GPU folklore.

1. Workload descriptor (ids, hashes, cold/warm, caps) + bracket runner in `uie-e2e` or `uie-acceptance`.
2. Injected clock / trace fixtures: drift vs eligible win; incomplete → not comparable.
3. Token accounting from usage/tokenizer; unknown ≠ 0.
4. Negative-control hooks (wrong answer / over budget) block promotion.
5. Optional live A/B/A script for Pi; no numeric SLO until baselines exist.

**Exit:** CI bracket tests un-ignored; US-023 remains the schema source of truth.

### Phase 3 — US-027 bounded state

**Depends on:** Phase 1 for “model already admitted”; audit llama.cpp `n_ctx` / KV first.

1. Document existing binding knobs; add only what’s missing.
2. Session-level budget check before prefill; reject or negotiate shorter context.
3. Cancellation releases; optional prefix cache only with exact keys.
4. Un-ignore `us_027_reject_over_context_budget`.

### Phase 4 — US-029 capability kernels

**Depends on:** Phase 2 to prove wins; prefer Tract/llama.cpp/ONNX before new NEON kernels.

1. Capability probe injectable.
2. Dispatch telemetry event; forced-missing feature → fallback test.
3. Pi live optional.

### Phase 5 — Pi GPU + NPU backends (hardware-gated)

**Stories:** [US-030](../stories/030-pi-gpu-backend.md), [US-031](../stories/031-npu-backend.md).  
**Depends on:** Phase 4 probes; actual **hardware access** before live work.

1. Extend capability probes for “Pi GPU present” / “NPU present” (CI fakes first).
2. Register backends via DI; same admission + telemetry as CPU.
3. Prefer existing EP / llama.cpp / Vulkan (or vendor NPU runtime) over greenfield kernels.
4. Live acceptance stays opt-in in `.env` / injectables — no CI failure when silicon is absent.
5. Measure with US-028 brackets; never treat cloud-GPU numbers as Pi GPU/NPU forecasts.

## Deferred from the Troublescan (until a story claims them)

| Theme | Examples | Parked under |
|-------|----------|--------------|
| MoE offload / SSD stream | InferSSD, tandem.cpp, FreeToken | technique backlog |
| Speculative decode / draft KV | Nemotron Bolt, Lafa, EXL3 | backlog; US-028 notes future metrics |
| PagedAttention ports | vLLM mini-engines | consider only after US-027 baseline |
| WebGPU flash/dequant | browser kernels | after US-026 WebGPU reservation |

## Suggested experimental matrix (live, later)

Container caps from the edge note: 256/512 MiB (1 CPU), 1 GiB (2), 2/4 GiB (4). Stub → MNIST → small GGUF. Admission failures are expected for unsuitable combos, not bench failures.

## Definition of done (per story)

1. Story status → `partial` / `shipped` with real acceptance (not only contract markers).
2. Governance + `./scripts/acceptance-ci.sh` green.
3. Inspiration EDGE ids cross-linked to the shipped US (leave EDGE text as provenance).
4. No secrets or absolute machine paths in-tree — use `.env` / injectables.
