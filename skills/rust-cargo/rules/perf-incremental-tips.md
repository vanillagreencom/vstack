---
title: Incremental Build Tuning
impact: HIGH
impactDescription: suboptimal debug profile wastes 20-40% of every rebuild
tags: incremental, debug, profile, codegen-units, split-debuginfo
---

## Incremental Build Tuning

**Impact: HIGH (suboptimal debug profile wastes 20-40% of every rebuild)**

Tune `[profile.dev]` for fast incremental builds. `debug = 1` (line tables only) saves 20-40% build time over `debug = 2` (full debuginfo). `split-debuginfo = "unpacked"` on macOS avoids dsymutil bottleneck. `[profile.dev.package."*"] opt-level = 1` optimizes dependencies in debug mode without hurting own-code incremental compilation. `codegen-units = 256` maximizes dev parallelism.

**Incorrect (default dev profile with full debuginfo):**

```toml
# Cargo.toml — using defaults
[profile.dev]
# debug = 2 (default) — full DWARF, slow to generate and link
# codegen-units = 16 (default) — could be higher for dev
# Dependencies compiled at opt-level = 0 — slow runtime in tests
```

**Correct (tuned dev profile):**

```toml
# Cargo.toml
[profile.dev]
debug = 1               # Line tables only — 20-40% faster builds
codegen-units = 256      # Maximum parallelism for dev builds
incremental = true       # Explicit (default for dev, but document intent)
split-debuginfo = "unpacked"  # macOS: skip dsymutil bottleneck

# Optimize dependencies even in dev (faster test execution)
[profile.dev.package."*"]
opt-level = 1            # Light optimization for deps
                         # Own code stays at opt-level 0 for fast incremental

# CI-specific: disable incremental (no cache between runs)
# Set via environment: CARGO_INCREMENTAL=0
```
