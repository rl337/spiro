# US-031: NPU / on-device accelerator backend

**Status:** partial (CI fake probe + stub Backend; live NPU gated)  
**Personas:** Pete, Jack  
**Crate / path:** `uie-core::accelerator` (`AcceleratorKind::Npu`, `StubAcceleratorBackend`); future vendor crates behind same trait  
**Depends on:** [US-003](./003-backends-cpu.md), [US-024](./024-dependency-injection.md), [US-029](./029-capability-aware-kernels.md)

## User story

**As a** Jack  
**I want** UIE to target **NPU** (neural processing unit) and similar on-device accelerators when we have access to the silicon and SDKs  
**So that** edge deployments can use dedicated inference hardware without abandoning the shared engine, loaders, and telemetry.

## In plain terms

1. NPUs are another [backend](../glossary.md#backend) / device class — same admission, traces, and DI patterns as CPU and Pi GPU.
2. Vendor SDKs differ (Apple Neural Engine, Qualcomm, Coral, Hailo, etc.); the story is the **UIE surface**, not a single chip forever.
3. Work starts when we have **real hardware access** for at least one target; until then CI keeps fake probes and contract tests only.
4. Missing NPU → explicit unsupported or CPU/GPU fallback; never pretend NPU ran.
5. Quantization and I/O layouts often differ from GPU — measure under [US-028](./028-constrained-benchmark-controls.md); do not import cloud-NPU folklore as Pi results.

## Related stories

- [US-003](./003-backends-cpu.md) — backend trait
- [US-030](./030-pi-gpu-backend.md) — Pi GPU sibling
- [US-025](./025-bounded-model-admission.md) — budgets still apply (NPU + host RAM)
- [US-028](./028-constrained-benchmark-controls.md) — trustworthy comparisons
- [Mandate](../mandate.md)

## Terms used

- [NPU](../glossary.md#npu)
- [Backend](../glossary.md#backend)
- [Capability probe](../glossary.md#capability-probe)
- [Edge compute](../glossary.md#edge-compute)

## Acceptance notes

- **Today (CI):** `FakeAcceleratorProbe` + `StubAcceleratorBackend` — present → stub NPU device; absent → CPU fallback; never pretend NPU ran. Bound via `uie-iv`.
- **Live (future):** gated on operator-provided NPU access (injectable device selector in `.env` / modules).

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_031_npu_backend.rs` | `cargo test -p uie-acceptance --test us_031_npu_backend` |

## Observability

- **Events:** `StoryEventLog` contract markers (`npu_contract_*`).
- **CI test:** `us_031_npu_backend.rs`
