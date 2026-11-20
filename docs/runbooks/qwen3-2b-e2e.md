# Qwen3 2B — local end-to-end (GGUF)

Run a **Qwen3 ~2B** instruct GGUF from LM Studio (or any tree under `SPIRO_MODEL_ROOTS`) through UIE with the model’s baked-in **chat template** (required for sane Qwen replies).

## Prerequisites

- Rust **1.96+** (`rust-toolchain.toml`)
- Linux/macOS: **g++/build-essential** (see [`scripts/build-gguf.sh`](../../scripts/build-gguf.sh))
- A local **Qwen3 2B** (or 1.7B/2.x) **GGUF** — e.g. download in [LM Studio](https://lmstudio.ai/) into `~/.lmstudio/models/`

## One-time setup

```bash
cp config/model_roots.example.yaml config/model_roots.yaml
# Ensure lmstudio root points at ~/.lmstudio/models

export SPIRO_MODEL_ROOTS=$PWD/config/model_roots.yaml
./scripts/build-gguf.sh
```

Confirm the registry sees your file:

```bash
cargo run -p uie-llm -- list-gguf | rg -i qwen
```

Note the **registry id** (first column), e.g. `lmstudio:Qwen/Qwen3-2B-.../model.gguf`.

## Path A — Direct completion (recommended)

Chat-formatted single user turn (uses llama.cpp `apply_chat_template` from GGUF metadata):

```bash
export SPIRO_MODEL_ROOTS=$PWD/config/model_roots.yaml

# Auto-pick first registry entry whose id/path contains "qwen" and "2b"
cargo run -p uie-llm -- --auto-qwen3-2b --chat \
  --prompt "What is the capital of France? Reply in one short sentence." \
  --max-new 64

# Or pin a registry id / file
export UIE_QWEN3_2B_REGISTRY_ID='lmstudio:...'
cargo run -p uie-llm -- --chat --registry-id "$UIE_QWEN3_2B_REGISTRY_ID" --prompt "Hello"

# Or absolute path
UIE_GGUF_MODEL=~/.lmstudio/models/.../model.gguf \
  cargo run -p uie-llm -- --chat --model "$UIE_GGUF_MODEL" --prompt "Hello"
```

Optional: same flow through **`InferenceRuntime`** + `GgufModelLoader`:

```bash
UIE_LLM_USE_RUNTIME=1 cargo run -p uie-llm -- --auto-qwen3-2b --chat --prompt "Hi"
```

Telemetry lines on stderr; completion text on stdout.

### End-to-end harness (preferred)

E2E tests are command objects under the **`uie-e2e`** binary — see [End-to-end tests](../testing/end-to-end-tests.md).

```bash
cargo run -p uie-e2e --features gguf -- list
cargo run -p uie-e2e --features gguf -- run qwen3_2b_chat_completion
cargo run -p uie-e2e --features gguf -- run 'qwen*' -- --prompt "Hello" --max-new 32
```

### Convenience script

Wraps the harness target above (builds GGUF native deps first):

```bash
./scripts/e2e-qwen3-2b.sh
UIE_LLM_PROMPT="Explain RAM in one line." ./scripts/e2e-qwen3-2b.sh -- --prompt "Explain RAM in one line."
```

## Path B — Serve + IPC (optional)

1. Build serve with GGUF loaders:

   ```bash
   cargo build -p uie-serve --features gguf
   ```

2. In one terminal:

   ```bash
   export UIE_SERVE_PROFILE=live
   export SPIRO_MODEL_ROOTS=$PWD/config/model_roots.yaml
   cargo run -p uie-serve --features gguf
   ```

   Note the socket path logged (`UIE_SOCKET` or `$TMPDIR/uie-serve-<pid>.sock`).

3. Prefer **Path A** for chat formatting today; IPC clients send a **already formatted** U8 `prompt` tensor (`uie-loader-gguf` I/O spec). Path A’s `--chat` output is what you would bind as prompt bytes for a custom IPC client.

## Live acceptance (optional)

```bash
export UIE_ACCEPTANCE_MODE=live
export UIE_GGUF_MODEL=/path/to/qwen3-2b.gguf
cargo test -p uie-acceptance --features gguf live_us_010_gguf_text_completion -- --nocapture
```

For chat-shaped smoke:

```bash
UIE_LLM_CHAT=1 UIE_GGUF_MODEL=... cargo run -p uie-llm -- --chat --prompt "Say OK"
```

## Troubleshooting

| Symptom | Fix |
|---------|-----|
| `cstdlib` / C++ link errors | Run `./scripts/build-gguf.sh` (sets `CXX=g++`) |
| Gibberish / template leak | Use **`--chat`**, not raw `--prompt` |
| No model in `list-gguf` | Fix `model_roots.yaml` path; refresh LM Studio download |
| Slow on CPU | Expected for 2B; set `UIE_GGUF_GPU=1` when GPU build enabled |

Story: [US-010](../stories/010-llm-text-completions.md).