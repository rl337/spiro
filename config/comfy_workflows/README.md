# Comfy API workflows (local only)

1. In ComfyUI, load the official **Qwen-Image** template ([Comfy docs](https://docs.comfy.org/tutorials/image/qwen/qwen-image)).
2. Point loaders at weights under `~/ComfyUI-Shared/models` (diffusion, CLIP/text encoder, VAE).
3. **Workflow → Export (API)** → save as `qwen_image_api.json` in this directory (gitignored).
4. Open the JSON, find the node id for your positive prompt (e.g. `TextEncodeQwenImage21` or `CLIPTextEncode`).
5. Copy `qwen_image_patches.example.json` → `qwen_image_patches.json` and set that node id + input field name (`prompt`, `text`, …).

Run:

```bash
./scripts/local-qwen-image-pipeline.sh
```