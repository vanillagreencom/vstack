---
title: Use iai-callgrind for Deterministic CI
impact: HIGH
impactDescription: Time-based benchmarks are noisy in CI; instruction counts are deterministic
tags: iai-callgrind, ci, regression, deterministic
---

## Use iai-callgrind for Deterministic CI

**Impact: HIGH (time-based benchmarks are noisy in CI; instruction counts are deterministic)**

iai-callgrind counts CPU instructions instead of wall-clock time, making it immune to CI environment noise (shared runners, CPU throttling, load spikes). Use it for automated regression gates.

```bash
# In CI (Linux runners only -- requires valgrind)
cargo bench --bench iai_benchmarks --features iai -- --regress ir::count=0%
```

Time-based benchmarks (Criterion) should run nightly on dedicated hardware as advisory, non-blocking checks. iai-callgrind gates block the merge.
