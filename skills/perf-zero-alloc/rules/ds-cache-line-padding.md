---
title: Cache Line Padding
impact: HIGH
impactDescription: False sharing between threads destroys lock-free performance
tags: cache, padding, false-sharing, atomic, cacheline
---

## Cache Line Padding

**Impact: HIGH (false sharing between threads destroys lock-free performance)**

Use 128-byte cache padding for atomics shared across threads. Intel prefetcher pulls 64-byte pairs, so 128 bytes prevents false sharing even with adjacent prefetch.

```rust
use crossbeam::utils::CachePadded;
use std::sync::atomic::AtomicUsize;

pub struct SPSCQueue<T> {
    buffer: Box<[Option<T>]>,
    capacity: usize,
    head: CachePadded<AtomicUsize>,  // Writer only
    tail: CachePadded<AtomicUsize>,  // Reader only
}
```

**Why 128 bytes:** Head and tail are accessed by different threads. Without padding, they share a cache line, causing constant invalidation. Intel's spatial prefetcher fetches pairs of 64-byte lines, so 64-byte padding is insufficient.

### Power-of-Two Capacity

Always use power-of-two capacity for fast modulo via bitmask:

```rust
impl<T> SPSCQueue<T> {
    pub fn new(capacity: usize) -> Self {
        let capacity = capacity.next_power_of_two();
        // ...
    }

    #[inline]
    fn index(&self, pos: usize) -> usize {
        pos & (self.capacity - 1) // Fast modulo via bitmask
    }
}
```

Size ring buffers to fit within L2 cache (256-512KB typical) for optimal latency. Exceeding L2 causes measurable performance degradation.
