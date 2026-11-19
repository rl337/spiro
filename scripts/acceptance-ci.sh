#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
export UIE_ACCEPTANCE_MODE="${UIE_ACCEPTANCE_MODE:-ci}"
python3 -c "import yaml" 2>/dev/null || python3 -m pip install -q pyyaml
python3 scripts/verify-story-governance.py --skip-diff
python3 scripts/verify-di-first.py
cargo test -p uie-acceptance
python3 -m unittest discover -s python/acceptance -v