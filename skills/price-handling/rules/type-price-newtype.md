---
title: Price Newtype Pattern
impact: MEDIUM
impactDescription: Bare f64 allows accidental == usage on prices
tags: newtype, type-safety, f64, comparison
---

## Price Newtype Pattern

**Impact: MEDIUM (bare f64 allows accidental == usage on prices)**

For additional type safety, wrap f64 in a zero-overhead newtype. Intentionally omit `PartialEq` to force explicit epsilon comparison.

```rust
#[derive(Clone, Copy, Debug, PartialOrd)]
#[repr(transparent)]  // Zero-overhead newtype
pub struct Price(f64);

impl Price {
    pub const ZERO: Price = Price(0.0);

    pub fn new(value: f64) -> Self {
        debug_assert!(value.is_finite(), "Price must be finite");
        Price(value)
    }

    pub fn raw(self) -> f64 { self.0 }

    pub fn approx_eq(self, other: Price) -> bool {
        prices_equal(self.0, other.0)
    }
}

// Intentionally NO PartialEq impl -- forces explicit comparison
```

**Trade-off**: Adds friction but prevents accidental `==` usage. Consider for order types; skip for market data hot path where the overhead of wrapping/unwrapping adds noise.
