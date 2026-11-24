#!/usr/bin/env bash
# US-026: real browser GGUF CPU chat; requires an operator-provided Qwen3 0.6B.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
# shellcheck source=/dev/null
source "$ROOT/scripts/load-dotenv.sh"
cd "$ROOT/web/qwen"
npm ci
npx playwright install chromium
npm test
