# Requirements

Normative engineering constraints for Spiro UIE. User-facing capabilities remain in
[`docs/stories/`](../stories/README.md); these docs capture **how we build** the engine.
Product goal (any-scale; edge as compromise direction): [`docs/mandate.md`](../mandate.md).

| Doc | Scope |
|-----|--------|
| [../mandate.md](../mandate.md) | **Product mandate** — any-scale inference; compromise toward edge/embedded |
| [dependency-injection.md](./dependency-injection.md) | **DI-first (pyiv / uie-iv)** — composition root, modules, env binding |
| [validation-and-ci.md](./validation-and-ci.md) | Static checks, acceptance CI, Bugbot gate |

Agents: read relevant requirement docs before adding shared services, loaders, or CI vs live wiring.
See also `.cursor/rules/di-first.mdc` and `.cursor/skills/di-config-ownership/SKILL.md`.