# US-024: Dependency injection (uie-iv / pyiv-style)

**Status:** shipped (composition root + `verify-di-first`; mode via injector)  
**Personas:** Jack, Pete  
**Crate / path:** `crates/uie-iv`, `crates/uie-acceptance/src/composition.rs`

## User story

**As** Jack, wiring loaders for CI and laptop  
**I want** a Guice/pyiv-style **composition root** instead of `if live` / `#[cfg]` at every call site  
**So that** stub vs live behavior is a binding, not a conditional.

## In plain terms

1. [`uie-iv`](../../crates/uie-iv/README.md) mirrors [rl337/pyiv](https://github.com/rl337/pyiv): `Module` → `Binder` → `Injector`.
2. `AcceptanceEnvModule` reads `UIE_ACCEPTANCE_MODE` and binds `CiAcceptanceProfile` + `StubOnlyLoaderSet`, or live profile + `LiveLoaderSet`.
3. `ServeEnvModule` reads `UIE_SERVE_PROFILE` for `uie-serve`.
4. Tests override with `OverrideCiModule` / `OverrideLiveModule` (pyiv test doubles).

## Requirements

Normative DI-first rules: [`docs/requirements/dependency-injection.md`](../requirements/dependency-injection.md) (aligned with [rl337/project-template](https://github.com/rl337/project-template) `di-first`).

## Related stories

- [US-007](./007-models-loaders.md) — loader chain
- [US-020](./020-inference-byte-channel.md) — `uie-serve` host
- [acceptance-tests.md](../acceptance-tests.md) — CI vs live modes

## Acceptance notes

- `cargo test -p uie-iv`
- `cargo test -p uie-acceptance --test us_024_dependency_injection`

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_024_dependency_injection.rs` | `cargo test -p uie-acceptance --test us_024_dependency_injection` |
| Rust | `crates/uie-iv/tests/injector_profiles.rs` | `cargo test -p uie-iv` |

## Observability

- **Events:** `ci_profile_injected`, `composition_root_ok` via `StoryEventLog`
- **CI test:** `us_024_dependency_injection.rs`