---
title: Validate Latency Against Budgets
impact: HIGH
impactDescription: Performance budgets without automated validation are ignored
tags: budget, validation, percentile, regression
---

## Validate Latency Against Budgets

**Impact: HIGH (performance budgets without automated validation are ignored)**

Every performance budget should have an automated check that fails when the budget is exceeded. Use HdrHistogram percentile queries against defined targets.

```rust
use hdrhistogram::Histogram;

pub struct LatencyTargets {
    pub p50_ns: u64,
    pub p999_ns: u64,
}

pub fn validate_latency(
    measurements: &[std::time::Duration],
    targets: &LatencyTargets,
) -> Result<(), String> {
    let mut histogram: Histogram<u64> = Histogram::new(3).unwrap();
    for d in measurements {
        histogram.record(d.as_nanos() as u64).unwrap();
    }

    let p50 = histogram.value_at_percentile(50.0);
    let p999 = histogram.value_at_percentile(99.9);

    if p50 > targets.p50_ns {
        return Err(format!("P50 {}ns exceeds budget {}ns", p50, targets.p50_ns));
    }
    if p999 > targets.p999_ns {
        return Err(format!("P99.9 {}ns exceeds budget {}ns", p999, targets.p999_ns));
    }

    Ok(())
}
```
