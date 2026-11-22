# Qwen-Image (Comfy) → vision analysis (LM Studio)

Multi-modality **local** path: **text→image** (Comfy / US-012) then **image→text** (vision LM / US-016–017). Spiro does not embed ComfyUI; it drives the **HTTP API** and LM Studio’s **OpenAI-compatible** server.

## Important: Qwen3 2B vs vision

| Model | Role |
|-------|------|
| **Qwen-Image** (Comfy weights) | Generate PNG |
| **Qwen2.5-VL / Qwen3-VL** (LM Studio) | **Analyze** the PNG (image in chat) |
| **Qwen3 ~2B text GGUF** (`uie-llm --auto-qwen3-2b`) | Text only — **cannot** read pixels. Optional **follow-up** on the vision model’s *text* (`UIE_LLM_FOLLOWUP=1`). |

## Prerequisites

- ComfyUI running: `http://127.0.0.1:8188` (or `COMFYUI_URL`)
- Qwen-Image checkpoints under `~/ComfyUI-Shared/models` (see Comfy Qwen-Image tutorial)
- LM Studio **local server** on port **1234** with a **vision** model loaded
- `SPIRO_MODEL_ROOTS` includes `comfy-shared` + `lmstudio` (see `config/model_roots.yaml`)

```bash
export SPIRO_MODEL_ROOTS=$PWD/config/model_roots.yaml
cargo run -p uie-e2e -- list-models | rg -i 'qwen|comfy-shared'
```

## One-time Comfy API setup

1. Export API workflow → `config/comfy_workflows/qwen_image_api.json` (not committed; see `config/comfy_workflows/README.md`).
2. Create `config/comfy_workflows/qwen_image_patches.json` with your prompt node id.

## Run pipeline

```bash
export LMSTUDIO_VISION_MODEL="your-vision-model-id"   # as shown in LM Studio
export UIE_IMAGE_PROMPT="A single green triangle on white, minimal flat design."
./scripts/local-qwen-image-pipeline.sh
```

Artifacts: `/tmp/uie-artifacts/comfy_qwen_image.png` (or `UIE_ARTIFACT_DIR`).

### Optional: Qwen3-2B text follow-up

```bash
export UIE_LLM_FOLLOWUP=1
./scripts/local-qwen-image-pipeline.sh
```

Runs `uie-llm` with `--auto-qwen3-2b --chat` summarizing the vision caption (not re-reading the image).

## Manual steps

```bash
python3 scripts/comfy_qwen_image_generate.py \
  --workflow config/comfy_workflows/qwen_image_api.json \
  --patches config/comfy_workflows/qwen_image_patches.json \
  --prompt-text "Your prompt" \
  --out /tmp/uie-artifacts/out.png

python3 scripts/lmstudio_analyze_image.py \
  --image /tmp/uie-artifacts/out.png \
  --model "$LMSTUDIO_VISION_MODEL"
```

## Stories / modality map

| Step | Modality | Story |
|------|----------|-------|
| Registry scan | Comfy + LM roots | US-021 |
| Comfy generate | T2I | US-012 |
| Vision chat | Image→text / VLM | US-016 / US-017 |
| Optional uie-llm | LLM text | US-010 |

Cloud agents **cannot** complete this without your Comfy + LM Studio processes; use a **local** Cursor agent or your laptop terminal.

## Troubleshooting

- **`ComfyUI not reachable`** — start Comfy with the same port as `COMFYUI_URL`.
- **`no images in Comfy outputs`** — wrong workflow or failed GPU OOM; check Comfy console.
- **Vision API errors with Qwen3-2B** — load a VL model; text-only GGUF rejects image parts.
- **Patches / node id** — re-export API JSON after editing the graph; node ids change.