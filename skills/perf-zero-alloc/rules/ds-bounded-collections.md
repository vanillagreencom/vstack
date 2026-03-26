---
title: Bounded Collections
impact: HIGH
impactDescription: Unbounded collections cause unpredictable heap growth and latency spikes
tags: arrayvec, stack, bounded, capacity
---

## Bounded Collections

**Impact: HIGH (unbounded collections cause unpredictable heap growth and latency spikes)**

Use `ArrayVec` for stack-allocated, fixed-capacity collections when the maximum size is known at compile time.

```rust
use arrayvec::ArrayVec;

pub struct BoundedBuffer<T, const N: usize> {
    items: ArrayVec<T, N>,
}

impl<T, const N: usize> BoundedBuffer<T, N> {
    pub fn new() -> Self {
        Self { items: ArrayVec::new() }
    }

    pub fn push(&mut self, item: T) -> Result<(), CapacityError> {
        self.items.try_push(item).map_err(|_| CapacityError)
    }

    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        self.items.drain(..)
    }
}
```

**When to use ArrayVec:**
- Known maximum size at compile time
- Total size < 10KB (stack limit consideration)
- Frequent creation/destruction where heap allocation is unacceptable

**When to use preallocated Vec instead:**
- Maximum size known only at runtime
- Size may exceed stack limits
