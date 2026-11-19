# Dependency injection requirements (uie-iv / pyiv-first)

Aligned with [rl337/project-template](https://github.com/rl337/project-template)
(`.cursor/rules/di-first.mdc`) and [MechaHarness DI requirements](https://github.com/rl337/mechaharness/blob/main/docs/requirements/dependency-injection.md),
adapted for Rust and [`uie-iv`](../../crates/uie-iv/README.md).

## Motivation

Spiro UIE hosts CI stub runs, optional live integration, and `uie-serve` on the
same code paths. Scattered `if live`, direct `std::env::var("UIE_*")`, and
hand-picked loader chains hide missing wiring, encourage `#[cfg]` forks, and make
tests brittle. **pyiv-first** means: bind CI vs live at a **composition root** via
[`uie-iv`](../../crates/uie-iv/README.md), consume **injected** profiles and
loader sets everywhere else.

Normative MUST/SHOULD below apply to library code under `crates/uie-*`, host
crates (`uie-serve`, `uie-acceptance`), and new acceptance tests. Reference:
[US-024](../stories/024-dependency-injection.md).

## Requirements

### DI-01: Modules are the binding surface

Every shared edge the engine exposes to hosts (acceptance profile, loader
multibinder, future clock/filesystem/HTTP clients) MUST be registered in a
[`Module::install`](../../crates/uie-iv/src/module.rs) implementation — typically
[`AcceptanceEnvModule`](../../crates/uie-iv/src/uie/modules.rs),
[`ServeEnvModule`](../../crates/uie-iv/src/uie/modules.rs), or a test
[`Override*Module`](../../crates/uie-iv/src/uie/modules.rs).

Hosts and acceptance entrypoints MUST build graphs with
[`Injector::from_modules`](../../crates/uie-iv/src/injector.rs) (see
[`composition::injector`](../../crates/uie-acceptance/src/composition.rs)) rather
than constructing loader chains inline.

### DI-02: Consume by injection; fail if unbound

Dependents MUST receive shared services through constructor arguments filled by
the injector, or by calling methods on an injected handle (e.g.
`injector().loaders()`, `injector().acceptance_profile()`).

If a binding is missing, resolution MUST fail when the injector is built or when
`inject` is called. Library code MUST NOT silently construct a “default live”
loader outside a module to paper over a missing binding.

### DI-03: Env and profile knobs live in modules only

`UIE_ACCEPTANCE_MODE`, `UIE_SERVE_PROFILE`, and future operator env vars that
select CI vs live MUST be read only inside `crates/uie-iv/**` (profile helpers
and env modules).

Application and test code outside `uie-iv` MUST NOT branch on those variables
directly. Prefer `injector().acceptance_profile()` or
[`skip_unless_live`](../../crates/uie-acceptance/tests/support.rs) (backed by the
injector).

### DI-04: No parallel composition roots

There MUST be a single documented composition root per process boundary:

| Boundary | Root |
|----------|------|
| Acceptance / integration tests | [`uie_acceptance::composition`](../../crates/uie-acceptance/src/composition.rs) |
| `uie-serve` | serve injector built from `ServeEnvModule` |

New binaries and test harnesses MUST NOT introduce ad hoc `OnceLock` graphs or
static loader registries outside `uie-iv`.

### DI-05: Injectable-owned configuration

Types that need operator knobs (model paths, API bases, codec options) MUST own
that configuration on the injectable (struct fields, nested config, or
`from_env()` on that type). They MUST NOT accumulate as unrelated fields on a
global “settings” struct that every crate imports.

Lane-specific loaders and backends are **module bindings**, not ambient globals.

### DI-06: Tests prefer overrides over env mutation

Unit and acceptance tests that need CI vs live doubles MUST use
`OverrideCiModule` / `OverrideLiveModule` (or a dedicated test `Module`) when
building an `Injector`, instead of mutating `UIE_ACCEPTANCE_MODE` mid-test.

Integration scripts and CI MAY set env once at process start (`acceptance-ci.sh`,
workflows); that is module input, not call-site branching.

### DI-07: Domain crates stay injector-agnostic

Core inference types (`uie-core`, `uie-ipc`, modality executors) MUST NOT depend
on `uie-iv`. Only composition roots, `uie-iv` itself, and thin host glue
(`uie-acceptance`, `uie-serve`) import the DI crate.

### DI-08: Facades hide the graph, not replace it

CLI, IPC, and Python acceptance helpers MAY hide `Injector` construction from
callers, but MUST still assemble modules and inject before running product logic.
Do not grow string-key factory registries parallel to `Module` + `Binder`.

## Verification

CI runs [`scripts/verify-di-first.py`](../../scripts/verify-di-first.py) (see
[`acceptance-ci.sh`](../../scripts/acceptance-ci.sh)).

### DI-VERIFY-01: No env branching outside uie-iv (Rust)

`std::env::var("UIE_ACCEPTANCE_MODE")` and `UIE_SERVE_PROFILE` MUST NOT appear
under `crates/**` except in `crates/uie-iv/**`.

## Testing requirements (anchors)

Existing acceptance coverage satisfies:

| ID | Intent | Anchor |
|----|--------|--------|
| DI-TEST-01 | CI profile skips live without env at test site | `us_024_ci_injector_skips_live_without_env_branch` |
| DI-TEST-02 | Override modules build injectors | `override_modules_build_injector`, `Override*Module` |
| DI-TEST-03 | Process composition root resolves | `us_024_acceptance_composition_root_resolves` |

## Related

- Guide: [`crates/uie-iv/README.md`](../../crates/uie-iv/README.md)
- Story: [US-024](../stories/024-dependency-injection.md)
- Rule: [`.cursor/rules/di-first.mdc`](../../.cursor/rules/di-first.mdc)
- Skill: [`.cursor/skills/di-config-ownership/SKILL.md`](../../.cursor/skills/di-config-ownership/SKILL.md)
- Template: [rl337/project-template — DI-first rule](https://github.com/rl337/project-template/blob/main/.cursor/rules/di-first.mdc)