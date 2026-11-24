# US-030: Raspberry Pi GPU backend

**Status:** partial (CI fake probe + stub Backend; live Pi GPU gated)  
**Personas:** Pete, Jack  
**Crate / path:** `uie-core::accelerator` (`AcceleratorKind::PiGpu`, `StubAcceleratorBackend`); future real EP/Vulkan crate  
**Depends on:** [US-003](./003-backends-cpu.md), [US-024](./024-dependency-injection.md), [US-029](./029-capability-aware-kernels.md) probes

## User story

**As a** Pete  
**I want** a first-class [backend](../glossary.md#backend) for **Raspberry Pi GPU** acceleration when the board and drivers are available  
**So that** edge demos can use on-device graphics compute without forking a separate “Pi product.”

## In plain terms

1. Same `Backend` / runtime story as CPU — injected via DI, not a special-case binary.
2. Capability probe detects Pi GPU availability; missing hardware → explicit unsupported or CPU fallback (never silent wrong path).
3. Live acceptance runs only when a Pi GPU device is present (operator / lab machine); CI uses a fake probe + stub backend.
4. Telemetry tags compute device as GPU (or a Pi-specific label) under [US-023](./023-inference-telemetry.md).
5. Prefer existing stacks that already target Pi GPU (e.g. Vulkan / vendor APIs via llama.cpp or ONNX EP) before writing kernels from scratch.

## Related stories

- [US-003](./003-backends-cpu.md) — backend trait
- [US-026](./026-browser-wasm-inference.md) — edge WASM (CPU today; WebGPU reserved)
- [US-029](./029-capability-aware-kernels.md) — probes / fallback
- [US-031](./031-npu-backend.md) — NPU sibling accelerator
- [Mandate](../mandate.md) — compromise toward edge when choosing

## Terms used

- [Backend](../glossary.md#backend)
- [Device](../glossary.md#device)
- [Capability probe](../glossary.md#capability-probe)
- [Edge compute](../glossary.md#edge-compute)

## Acceptance notes

- **Today (CI):** `FakeAcceleratorProbe` + `StubAcceleratorBackend` — present → stub accel device; absent → CPU fallback; `require_accelerator` → explicit unsupported. Bound via `uie-iv`.
- **Live (future):** gated on real Pi 4/5 (or equivalent) with GPU access — set via `.env` / injectable, not hardcoded paths.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_030_pi_gpu_backend.rs` | `cargo test -p uie-acceptance --test us_030_pi_gpu_backend` |

## Observability

- **Events:** `StoryEventLog` contract markers (`pi_gpu_contract_*`).
- **CI test:** `us_030_pi_gpu_backend.rs`
