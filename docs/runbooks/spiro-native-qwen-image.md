# Spiro-native: Qwen-Image weights → analyze (no Comfy / LM Studio HTTP)

Use **registry discovery** + **`InferenceRuntime`** + **byte pipeline** only.

## Architecture (phase 1 — shipped)

| Step | Loader | What happens |
|------|--------|----------------|
| T2I | `SafetensorsModelLoader` | `mmap`/read Comfy checkpoint from `comfy-shared:` id; **contract infer** (US-012 shapes) |
| Image→text | `uie-loader-gguf` + **mmproj** | **Qwen2-VL ~2B** reads the PNG via llama.cpp MTMD (pixels, not stub) |

Real Qwen-Image **denoise** inside Rust is US-012 follow-on; phase 1 proves **weights load through Spiro** and the multi-hop path runs.

Text-only **Qwen3-2B** cannot see images; use **Qwen2-VL-2B-Instruct** (main `.gguf` + `mmproj-*.gguf` in the same LM Studio folder).

## Local run

```bash
export SPIRO_MODEL_ROOTS=$PWD/config/model_roots.yaml
export CXX=g++ CC=gcc RUSTFLAGS="-C linker=g++"

cargo run -p uie-e2e --features spiro-chain -- list-models
cargo run -p uie-e2e --features spiro-chain -- run spiro_native_qwen_chain
```

Output: `/tmp/uie-artifacts/spiro_native_image.png` (or `--out-dir` / `UIE_ARTIFACT_DIR`).

Prerequisites for the vision step:

- **Qwen2-VL-2B-Instruct** (or similar) under `lmstudio:` roots — both `*.gguf` and matching `*mmproj*.gguf`
- Or set `UIE_QWEN_VL_GGUF` and `UIE_QWEN_VL_MMPROJ` to absolute paths

```bash
cargo run -p uie-llm -- list-gguf | rg -i 'qwen.*2b'
```

CI / no local VL weights: `--skip-llm` (stops after PNG; no stub caption).

Optional: `--prompt "..."` and `--max-new 64` on the e2e `run` args after `--`.

## Loader crate

`uie-loader-safetensors`: opens `.safetensors`, maps registry **role → modality contract**, executes via bound stub policy until diffusion kernels land.

`uie-serve` live profile: enable `safetensors` on `uie-iv` (`LiveLoaderSet`).

## vs HTTP scripts

`scripts/local-qwen-image-pipeline.sh` talks to Comfy/LM Studio APIs. Prefer this runbook for **in-process Spiro**.