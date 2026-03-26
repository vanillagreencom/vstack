---
title: Use Hardware Timestamps
impact: CRITICAL
impactDescription: Software timers lack precision for nanosecond-scale measurement
tags: tsc, rdtsc, quanta, clock, platform
---

## Use Hardware Timestamps

**Impact: CRITICAL (software timers lack precision for nanosecond-scale measurement)**

Use TSC/RDTSC via the `quanta` crate for nanosecond precision. `std::time::Instant` uses OS syscalls with higher overhead (~20ns vs ~0ns with quanta upkeep thread).

**Platform behavior:**
| Platform | Best Timer |
|----------|------------|
| Linux | `CLOCK_MONOTONIC_RAW` (via quanta TSC) |
| Windows | `QueryPerformanceCounter` (via quanta) |
| macOS | `mach_absolute_time` (via quanta) |

**Notes:**
- TSC is x86/x86_64 only; quanta falls back to stdlib on ARM
- First `Clock::new()` blocks ~10ms for TSC calibration -- create once at startup
- Use `Clock::recent()` with upkeep thread for ultra-low overhead reads

```rust
use quanta::Clock;

let clock = Clock::new(); // Create once, reuse
let start = clock.raw();
operation();
let end = clock.raw();
let duration_ns = clock.delta(start, end);
```
