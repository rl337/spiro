#!/usr/bin/env bash
# Clippy + rustfmt for PR CI (excludes native heavy optional loaders until ONNX/GGUF CI matrix exists).
set -euo pipefail
cd "$(dirname "$0")/.."

cargo fmt --all --check

EXCLUDE=(
  --exclude uie-loader-gguf
  --exclude uie-llm
  --exclude uie-loader-onnx
  --exclude uie-mnist
)
cargo clippy --workspace --all-targets "${EXCLUDE[@]}" -- -D warnings