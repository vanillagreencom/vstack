---
title: Build Timing Analysis
impact: HIGH
impactDescription: invisible serial bottlenecks waste minutes per build
tags: timings, profiling, bottleneck, parallelism
---

## Build Timing Analysis

**Impact: HIGH (invisible serial bottlenecks waste minutes per build)**

`cargo build --timings` generates an HTML timeline showing crate parallelism and bottlenecks. Identify: serial bottleneck crates (long bars with nothing compiling in parallel), proc-macro compilation (blocks all dependents), and underutilized CPU cores. Run periodically to catch regressions in build graph structure.

**Incorrect (blind to build bottlenecks):**

```bash
# Just build and hope it's fast
cargo build --release
# No idea which crate takes 40% of total build time
# No idea proc-macro blocks 12 downstream crates
```

**Correct (regular timing analysis):**

```bash
# Generate build timing report
cargo build --timings --release

# Opens cargo-timing.html showing:
# - Per-crate compile time as horizontal bars
# - Parallelism chart (how many crates compile simultaneously)
# - Critical path through the dependency graph
# - Codegen vs non-codegen time breakdown

# Look for:
# 1. Long bars with low parallelism = serial bottleneck
# 2. Proc-macro crates blocking many dependents
# 3. Single crate dominating total time = split candidate
```
