---
title: Cachegrind for Cache Simulation
impact: HIGH
impactDescription: Without per-line miss counts, optimization targets are guesswork
tags: measurement, cachegrind, valgrind, simulation, ci
---

## Cachegrind for Cache Simulation

**Impact: HIGH (without per-line miss counts, optimization targets are guesswork)**

`valgrind --tool=cachegrind ./prog` for per-line cache simulation without root/PMU access. `cg_annotate cachegrind.out.*` for source annotation. `cg_diff old.out new.out` for before/after comparison. 10-50x slower than native but gives exact miss counts per source line. Use when: no root access, CI environments, comparing optimization impact.

**Measurement commands:**

```bash
# Run cache simulation
valgrind --tool=cachegrind ./target/release/mybin

# Annotate source with miss counts
cg_annotate cachegrind.out.12345

# Compare before/after optimization
cg_diff cachegrind.out.before cachegrind.out.after | cg_annotate -
```

**Reading output:**

```text
# cg_annotate output — Dr = data reads, D1mr = L1 data read misses
#        Ir       Dr      D1mr      DLmr
# ------------------------------------------
     1,024    2,048       512        64  fn process_orders(book: &OrderBook)
       512    1,024         8         0      book.prices.iter()  // SoA: 8 misses
       512    1,024       504        64      book.orders.iter()  // AoS: 504 misses
# D1mr column identifies exact cache-miss hot spots
```
