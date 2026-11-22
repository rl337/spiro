# Spiro-native: Qwen-Image weights → analyze (no Comfy / LM Studio HTTP)

Use **registry discovery** + **`InferenceRuntime`** + **byte pipeline** only.

## Architecture (phase 1 — shipped)

| Step | Loader | What happens |
|------|--------|----------------|
| T2I | `SafetensorsModelLoader` | `mmap`/read Comfy checkpoint from `comfy-shared:` id; **contract infer** (US-012 shapes) |
| Image→text | `StubModelLoader` | `.stub` or future GGUF-VL |
| Text | `GgufModelLoader` | Qwen3-2B GGUF summarizes caption text (not pixels) |

Real Qwen-Image **denoise** inside Rust is US-012 follow-on; phase 1 proves **weights load through Spiro** and the multi-hop path runs.

## Local run

```bash
export SPIRO_MODEL_ROOTS=$PWD/config/model_roots.yaml
export CXX=g++ CC=gcc RUSTFLAGS="-C linker=g++"

cargo run -p uie-e2e --features spiro-chain -- list-models
cargo run -p uie-e2e --features spiro-chain -- run spiro_native_qwen_chain
```

Output: `/tmp/uie-artifacts/spiro_native_image.png` (or `--out-dir` / `UIE_ARTIFACT_DIR`).

Skip LLM hop: `--skip-llm`. Requires Qwen GGUF under lmstudio roots for full chain.

## Loader crate

`uie-loader-safetensors`: opens `.safetensors`, maps registry **role → modality contract**, executes via bound stub policy until diffusion kernels land.

`uie-serve` live profile: enable `safetensors` on `uie-iv` (`LiveLoaderSet`).

## vs HTTP scripts

`scripts/local-qwen-image-pipeline.sh` talks to Comfy/LM Studio APIs. Prefer this runbook for **in-process Spiro**.