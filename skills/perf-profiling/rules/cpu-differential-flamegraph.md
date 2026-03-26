---
title: Differential Flamegraph Comparison
impact: HIGH
impactDescription: Before/after optimization comparison without visual diff misses regressions
tags: flamegraph, diff, comparison, regression, inferno
---

## Differential Flamegraph Comparison

**Impact: HIGH (before/after optimization comparison without visual diff misses regressions)**

Use differential flamegraphs to visually compare before/after profiles. Red frames show regressions (more time), blue frames show improvements (less time). Essential for validating that optimizations helped and nothing regressed.

```bash
# Step 1: Capture before profile
perf record -F 99 -g -- ./target/release/my_app_before
perf script > before.perf
stackcollapse-perf.pl before.perf > before.folded

# Step 2: Capture after profile (same workload, same duration, same CPU)
perf record -F 99 -g -- ./target/release/my_app_after
perf script > after.perf
stackcollapse-perf.pl after.perf > after.folded

# Step 3: Generate differential flamegraph
difffolded.pl before.folded after.folded | flamegraph.pl > diff.svg
# Red = regression (function takes more time)
# Blue = improvement (function takes less time)

# Consistent palette for visual comparison across separate flamegraphs
flamegraph.pl --cp before.folded > before.svg
flamegraph.pl --cp after.folded > after.svg
```

**Rust-native pipeline with inferno:**

```bash
# cargo flamegraph with inferno crate (no Perl dependency)
cargo install inferno
cargo install flamegraph

# Generate comparable profiles
cargo flamegraph --bin my_app -o before.svg -- <same_args>
# ... apply changes ...
cargo flamegraph --bin my_app -o after.svg -- <same_args>

# Diff with inferno
inferno-diff-folded before.folded after.folded | inferno-flamegraph > diff.svg
```

**Critical for valid comparisons:** same workload, same duration, same CPU pinning, same system load. Pin to specific cores with `taskset` to eliminate NUMA/scheduling variance.
