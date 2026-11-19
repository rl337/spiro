#!/usr/bin/env bash
# llama-cpp-sys builds C++ with clang by default; use g++ on Linux dev boxes.
set -euo pipefail
cd "$(dirname "$0")/.."
export CXX="${CXX:-g++}"
export CC="${CC:-gcc}"
export RUSTFLAGS="${RUSTFLAGS:-} -C linker=g++"
cargo build -p uie-loader-gguf -p uie-llm "$@"