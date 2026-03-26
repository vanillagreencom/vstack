---
title: Release Profile Configuration
impact: MEDIUM
impactDescription: suboptimal release builds waste runtime performance or link time
tags: profile, release, lto, codegen-units, strip
---

## Release Profile Configuration

**Impact: MEDIUM (suboptimal release builds waste runtime performance or link time)**

Configure release profile for the right tradeoff: `opt-level = 3` for maximum runtime performance, `lto = "thin"` for +5-15% performance with moderate link time, `codegen-units = 1` for better optimization at the cost of slower builds, and `strip = "symbols"` for smaller binaries. `panic = "abort"` saves binary size if no `catch_unwind` is used.

| LTO Mode | Perf Gain | Link Time | Use Case |
|----------|-----------|-----------|----------|
| `false` | Baseline | Fast | Dev iteration |
| `"thin"` | +5-15% | Moderate | Default release |
| `true` / `"fat"` | +10-20% | Slow | Maximum performance |

**Incorrect (default release profile — no tuning):**

```toml
# Cargo.toml — using release defaults
[profile.release]
# opt-level = 3 (ok, default)
# lto = false (default) — missing 5-15% perf
# codegen-units = 16 (default) — prevents cross-crate optimization
# strip = false (default) — debug symbols inflate binary
```

**Correct (tuned release profile):**

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = "thin"           # +5-15% perf, moderate link time
codegen-units = 1      # Better optimization, slower build
strip = "symbols"      # Remove debug symbols from binary
panic = "abort"        # Smaller binary, no catch_unwind support

# Separate profile for profiling (needs debug symbols)
[profile.profiling]
inherits = "release"
debug = 1              # Line tables for perf/flamegraph
strip = false          # Keep symbols for profiler
```
