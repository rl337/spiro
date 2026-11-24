#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
# shellcheck source=/dev/null
source "$ROOT/scripts/load-dotenv.sh"
# This script always runs the live suite; other knobs (paths, features) come from `.env`.
export UIE_ACCEPTANCE_MODE=live
FEATURES="${UIE_ACCEPTANCE_FEATURES:-gguf,live-onnx}"
export CXX="${CXX:-g++}"
export RUSTFLAGS="${RUSTFLAGS:-} -C linker=g++"
cargo test -p uie-acceptance --features "$FEATURES"
# Optional: also re-run stub suite
if [[ "${UIE_ACCEPTANCE_RUN_STUBS_IN_LIVE:-}" == "1" ]]; then
  export UIE_ACCEPTANCE_MODE=ci
  cargo test -p uie-acceptance
fi
python3 -m unittest discover -s python/acceptance -v