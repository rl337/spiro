# Qwen3-VL 2B — LM Studio layout (Spiro lookup)

Cloud agents **cannot** read your laptop’s `~/.lmstudio/models`. This note records the **expected** on-disk layout when you install the LM Studio catalog model **[qwen/qwen3-vl-2b](https://www.lmstudio.ai/models/qwen/qwen3-vl-2b)** (GGUF source: [lmstudio-community/Qwen3-VL-2B-Instruct-GGUF](https://huggingface.co/lmstudio-community/Qwen3-VL-2B-Instruct-GGUF)).

## Files (same folder)

| File | Role |
|------|------|
| `Qwen3-VL-2B-Instruct-Q4_K_M.gguf` (or Q6/Q8, or `model.gguf` after import) | Language model |
| `mmproj-Qwen3-VL-2B-Instruct-F16.gguf` (or Q8 mmproj) | Vision projector — **required** for image input |

Older HF naming may use `Qwen3VL-2B-Instruct-…` without hyphens after `Qwen3`; Spiro pairs by **mmproj in the same directory**, not exact filenames.

## Typical path (macOS / Linux)

```text
~/.lmstudio/models/lmstudio-community/Qwen3-VL-2B-Instruct-GGUF/Qwen3-VL-2B-Instruct-Q4_K_M.gguf
~/.lmstudio/models/lmstudio-community/Qwen3-VL-2B-Instruct-GGUF/mmproj-Qwen3-VL-2B-Instruct-F16.gguf
```

Windows: `%USERPROFILE%\.lmstudio\models\...` (same nesting).

## Spiro registry id pattern

```text
lmstudio:lmstudio-community/Qwen3-VL-2B-Instruct-GGUF/Qwen3-VL-2B-Instruct-Q4_K_M.gguf
```

(`config/model_roots.yaml` → `id: lmstudio`, path `~/.lmstudio/models`.)

## Cloud VM smoke (0.6B, smaller download)

```bash
./scripts/download-qwen3-vl-0.6b-fixture.sh
export SPIRO_MODEL_ROOTS=$PWD/config/model_roots.fixture-vl.yaml
export CXX=g++ CC=gcc RUSTFLAGS="-C linker=g++"
cargo run -p uie-e2e --features spiro-chain -- run spiro_native_qwen_chain -- --max-new 96
```

Source: [oOundefinedOo/Qwen3-0.6B-Visual-GGUF](https://huggingface.co/oOundefinedOo/Qwen3-0.6B-Visual-GGUF) (~545 MiB). GGUF weights are gitignored; use the script to fetch.

## On your laptop: confirm paths

```bash
export SPIRO_MODEL_ROOTS=$PWD/config/model_roots.yaml
cargo run -p uie-e2e --features spiro-chain -- list-vl-bundles
```

Pin auto-discovery:

```bash
export UIE_QWEN_VL_REGISTRY_ID='lmstudio:...'   # from list-models, non-mmproj .gguf
```

## Not the same as text-only Qwen3-2B

If you also have **Qwen3-2B-Instruct** (chat only, no mmproj), `qwen3_2b_chat_completion` uses that; **`spiro_native_qwen_chain`** needs the **VL** folder above for the image step.