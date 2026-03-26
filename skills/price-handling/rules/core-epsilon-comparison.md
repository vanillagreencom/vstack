---
title: Epsilon Comparison
impact: CRITICAL
impactDescription: Direct f64 equality causes false negatives on price matches
tags: f64, comparison, epsilon, float_cmp
---

## Epsilon Comparison

**Impact: CRITICAL (direct f64 equality causes false negatives on price matches)**

f64 comparison bugs are the #1 source of price-related issues. Never use `==` on prices.

**Incorrect (direct equality):**

```rust
if price == target {
    execute_order();
}
```

**Correct (epsilon comparison):**

```rust
use float_cmp::{approx_eq, F64Margin};

pub const PRICE_EPSILON: f64 = 1e-10;  // Sub-pipette tolerance

pub fn prices_equal(a: f64, b: f64) -> bool {
    approx_eq!(f64, a, b, epsilon = PRICE_EPSILON, ulps = 4)
}

pub fn price_gte(a: f64, b: f64) -> bool {
    a > b || prices_equal(a, b)
}

pub fn price_lte(a: f64, b: f64) -> bool {
    a < b || prices_equal(a, b)
}
```
