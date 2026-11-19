---
name: di-config-ownership
description: >-
  Keep operator knobs on owning injectables and uie-iv modules, not scattered env
  checks or global settings structs. Use when adding loaders, backends, serve/acceptance
  profiles, from_env wiring, or tempted to branch on UIE_ACCEPTANCE_MODE outside uie-iv.
  For uie-iv / pyiv DI-first Spiro work.
---

# DI config ownership (uie-iv)

**Global settings is not a provider bag.** Injectable types and `Module` implementations own their configuration.

## When to apply

- Adding a loader, backend, or host binary entrypoint
- Tempted to read `UIE_ACCEPTANCE_MODE` / `UIE_SERVE_PROFILE` outside `crates/uie-iv`
- Adding path/URL/credential knobs for one lane only
- Wiring a new acceptance or serve profile

## Checklist

1. **Name the owner** — loader set, profile trait object, nested config struct — not a shared catch-all struct every crate imports.
2. **Bind in a `Module`** — env is read inside `uie-iv` modules or `profile.rs` helpers; register the product type in `Binder`.
3. **Consume from the composition root** — `composition::injector()` or `Injector::from_modules` in tests; use `acceptance_profile()` / `loaders()` at call sites.
4. **Tests use overrides** — `OverrideCiModule` / `OverrideLiveModule` instead of mutating env between assertions.
5. **Keep `uie-core` free of `uie-iv`** — host glue only in `uie-acceptance`, `uie-serve`, `uie-iv`.

## Anti-patterns

```rust
// BAD — env at call site
let live = std::env::var("UIE_ACCEPTANCE_MODE").ok().as_deref() == Some("live");

// BAD — hand-wired loader vec in a binary main
let loaders: Vec<Box<dyn ModelLoader>> = vec![Box::new(StubLoader)];

// GOOD — module + injector
let inj = acceptance_injector()?;
let host = InferenceEngineHost::new(inj.loaders());
```

See `.cursor/rules/di-first.mdc` and `docs/requirements/dependency-injection.md`.