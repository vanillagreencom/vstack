---
title: Measure Percentiles, Not Averages
impact: CRITICAL
impactDescription: Averages hide tail latency that causes user-visible stalls
tags: percentile, p99, hdrhistogram, statistics
---

## Measure Percentiles, Not Averages

**Impact: CRITICAL (averages hide tail latency that causes user-visible stalls)**

Averages mask tail latency spikes. A system averaging 100us can have P99.9 at 50ms. Always report P50, P95, P99, and P99.9. Use HdrHistogram for correct percentile calculation.

**Incorrect (average hides tail latency):**

```rust
let avg = measurements.iter().sum::<u64>() / measurements.len() as u64;
println!("Average latency: {}ns", avg);
```

**Correct (report percentiles):**

```rust
use hdrhistogram::Histogram;

let mut histogram: Histogram<u64> = Histogram::new(3).unwrap();
for &m in &measurements {
    histogram.record(m).unwrap();
}
println!("P50:   {}ns", histogram.value_at_percentile(50.0));
println!("P99:   {}ns", histogram.value_at_percentile(99.0));
println!("P99.9: {}ns", histogram.value_at_percentile(99.9));
```
