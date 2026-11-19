# Bugbot review hints (Spiro UIE)

- Map changes to `US-0NN` user stories; call out missing governance or observability assertions.
- **DI-first:** env/profile switches belong in `crates/uie-iv/**`; flag `UIE_ACCEPTANCE_MODE` branches elsewhere.
- Acceptance tests must assert telemetry via `StoryEventLog`, `assert_phases`, or stub telemetry helpers.
- Prefer injected composition (`uie_acceptance::composition`) over hand-wired loaders.
- Do not expand `Settings`-style global config bags; injectable-owned config only.