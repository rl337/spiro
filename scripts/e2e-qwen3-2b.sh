#!/usr/bin/env bash
# US-010: thin wrapper around the uie-e2e harness (see docs/testing/end-to-end-tests.md).
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

exec cargo run -q -p uie-e2e --features gguf -- run qwen3_2b_chat_completion -- "$@"