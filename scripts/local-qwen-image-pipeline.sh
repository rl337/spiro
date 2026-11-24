#!/usr/bin/env bash
# Local multi-modality smoke: Comfy Qwen-Image → LM Studio vision chat → optional UIE text echo.
#
# Prerequisites (your Mac):
#   - ComfyUI running (default http://127.0.0.1:8188) with Qwen-Image workflow; export API JSON.
#   - LM Studio server (http://127.0.0.1:1234) with a **vision** model (Qwen2.5-VL / Qwen3-VL), not text-only Qwen3-2B.
#   - config/comfy_workflows/qwen_image_api.json (you export) + patches in config/comfy_workflows/qwen_image_patches.json
#
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
# shellcheck source=/dev/null
source "$ROOT/scripts/load-dotenv.sh"

export SPIRO_MODEL_ROOTS="${SPIRO_MODEL_ROOTS:-$PWD/config/model_roots.yaml}"
WORKFLOW="${COMFY_WORKFLOW:-$PWD/config/comfy_workflows/qwen_image_api.json}"
PATCHES="${COMFY_PATCHES:-$PWD/config/comfy_workflows/qwen_image_patches.json}"
PROMPT="${UIE_IMAGE_PROMPT:-A simple blue square on a white background, flat graphic.}"
export UIE_IMAGE_PROMPT="$PROMPT"
OUT="${UIE_ARTIFACT_DIR:-/tmp/uie-artifacts}/comfy_qwen_image.png"
QUESTION="${UIE_IMAGE_QUESTION:-Describe this image briefly. What is the main subject and dominant colors?}"

mkdir -p "$(dirname "$OUT")"

if [[ ! -f "$WORKFLOW" ]]; then
  echo "missing Comfy API workflow: $WORKFLOW"
  echo "Export from ComfyUI: Workflow → Export (API). See docs/runbooks/qwen-image-comfy-lmstudio.md"
  exit 2
fi

if [[ -f "$PATCHES" ]]; then
  export COMFY_PATCHES_JSON
  COMFY_PATCHES_JSON="$(python3 -c "
import json, os
p=json.load(open('$PATCHES'))
prompt=os.environ.get('UIE_IMAGE_PROMPT') or os.environ.get('PROMPT', '')
for n in p.values():
    if not isinstance(n, dict):
        continue
    ins=n.get('inputs') or {}
    for k,v in list(ins.items()):
        if v == 'REPLACE_VIA_SHELL_PROMPT':
            ins[k]=prompt
print(json.dumps(p))
" )"
else
  : "${COMFY_PROMPT_NODE:?set COMFY_PROMPT_NODE or provide qwen_image_patches.json}"
fi

python3 scripts/comfy_qwen_image_generate.py \
  --workflow "$WORKFLOW" \
  --prompt-text "$PROMPT" \
  --out "$OUT"

echo "--- LM Studio vision analysis ---"
VISION="$(python3 scripts/lmstudio_analyze_image.py --image "$OUT" --question "$QUESTION")"
printf '%s\n' "$VISION"
export VISION

if [[ "${UIE_LLM_FOLLOWUP:-0}" == "1" ]]; then
  echo "--- UIE Qwen3-2B text follow-up (no image; summarizes vision output) ---"
  export CXX="${CXX:-g++}" CC="${CC:-gcc}" RUSTFLAGS="${RUSTFLAGS:-} -C linker=g++"
  python3 - "$VISION" <<'PY'
import os
import subprocess
import sys

vision = sys.argv[1]
prompt = (
    "Vision model said: "
    + vision
    + "\n\nIn one sentence, restate the image content for a log entry."
)
subprocess.run(
    [
        "cargo",
        "run",
        "-q",
        "-p",
        "uie-llm",
        "--features",
        "gguf",
        "--",
        "--auto-qwen3-2b",
        "--chat",
        "--prompt",
        prompt,
    ],
    check=True,
    env=os.environ,
)
PY
fi