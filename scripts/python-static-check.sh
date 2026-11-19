#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

python3 -m pip install -q ruff
python3 -m ruff check python
python3 -m ruff format --check python