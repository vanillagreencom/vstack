---
title: f64 for All Prices
impact: CRITICAL
impactDescription: Wrong numeric type causes unnecessary complexity or precision bugs
tags: f64, price, numeric-type
---

## f64 for All Prices

**Impact: CRITICAL (wrong numeric type causes unnecessary complexity or precision bugs)**

IEEE 754 double-precision (`f64`) is the standard price type. No fixed-point, no decimal types.

Rationale: Industry standard (MT5, NinjaTrader, MultiCharts), 15-17 significant digits, SIMD-friendly, zero-overhead newtypes possible.

Migrate to `i64` fixed-point **only if**:
- Building a matching engine (bit-exact required)
- Regulatory audit trail mandates reproducibility
- Settlement system with legal precision requirements

Hybrid approach: `i64` for execution hot path, `f64` for display/analytics.
