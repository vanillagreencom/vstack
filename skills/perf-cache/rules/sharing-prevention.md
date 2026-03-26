---
title: False Sharing Prevention
impact: CRITICAL
impactDescription: Unpadded cross-thread atomics on the same cache line cause 10-100x slowdown
tags: false-sharing, cache-padding, atomics, crossbeam, alignment
---

## False Sharing Prevention

**Impact: CRITICAL (unpadded cross-thread atomics on the same cache line cause 10-100x slowdown)**

Use `crossbeam_utils::CachePadded<T>` to wrap cross-thread atomics with padding to 128 bytes (not 64 — Intel adjacent-line prefetch fetches cache line pairs). Manual alternative: `#[repr(align(128))]`. Only pad cross-thread atomics — padding intra-thread data wastes cache.

**Incorrect (two atomics share a cache line — every write invalidates both):**

```rust
use std::sync::atomic::AtomicU64;

struct Counters {
    producer_count: AtomicU64, // offset 0
    consumer_count: AtomicU64, // offset 8 — same cache line!
}
// Producer writes invalidate consumer's cache line and vice versa
// Both threads constantly reload from L3 or worse
```

**Correct (each atomic gets its own cache line pair):**

```rust
use crossbeam_utils::CachePadded;
use std::sync::atomic::AtomicU64;

struct Counters {
    producer_count: CachePadded<AtomicU64>, // 128-byte aligned
    consumer_count: CachePadded<AtomicU64>, // separate cache line pair
}
// Writer threads only invalidate their own cache line
// Readers see no false contention
```
