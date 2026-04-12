# Maintenance Guide

## Architecture

```
cuda-grimoire
├── src/
│   ├── lib.rs          — re-exports
│   ├── pattern.rs      — AgentPattern, PatternCategory
│   ├── spellbook.rs    — Spellbook (per-vessel pattern store)
│   ├── recipe.rs       — built_in_recipes() — 8 starter patterns
│   └── compendium.rs   — FleetCompendium (cross-vessel registry)
└── tests/
    └── integration.rs  — 17 tests covering all modules
```

### Data Flow

1. `built_in_recipes()` returns 8 starter `AgentPattern` values with real flux-ese source.
2. A vessel creates a `Spellbook`, adds built-in recipes plus custom patterns.
3. `FleetCompendium` aggregates spellbooks from multiple vessels for cross-vessel search.

## Why Patterns, Not Code Generation

Patterns carry *intent*. Generated code is static — it works for the case it was generated for, then rots. A pattern like "trust_gate" encodes the *idea* (check trust before processing), with enough structure to be executable but enough flexibility to adapt.

When a vessel's environment changes — new threat models, different energy constraints — a pattern can be reinterpreted. Generated code just breaks.

## Recipe Design Philosophy

Each built-in recipe follows these principles:

1. **Real flux-ese source**: Not pseudocode. Actual compilable FLUX ESE that a vessel can execute.
2. **Energy awareness**: Every recipe has an honest energy cost. Cheap patterns (reflex_dodge: 2) coexist with expensive ones (delegate_and_reduce: 8).
3. **Trust boundaries**: Patterns that interact with other vessels specify minimum trust thresholds.
4. **Confidence sensitivity**: Patterns that depend on accurate self-assessment are marked `confidence_sensitive`.

## Testing

Run all tests: `cargo test`

Tests cover: add/get, category listing, search, ranking (by usage and reliability), energy filtering, success rate updates, recipe properties, and compendium operations.

## Future: Pattern Evolution

The `times_used` and `success_rate` fields are the foundation. Future work:

- **Adaptive patterns**: Patterns whose parameters adjust based on historical success (e.g., auto-tuning trust thresholds).
- **Pattern composition**: Combining smaller patterns into larger workflows.
- **Fleet learning**: Aggregating success rates across vessels to identify universally effective vs. context-dependent patterns.
- **Pattern pruning**: Automatically deprecating patterns whose fleet-wide success rate drops below a threshold.
