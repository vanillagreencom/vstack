---
title: Warm Up Before Measuring
impact: CRITICAL
impactDescription: Cold cache effects corrupt measurement data
tags: warmup, cache, cold-start, bias
---

## Warm Up Before Measuring

**Impact: CRITICAL (cold cache effects corrupt measurement data)**

The first iterations of any benchmark are always slower due to cold CPU caches, branch predictor training, and memory page faults. Include a warmup phase that is excluded from measurement.

**Incorrect (first measurement includes cold cache penalty):**

```rust
let start = Instant::now();
let result = operation();
let duration = start.elapsed();
```

**Correct (warm up before measuring):**

```rust
// Warm up (excluded from measurement)
for _ in 0..1_000 {
    black_box(operation());
}

// Measure
let mut histogram: Histogram<u64> = Histogram::new(3).unwrap();
for _ in 0..10_000 {
    let start = clock.raw();
    black_box(operation());
    let end = clock.raw();
    histogram.record(clock.delta(start, end)).unwrap();
}
```
