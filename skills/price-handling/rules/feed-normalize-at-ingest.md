---
title: Normalize to f64 at Ingest
impact: MEDIUM
impactDescription: Late conversion scatters parsing logic and risks precision loss in the wrong layer
tags: feed, parsing, normalization, ingest
---

## Normalize to f64 at Ingest

**Impact: MEDIUM (late conversion scatters parsing logic and risks precision loss in the wrong layer)**

Convert all feed data to f64 at the ingest boundary. Different feeds use different wire formats; normalize once at entry.

```rust
// Feeds sending doubles (IB, dxFeed, Rithmic): pass through
fn ingest_double(value: f64) -> f64 { value }

// Feeds sending strings (Binance, Coinbase): parse
fn ingest_string(s: &str) -> Result<f64, ParseFloatError> {
    s.parse()
}

// Feeds sending scaled integers (CME MDP): unscale
fn ingest_scaled(mantissa: i64, exponent: i8) -> f64 {
    mantissa as f64 * 10f64.powi(exponent as i32)
}
```

After normalization, all downstream code works with plain `f64` regardless of feed source.
