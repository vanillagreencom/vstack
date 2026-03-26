---
title: Subtract Timer Overhead
impact: CRITICAL
impactDescription: Timer cost inflates all measurements, especially sub-microsecond operations
tags: overhead, timer, quanta, tsc
---

## Subtract Timer Overhead

**Impact: CRITICAL (timer cost inflates all measurements, especially sub-microsecond operations)**

Every timing call has overhead (~20ns for `Instant::now()`, ~0ns for quanta with upkeep thread). For sub-microsecond operations, this overhead is significant. Measure and subtract it.

**Correct (measure and subtract overhead):**

```rust
// Measure timer overhead
let empty_start = clock.raw();
let empty_end = clock.raw();
let overhead = clock.delta(empty_start, empty_end);

// Subtract from each measurement
let actual_duration = measured_duration.saturating_sub(overhead);
```
