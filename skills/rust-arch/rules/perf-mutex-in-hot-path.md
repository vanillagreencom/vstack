---
title: Mutex in Hot Path
impact: CRITICAL
impactDescription: +10-100us per contention event
tags: performance, hot-path, concurrency, mutex
---

## Mutex in Hot Path

**Impact: CRITICAL (+10-100us per contention event)**

`Mutex<T>` or `RwLock<T>` in latency-sensitive paths (order processing, tick handling) adds contention-dependent latency that violates sub-microsecond budgets.

**Detection:** `Mutex<T>` or `RwLock<T>` used in data flow paths.

**Fix:** Lock-free alternatives — SPSC ring buffers, atomics, `ArcSwap` for read-heavy shared state.
