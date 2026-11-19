# US-020: Inference byte channel (in-process and Unix socket)

**Status:** partial (stubs + protocol shipped; PyO3 embed optional)  
**Personas:** Liz, Pete  
**Crate / path:** `crates/uie-ipc`, `crates/uie-serve`, `python/uie_client.py`

## User story

**As** Liz, integrating Python tools—and **as** Pete, running a long-lived engine process  
**I want** every infer request/response to go through a **byte channel** with the same framing everywhere  
**So that** local in-process tests, `uie-serve` on a Unix socket, and Python clients all speak one protocol.

## In plain terms

1. Client sends **length-prefixed JSON** (`uie-ipc` framing): `hello`, `infer`, `infer_result`, or `error`.
2. `InferenceEngineHost` owns the [runtime](./004-runtime-sessions.md), [loaders](./007-models-loaders.md), and a model cache.
3. **In-process:** `InProcessChannel::roundtrip` — no socket; same messages (tests, future PyO3).
4. **Unix socket:** `cargo run -p uie-serve` → `UIE_SOCKET` (default `/tmp/uie.sock`); Python uses `python/uie_client.py`.
5. Tensors cross the wire as `name`, `dtype`, `shape`, raw `data` bytes ([tensor](./002-tensors.md) layout).

## Related stories

- [US-004 — Runtime](./004-runtime-sessions.md)
- [US-007 — Loaders](./007-models-loaders.md)
- [US-008 — Spiro](./008-spiro-demo.md) (native demo; IPC path for `python/spiro_via_engine.py`)

## Terms used

- [Inference runtime](../glossary.md#inference-runtime)
- [Byte channel](../glossary.md#byte-channel)

## Acceptance notes

- `cargo test -p uie-ipc`
- `cargo run -p uie-serve` + `python3 python/uie_client.py --stub llm`
- ONNX over IPC: register `OnnxModelLoader` on the host when `uie-loader-onnx` build is green.