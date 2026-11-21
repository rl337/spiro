#!/usr/bin/env bash
# Ephemeral cloud/laptop bootstrap: Comfy-shaped tree + tiny HF artifact for list-models.
# Does not install ComfyUI; Spiro only needs the directory layout for scan + stub infer.
set -euo pipefail
cd "$(dirname "$0")/.."

COMFY_ROOT="${COMFY_UI_SHARED:-$HOME/ComfyUI-Shared/models}"
export COMFY_ROOT
mkdir -p "$COMFY_ROOT"/{checkpoints,vae,text_encoders,loras}

if python3 -c "import huggingface_hub" 2>/dev/null; then
  python3 <<'PY'
from pathlib import Path
import os
from huggingface_hub import hf_hub_download

root = Path(os.environ["COMFY_ROOT"])
hf_hub_download(
    repo_id="hf-internal-testing/tiny-random-vit",
    filename="model.safetensors",
    local_dir=str(root / "checkpoints"),
    local_dir_use_symlinks=False,
)
src = root / "checkpoints" / "model.safetensors"
dst = root / "checkpoints" / "tiny_random_vit.safetensors"
if src.exists():
    src.replace(dst)
    print("wrote", dst)
PY
else
  echo "optional: pip install huggingface_hub for tiny checkpoint download"
fi

chmod +x scripts/bootstrap-cloud-live.sh 2>/dev/null || true
export SPIRO_MODEL_ROOTS="${SPIRO_MODEL_ROOTS:-$PWD/config/model_roots.yaml}"
echo "SPIRO_MODEL_ROOTS=$SPIRO_MODEL_ROOTS"
echo "Comfy tree: $COMFY_ROOT"
echo "Run: cargo run -p uie-e2e -- list-models"
echo "Run: cargo run -p uie-e2e -- run modality_live_smoke"