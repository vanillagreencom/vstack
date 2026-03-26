---
title: Collect Sufficient Samples
impact: CRITICAL
impactDescription: Too few samples make high-percentile estimates statistically meaningless
tags: sample-size, p999, statistics
---

## Collect Sufficient Samples

**Impact: CRITICAL (too few samples make high-percentile estimates statistically meaningless)**

To estimate P99.9 you need at least 10,000 measurements (1 in 1,000 events). For P99, at least 1,000. Fewer samples produce unreliable tail estimates.

**Incorrect (100 samples cannot estimate P99.9):**

```rust
for _ in 0..100 {
    measure();
}
let p999 = histogram.value_at_percentile(99.9); // Statistically meaningless
```

**Correct (10,000+ samples for P99.9):**

```rust
for _ in 0..10_000 {
    measure();
}
let p999 = histogram.value_at_percentile(99.9); // Reliable estimate
```
