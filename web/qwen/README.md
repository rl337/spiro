# Browser Qwen CPU chat (US-026)

This demo loads the existing Qwen3 0.6B GGUF into llama.cpp WASM through
`@wllama/wllama@4.0.0`. Inference runs in a browser worker on CPU. This is a
separate GGUF executor alongside the Rust `uie-wasm` Tract ONNX engine; it does
not compile the native Rust GGUF loader to WASM.

```bash
cd web/qwen
npm ci
npm start
# Open http://127.0.0.1:18766
```

Set `SPIRO_QWEN_MODEL` in the repo-root gitignored `.env` (see `.env.example`), or export it
in the shell. `./scripts/browser-qwen-ci.sh` loads `.env` automatically. If unset, the
server falls back to
`~/.lmstudio/models/Qwen/Qwen3-0.6B-GGUF/Qwen3-0.6B-Q8_0.gguf`.
The model is served from disk; no weights are copied into the repository.
The loopback-only server serves files, HEAD and byte ranges, and provides
COOP/COEP for WASM threads. It has no inference API.

Run the real-model Chromium acceptance test from the repository root:

```bash
./scripts/browser-qwen-ci.sh
```

This is an opt-in live test because it requires a large model file. The existing
MNIST workflow remains the small default CI gate. Provision the GGUF and set
`SPIRO_QWEN_MODEL` to run the Qwen test in CI; on Linux, install Chromium system
dependencies with `npx playwright install --with-deps chromium`.

The test disables Chromium GPU acceleration and sets `n_gpu_layers: 0`, uses
four CPU threads, and disables thinking via the model's embedded chat template.
After loading it switches the browser offline, checks three dependent turns
with a randomly generated code, and verifies a fresh conversation cannot recall
it. Requests replay full history with `cache_prompt: false`; reset clears that
history. Reset and concurrent chat requests are rejected during generation.
This establishes multi-turn behavior, not a claim of persistent KV-cache reuse.
This demo rejects requests exceeding 15 messages or 1024 UTF-8 bytes of serialized
history, asks the user to start a new conversation, and preserves existing history
on failure. It does not implement an automatic context-truncation policy.

Playwright attaches the full transcript, timings, model SHA-256, browser/runtime
versions, settings, metadata and request log to its test results. No automatic
retries are enabled. `preferredDevice: auto|cpu` selects CPU; `webgpu` is reserved
and explicitly rejected so a future executor can implement it without changing
the chat facade.
