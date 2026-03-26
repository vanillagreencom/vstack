---
title: Use HdrHistogram for Runtime Tracking
impact: HIGH
impactDescription: Ad-hoc percentile code has edge-case bugs and lacks coordinated omission support
tags: hdrhistogram, runtime, percentile, quanta
---

## Use HdrHistogram for Runtime Tracking

**Impact: HIGH (ad-hoc percentile code has edge-case bugs and lacks coordinated omission support)**

Use HdrHistogram with quanta for runtime latency tracking. HdrHistogram handles percentile calculation correctly, supports coordinated omission correction, and has constant memory usage regardless of sample count.

```rust
use quanta::Clock;
use hdrhistogram::Histogram;

pub struct LatencyTracker {
    clock: Clock,
    histogram: Histogram<u64>,
}

impl LatencyTracker {
    pub fn new() -> Self {
        Self {
            clock: Clock::new(),
            histogram: Histogram::new(5).unwrap(), // 5 significant digits
        }
    }

    pub fn measure<F, R>(&mut self, operation: F) -> R
    where F: FnOnce() -> R
    {
        let start = self.clock.raw();
        let result = operation();
        let end = self.clock.raw();
        let duration_ns = self.clock.delta(start, end);
        self.histogram.record(duration_ns).unwrap();
        result
    }

    pub fn check_budget(&self, budget_us: f64) -> bool {
        let p999_us = self.histogram.value_at_percentile(99.9) as f64 / 1000.0;
        p999_us <= budget_us
    }
}
```
