# uie-iv

Guice / [pyiv](https://github.com/rl337/pyiv)-style dependency injection for Spiro UIE.

pyiv uses `Config.configure()` + `Injector.inject(Type)`. **uie-iv** uses Rust traits:

| pyiv | uie-iv |
|------|--------|
| `Config` | [`Module`](src/module.rs) |
| `Binder.register` | [`Binder::bind_*`](src/binder.rs) |
| `get_injector(Config)` | [`Injector::from_modules`](src/injector.rs) |
| `Named("ci")` / test doubles | [`CiAcceptanceProfile`](src/uie/profile.rs) vs [`LiveAcceptanceProfile`](src/uie/profile.rs) |
| `MemoryFilesystem` vs `RealFilesystem` | [`StubOnlyLoaderSet`](src/uie/loaders.rs) vs [`LiveLoaderSet`](src/uie/loaders.rs) |

## Environment modules

| Variable | Module | Binds |
|----------|--------|--------|
| `UIE_ACCEPTANCE_MODE=ci\|live` | `AcceptanceEnvModule` | acceptance profile + loader chain |
| `UIE_SERVE_PROFILE=stub\|live` | `ServeEnvModule` | loaders for `uie-serve` |

## Example

```rust
use uie_iv::acceptance_injector;
use uie_ipc::InferenceEngineHost;

let inj = acceptance_injector()?;
if inj.acceptance_profile().skip_live_test("live_us_009") {
    return;
}
let host = InferenceEngineHost::new(inj.loaders());
```

Override in unit tests with `OverrideLiveModule` / `OverrideCiModule` instead of mutating `UIE_ACCEPTANCE_MODE`.