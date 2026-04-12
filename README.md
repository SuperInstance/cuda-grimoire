# cuda-grimoire 📖✨

*A spellbook for the fleet — proven patterns, not reinvention.*

`cuda-grimoire` is a structured library of reusable patterns, idioms, and battle-tested recipes for FLUX agent development. A grimoire is a book of spells. This is a book of patterns.

## Why a Spellbook?

Every vessel in the fleet faces the same fundamental problems: trust checks, energy management, delegation, memory consultation. Each new vessel could figure these out from scratch, or it could inherit the hard-won wisdom of every vessel that came before.

**Proven patterns > reinventing the wheel.**

Patterns adapt. Unlike code generation, which produces static output that rots, patterns carry intent and context. They evolve with use — their success rates track real outcomes, and they can be refined based on fleet-wide experience.

## Use Cases

- **New vessel bootstrapping**: A freshly spawned vessel loads the built-in recipes and immediately knows how to ask safely, conserve energy, and trust-gate messages.
- **Cross-vessel pattern sharing**: The `FleetCompendium` lets vessels discover what patterns others have found useful, spreading innovation across the fleet.
- **Fleet-wide optimization**: Track which patterns are most used and most reliable across all vessels. Retire what doesn't work. Double down on what does.

## Modules

- **`pattern`** — `AgentPattern` struct and `PatternCategory` enum
- **`spellbook`** — Per-vessel pattern storage with search, ranking, and energy filtering
- **`recipe`** — 8 built-in recipes every vessel should know
- **`compendium`** — Fleet-wide pattern registry for cross-vessel discovery

## Related Crates

- [`cuda-flux-ese-stdlib`](https://crates.io/crates/cuda-flux-ese-stdlib) — Standard library for the FLUX ESE language
- [`cuda-telepathy`](https://crates.io/crates/cuda-telepathy) — A2A communication layer
- [`cuda-trust`](https://crates.io/crates/cuda-trust) — Trust scoring and reputation system

## Build & Test

```bash
cargo build
cargo test
```

## The Deeper Connection

A grimoire is personal. No two mages carry the same spells — they annotate, adapt, and compose from their own experience. `cuda-grimoire` honors this: every vessel builds its own spellbook from recipes and discovery, and the fleet compendium is the shared library where wisdom flows between vessels. The patterns aren't prescriptions. They're starting points. The real magic is what each vessel makes of them.
