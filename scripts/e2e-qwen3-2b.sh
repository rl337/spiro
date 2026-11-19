#!/usr/bin/env bash
# US-010: smoke Qwen3 ~2B GGUF end-to-end (chat template + completion).
set -euo pipefail
cd "$(dirname "$0")/.."

export SPIRO_MODEL_ROOTS="${SPIRO_MODEL_ROOTS:-$PWD/config/model_roots.yaml}"
if [[ ! -f "$SPIRO_MODEL_ROOTS" ]]; then
  echo "creating $SPIRO_MODEL_ROOTS from example"
  cp config/model_roots.example.yaml "$SPIRO_MODEL_ROOTS"
fi

export CXX="${CXX:-g++}"
export CC="${CC:-gcc}"
export RUSTFLAGS="${RUSTFLAGS:-} -C linker=g++"

./scripts/build-gguf.sh -q

PROMPT="${UIE_LLM_PROMPT:-What is 2+2? Answer in one short sentence.}"
MAX_NEW="${UIE_LLM_MAX_NEW:-64}"
EXTRA_ARGS=()
if [[ -n "${UIE_QWEN3_2B_REGISTRY_ID:-}" ]]; then
  EXTRA_ARGS+=(--registry-id "$UIE_QWEN3_2B_REGISTRY_ID")
elif [[ -n "${UIE_GGUF_MODEL:-}" ]]; then
  EXTRA_ARGS+=(--model "$UIE_GGUF_MODEL")
else
  EXTRA_ARGS+=(--auto-qwen3-2b)
fi

echo "=== list-gguf (qwen) ==="
cargo run -q -p uie-llm -- list-gguf | rg -i qwen || {
  echo "No Qwen GGUF found. Download Qwen3 2B in LM Studio under ~/.lmstudio/models"
  echo "See docs/runbooks/qwen3-2b-e2e.md"
  exit 1
}

echo "=== completion (--chat) ==="
cargo run -q -p uie-llm -- "${EXTRA_ARGS[@]}" --chat --prompt "$PROMPT" --max-new "$MAX_NEW"

echo "=== done ==="