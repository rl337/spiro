#!/usr/bin/env bash
# Build uie-wasm, stage MNIST web demo, run Playwright Chromium (US-026).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WEB="$ROOT/web/mnist"
PKG="$WEB/pkg"

# Prefer a local Node install when system npm is missing (dev machines / agents).
export PATH="${HOME}/.local/node/bin:${HOME}/.cargo/bin:${PATH}"

cd "$ROOT"

if ! command -v wasm-pack >/dev/null 2>&1; then
  echo "installing wasm-pack…"
  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
  export PATH="${HOME}/.cargo/bin:${PATH}"
fi

rustup target add wasm32-unknown-unknown

echo "==> wasm-pack build (uie-wasm)"
# --no-opt: wasm-opt can emit unreachable traps that break Tract ONNX on wasm32.
wasm-pack build crates/uie-wasm \
  --target web \
  --out-dir "$PKG" \
  --release \
  --no-opt

mkdir -p "$WEB/models"
cp -f "$ROOT/models/mnist-8.onnx" "$WEB/models/mnist-8.onnx"
cp -f "$ROOT/models/fixtures/digit_one_f32.bin" "$WEB/models/digit_one_f32.bin"

echo "==> npm install + Playwright Chromium"
cd "$WEB"
if [[ ! -d node_modules ]]; then
  npm install
fi
npx playwright install --with-deps chromium

echo "==> playwright test"
npx playwright test
echo "wasm-mnist-ci: OK"
