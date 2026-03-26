---
title: Tick-Size Rounding
impact: CRITICAL
impactDescription: Incorrect rounding submits invalid prices or destroys feed precision
tags: tick-size, rounding, validation
---

## Tick-Size Rounding

**Impact: CRITICAL (incorrect rounding submits invalid prices or destroys feed precision)**

Round prices to valid tick increments. Validate alignment after rounding.

```rust
pub fn round_to_tick(price: f64, tick_size: f64) -> f64 {
    (price / tick_size).round() * tick_size
}

pub fn validate_tick_alignment(price: f64, tick_size: f64) -> bool {
    let rounded = round_to_tick(price, tick_size);
    prices_equal(price, rounded)
}
```

Where rounding applies is governed by the boundary rules (`boundary-*`).
