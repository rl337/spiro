#!/usr/bin/env bash
# US-026: real browser GGUF CPU chat; requires an operator-provided Qwen3 0.6B.
set -euo pipefail
cd "$(dirname "$0")/../web/qwen"
npm ci
npx playwright install chromium
npm test
