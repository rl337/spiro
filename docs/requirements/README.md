# Requirements

Normative engineering constraints for Spiro UIE. User-facing capabilities remain in
[`docs/stories/`](../stories/README.md); these docs capture **how we build** the engine.

| Doc | Scope |
|-----|--------|
| [dependency-injection.md](./dependency-injection.md) | **DI-first (pyiv / uie-iv)** — composition root, modules, env binding |

Agents: read relevant requirement docs before adding shared services, loaders, or CI vs live wiring.
See also `.cursor/rules/di-first.mdc` and `.cursor/skills/di-config-ownership/SKILL.md`.