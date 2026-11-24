# US-009: ONNX MNIST digit classification

**Status:** shipped (vendored model; loader + CI infer in `native-compile`)  
**Personas:** Liz, Kenneth  
**Crate / path:** `models/mnist-8.onnx`, `crates/uie-loader-onnx`, `crates/uie-mnist`

## User story

**As** Liz, validating the engine—and **as** Kenneth, running his first “real” model  
**I want** a tiny, well-known neural net I can load and run from the CLI  
**So that** we verify [model loaders](./007-models-loaders.md) and [inference](./004-runtime-sessions.md) on real weights—not only the Spirograph.

## In plain terms

**Model chosen:** ONNX Model Zoo **MNIST-8** (`mnist-8.onnx`, ~26 KiB).

**Why this model:**

- Small enough to vendor in git.
- Widely documented (28×28 grayscale digits 0–9).
- Exercises the [inference executor](../glossary.md#inference-executor) path end-to-end.
- Runs on CPU via the Tract ONNX runtime in `uie-loader-onnx` (no GPU required).

**Flow:**

1. Register `OnnxModelLoader` on the [runtime](./004-runtime-sessions.md).
2. `load_model("models/mnist-8.onnx")`.
3. Load a fixture [tensor](./002-tensors.md) (`models/fixtures/digit_one_f32.bin`, synthetic “1”).
4. `infer` → logits → argmax digit printed by `uie-mnist`.

See `models/README.md` for input/output shapes and license.

## Related stories

- [US-007 — Loaders](./007-models-loaders.md)
- [US-002 — Tensors](./002-tensors.md) (`F32`, shape `[1,1,28,28]`)
- [US-004 — Runtime](./004-runtime-sessions.md) (`infer`, session output binding)
- [US-026 — Browser WASM](./026-browser-wasm-inference.md) (same MNIST weights in Chromium)

## Terms used

- [Model loader](../glossary.md#model-loader)
- [Inference executor](../glossary.md#inference-executor)
- [I/O spec](../glossary.md#io-spec)
- [Tensor](../glossary.md#tensor)

## Acceptance notes

- **Target command:** `cargo run -p uie-mnist`.
- **CI infer:** `.github/workflows/native-compile.yml` runs `cargo test -p uie-mnist mnist_inference_digit_one` (digit `1` on stroke fixture + ONNX phases).
- **Env overrides:** `UIE_MNIST_MODEL`, `UIE_MNIST_FIXTURE`.
- **Limitation:** Requires Rust **1.96+** in `rust-toolchain.toml` for current Tract dependency chain; ONNX dtypes beyond `F32` not supported yet.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_009_mnist.rs` | `cargo test -p uie-acceptance --test us_009_mnist` |
| Rust (native CI) | `crates/uie-mnist/src/main.rs` | `cargo test -p uie-mnist mnist_inference_digit_one` |
| Rust (live) | `crates/uie-acceptance/tests/live_models.rs` | `UIE_ACCEPTANCE_MODE=live cargo test -p uie-acceptance --features live-onnx live_us_009_mnist_onnx_infer` |

## Observability

- **Events:** `mnist_fixture_present`, `mnist_input_tensor_bytes`
- **CI test:** `us_009_mnist.rs`
- **Live:** `live_us_009_mnist_onnx_infer` uses `assert_infer_outcome_trace` (ONNX phases)
