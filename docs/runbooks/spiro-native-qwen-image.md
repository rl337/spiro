# Spiro-native: Qwen-Image weights → analyze (no Comfy / LM Studio HTTP)

Use **registry discovery** + **`InferenceRuntime`** + **byte pipeline** only.

## Architecture (phase 1 — shipped)

| Step | Loader | What happens |
|------|--------|----------------|
| T2I | `SafetensorsModelLoader` | `mmap`/read Comfy checkpoint from `comfy-shared:` id; **contract infer** (US-012 shapes) |
| Image→text | `uie-loader-gguf` + **mmproj** | **Qwen VL ~2B** (your LM Studio download) reads the PNG via llama.cpp MTMD |

Real Qwen-Image **denoise** inside Rust is US-012 follow-on; phase 1 proves **weights load through Spiro** and the multi-hop path runs.

Use the **same Qwen ~2B VL** you run in LM Studio — not the text-only 2B chat model. Spiro detects VL by a **`mmproj` `.gguf` in the same folder** as the main weights (names often omit `vl`, e.g. `model.gguf` + `mmproj-….gguf`). Expected catalog entry: [Qwen3-VL 2B in LM Studio](https://www.lmstudio.ai/models/qwen/qwen3-vl-2b) — see [qwen3-vl-2b-lmstudio.md](./qwen3-vl-2b-lmstudio.md) for typical paths.

## Local run

```bash
export SPIRO_MODEL_ROOTS=$PWD/config/model_roots.yaml
export CXX=g++ CC=gcc RUSTFLAGS="-C linker=g++"

cargo run -p uie-e2e --features spiro-chain -- list-models
cargo run -p uie-e2e --features spiro-chain -- run spiro_native_qwen_chain
```

Output: `/tmp/uie-artifacts/spiro_native_image.png` (or `--out-dir` / `UIE_ARTIFACT_DIR`).

Prerequisites for the vision step:

- Qwen **VL** ~2B under `lmstudio:` roots (main `.gguf` + `*mmproj*.gguf` in that folder)
- Or `UIE_QWEN_VL_REGISTRY_ID` from `list-models` / `list-gguf`
- Or `UIE_QWEN_VL_GGUF` + `UIE_QWEN_VL_MMPROJ`

```bash
cargo run -p uie-e2e --features spiro-chain -- list-models | rg -i qwen
# pick the folder that contains both model.gguf (or named .gguf) and mmproj
```

CI / no local VL weights: `--skip-llm` (stops after PNG; no stub caption).

Optional: `--prompt "..."` and `--max-new 64` on the e2e `run` args after `--`.

## Loader crate

`uie-loader-safetensors`: opens `.safetensors`, maps registry **role → modality contract**, executes via bound stub policy until diffusion kernels land.

`uie-serve` live profile: enable `safetensors` on `uie-iv` (`LiveLoaderSet`).

## vs HTTP scripts

`scripts/local-qwen-image-pipeline.sh` talks to Comfy/LM Studio APIs. Prefer this runbook for **in-process Spiro**.