---
title: Object Pools
impact: CRITICAL
impactDescription: Hot-path allocations cause latency spikes from allocator contention
tags: pool, slab, free-list, reuse
---

## Object Pools

**Impact: CRITICAL (hot-path allocations cause latency spikes from allocator contention)**

Pre-allocate pools of reusable objects at startup. Acquire/release in the hot path without heap allocation.

### Slab Pool (recommended for lifecycle-managed objects)

`slab` provides pre-allocated arena storage with stable keys:

```rust
use slab::Slab;

pub struct Pool<T: Default> {
    pool: Slab<T>,
    max_capacity: usize,
}

impl<T: Default> Pool<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            pool: Slab::with_capacity(capacity),
            max_capacity: capacity,
        }
    }

    #[inline]
    pub fn create(&mut self) -> Option<usize> {
        if self.pool.len() < self.max_capacity {
            Some(self.pool.insert(T::default()))
        } else {
            None // Pool exhausted
        }
    }

    #[inline]
    pub fn get(&self, key: usize) -> Option<&T> {
        self.pool.get(key)
    }

    #[inline]
    pub fn release(&mut self, key: usize) {
        self.pool.remove(key);
    }
}
```

Slab keys (usize) are opaque handles, not raw pointers.

### Free-List Pool (simpler, no lifecycle management)

```rust
pub struct ObjectPool<T> {
    objects: Vec<T>,
    free_indices: Vec<usize>,
}

impl<T: Default + Clone> ObjectPool<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            objects: (0..capacity).map(|_| T::default()).collect(),
            free_indices: (0..capacity).rev().collect(),
        }
    }

    #[inline]
    pub fn acquire(&mut self) -> Option<(usize, &mut T)> {
        self.free_indices.pop().map(|idx| (idx, &mut self.objects[idx]))
    }

    #[inline]
    pub fn release(&mut self, idx: usize) {
        self.free_indices.push(idx);
    }
}
```

### Thread-Safe Pool (SPSC ring buffer)

```rust
use ringbuf::{HeapRb, Producer, Consumer};
use std::sync::Arc;

pub struct ThreadSafePool<T> {
    producer: Producer<T, Arc<HeapRb<T>>>,
    consumer: Consumer<T, Arc<HeapRb<T>>>,
}

impl<T: Default> ThreadSafePool<T> {
    pub fn new(capacity: usize) -> Self {
        let rb = HeapRb::new(capacity);
        let (mut producer, consumer) = rb.split();
        for _ in 0..capacity {
            producer.try_push(T::default()).expect("Pool init failed");
        }
        Self { producer, consumer }
    }

    #[inline]
    pub fn acquire(&mut self) -> Option<T> {
        self.consumer.try_pop()
    }

    #[inline]
    pub fn release(&mut self, obj: T) {
        let _ = self.producer.try_push(obj);
    }
}
```
