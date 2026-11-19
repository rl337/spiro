#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
export UIE_ACCEPTANCE_MODE="${UIE_ACCEPTANCE_MODE:-ci}"
cargo test -p uie-acceptance
python3 -m unittest discover -s python/acceptance -v