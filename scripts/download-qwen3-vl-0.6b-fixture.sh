#!/usr/bin/env bash
# Fetch Qwen3-0.6B-Visual GGUF + mmproj for cloud/local Spiro VL smoke (≈545 MiB).
set -euo pipefail
cd "$(dirname "$0")/.."
DEST="fixtures/lmstudio/oOundefinedOo/Qwen3-0.6B-Visual-GGUF"
BASE="https://huggingface.co/oOundefinedOo/Qwen3-0.6B-Visual-GGUF/resolve/main"
mkdir -p "$DEST"
for f in Qwen3-0.6B-Visual-Q4_K_M.gguf mmproj-Qwen3-0.6B-Visual-F16.gguf; do
  echo "downloading $f ..."
  curl -fL --retry 3 -C - -o "$DEST/$f" "$BASE/$f"
done
ls -lh "$DEST"
echo "Use: export SPIRO_MODEL_ROOTS=\$PWD/config/model_roots.fixture-vl.yaml"