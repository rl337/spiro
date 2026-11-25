# US-030: Raspberry Pi GPU backend

**Status:** partial (CI fakes + GGUF `n_gpu_layers` offload; Vulkan=Pi, Metal/CUDA=HostGpu lab)  
**Personas:** Pete, Jack  
**Crate / path:** `uie-core::accelerator`, `uie-loader-gguf` (`LiveGgmlAcceleratorProbe`, `GgufGpuOffload`); Pi live still hardware-gated  
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

- **Today (CI):** `FakeAcceleratorProbe` + `StubAcceleratorBackend`; offload preference prefers `PiGpu` over `HostGpu`.
- **Today (lab):** `LiveGgmlAcceleratorProbe` maps ggml **Vulkan→PiGpu**, **Metal/CUDA→HostGpu**. `uie-llm` auto-offloads with `n_gpu_layers=99` when either is present (`--cpu-only` / `--gpu-layers N` override). `uie-llm list-gpu` prints devices.
- **Live Pi (crystalline):** Raspberry Pi 5 (`CRYSTALLINE_HOST` in `~/dev/home_network/inventory.env`). Image `uie-llm:vulkan-pi` (Trixie + llama Vulkan) runs beside `hn_laya`. `list-gpu` sees **V3D 7.1 → PiGpu**. Stock llama.cpp cannot **execute** on V3D yet (16KiB shared memory / WG limits — [llama.cpp#9801](https://github.com/ggml-org/llama.cpp/issues/9801)); CPU completions work without `/dev/dri`. Script: `scripts/crystalline-pi-gpu-experiment.sh`.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_030_pi_gpu_backend.rs` | `cargo test -p uie-acceptance --test us_030_pi_gpu_backend` |

## Observability

- **Events:** `StoryEventLog` contract markers (`pi_gpu_contract_*`).
- **CI test:** `us_030_pi_gpu_backend.rs`
