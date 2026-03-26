---
title: Automate Regression Detection
impact: HIGH
impactDescription: Manual performance comparisons miss gradual regressions
tags: regression, baseline, ci, detection
---

## Automate Regression Detection

**Impact: HIGH (manual performance comparisons miss gradual regressions)**

Compare current benchmark results against stored baselines automatically. Flag regressions that exceed a threshold percentage on P99.9 latency. Store results as structured data (JSON) for historical tracking.

```rust
pub struct BenchmarkResult {
    pub component: String,
    pub operation: String,
    pub timestamp: u64,
    pub commit_hash: String,
    pub p50_ns: u64,
    pub p99_ns: u64,
    pub p999_ns: u64,
    pub sample_count: usize,
}

pub fn detect_regression(
    current: &BenchmarkResult,
    baseline: &BenchmarkResult,
    threshold_pct: f64,
) -> Option<String> {
    let regression_pct = ((current.p999_ns as f64 - baseline.p999_ns as f64)
        / baseline.p999_ns as f64) * 100.0;

    if regression_pct > threshold_pct {
        Some(format!(
            "{}/{}: P99.9 regressed {:.1}% ({} -> {} ns)",
            current.component, current.operation,
            regression_pct, baseline.p999_ns, current.p999_ns
        ))
    } else {
        None
    }
}
```
