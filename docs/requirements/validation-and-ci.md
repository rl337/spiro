# Validation and CI

## Motivation

PR quality should lean on **static analysis** (fast, deterministic) before long acceptance runs.

## Requirements

### VAL-01: Rust static checks on every PR

Rust PRs MUST pass `cargo fmt --all --check` and `cargo clippy` with `-D warnings` via
[`scripts/rust-static-check.sh`](../../scripts/rust-static-check.sh) (see
[`.github/workflows/static-checks.yml`](../../.github/workflows/static-checks.yml)).

Default `clippy` excludes optional native crates (`uie-loader-onnx`, `uie-loader-gguf`,
`uie-llm`, `uie-mnist`). [`native-compile.yml`](../../.github/workflows/native-compile.yml)
runs `cargo check` on those crates on every PR.

### VAL-02: Python static checks on every PR

Python under `python/` MUST pass `ruff check` and `ruff format --check` via
[`scripts/python-static-check.sh`](../../scripts/python-static-check.sh)
(config: [`pyproject.toml`](../../pyproject.toml)).

### VAL-03: Acceptance and governance

[`scripts/acceptance-ci.sh`](../../scripts/acceptance-ci.sh) runs story governance,
DI-first lint, and `cargo test -p uie-acceptance`.

### VAL-04: Bugbot

Bugbot MUST be enabled for the repository (Cursor GitHub App). PR CI runs
[`bugbot-gate.yml`](../../.github/workflows/bugbot-gate.yml) to wait for the
**Cursor Bugbot** check when it appears. Hints: [`.cursor/BUGBOT.md`](../../.cursor/BUGBOT.md).

The gate is **advisory by default**: if the App never posts a check (oversized diff,
delay, etc.), the workflow warns and passes. Set GitHub repo variable
`BUGBOT_GATE_STRICT=true` to fail on timeout. Completed runs with findings use
check conclusion `neutral` (gate passes); only `failure` fails the job.

## Related

- [contributing.md](../contributing.md)