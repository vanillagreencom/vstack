---
title: Heap Allocation in Hot Path
impact: CRITICAL
impactDescription: +50-500ns per allocation
tags: performance, hot-path, allocation, heap
---

## Heap Allocation in Hot Path

**Impact: CRITICAL (+50-500ns per allocation)**

`Vec::new()`, `Box::new()`, `String::from()` in hot paths cause allocator contention and unpredictable latency spikes from system allocator calls.

**Detection:** `Vec::new()`, `Box::new()`, `String::new()`, `format!()` in latency-sensitive code paths.

**Fix:** Pre-allocate at startup. Use object pools, bounded ring buffers, or stack-allocated arrays. All collections should be created with known capacity during initialization.
