# US-026: Browser WASM inference (ONNX MNIST and GGUF chat)

**Status:** shipped  
**Personas:** Pete, Liz  
**Crate / path:** `crates/uie-wasm`, `web/mnist/`, `crates/uie-loader-onnx` (`from_bytes`)

## User story

**As a** Pete  
**I want** a CPU WebAssembly inference engine in Chromium that runs our tiny ONNX MNIST model  
**So that** we can demonstrate UIE in the browser and gate it in CI—without blocking a future WebGPU backend.

## In plain terms

1. `OnnxModel::from_bytes` loads ONNX from memory (no filesystem), shared by native and WASM.
2. `uie-wasm` exposes `WasmInferenceEngine` via wasm-bindgen: prefer `auto`/`cpu` today; `webgpu` fails with `webgpu_not_implemented`.
3. The static page under `web/mnist/` fetches vendored `mnist-8.onnx` + digit-one fixture, runs infer, shows predicted digit.
4. Playwright launches **Chromium** and asserts digit `1`, device `cpu`, and positive `cpu_time_ms`.
5. `web/qwen/` loads the existing Qwen3 0.6B GGUF into a pinned llama.cpp WASM runtime (`@wllama/wllama`), alongside the Rust ONNX engine. CPU-only multi-turn chat runs after Chromium goes offline; reset discards conversation history.

## Related stories

- [US-003](./003-backends-cpu.md) — backends / future WASM  
- [US-009](./009-onnx-mnist.md) — MNIST ONNX on native  
- [US-023](./023-inference-telemetry.md) — CPU phase metrics  

## Terms used

- [Backend](../glossary.md#backend)
- [Device](../glossary.md#device)
- [Model loader](../glossary.md#model-loader)

## Acceptance notes

- Native bytes path: `cargo test -p uie-wasm`
- Browser CI: `./scripts/wasm-mnist-ci.sh` / `.github/workflows/uie-wasm-chromium.yml`
- WebGPU is an explicit reserved device string, not a silent CPU fallback.
- Live GGUF browser test: `./scripts/browser-qwen-ci.sh` with the local LM Studio Qwen3 0.6B or `SPIRO_QWEN_MODEL`. See `web/qwen/README.md`; it requires a model and is separate from the small default MNIST CI gate.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-wasm/src/engine.rs` (`native_tests`) | `cargo test -p uie-wasm` |
| Rust | `crates/uie-acceptance/tests/us_026_browser_wasm.rs` | `cargo test -p uie-acceptance --test us_026_browser_wasm` |
| Chromium | `web/mnist/mnist.spec.mjs` | `./scripts/wasm-mnist-ci.sh` |
| Chromium (live) | `web/qwen/qwen.spec.mjs` | `./scripts/browser-qwen-ci.sh` |

## Observability

- Native unit test: `assert_phases` for `onnx_validate_inputs` / `onnx_tract_run` on the bytes-loaded model.
- Chromium Playwright: asserts `data-digit=1`, `data-device=cpu`, and `cpu_time_ms > 0`.
- Acceptance: `StoryEventLog` / `assert_story_events` for packaging assets (`us_026_browser_wasm.rs`).
- GGUF: Playwright captures offline three-turn recall and reset-control transcripts, first-token and total timings, actual GGUF SHA-256, browser version, settings and request log. No prompts go to a server.
