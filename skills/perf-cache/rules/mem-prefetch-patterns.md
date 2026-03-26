---
title: Hardware Prefetcher Patterns
impact: HIGH
impactDescription: Pointer chasing and random access defeat hardware prefetch entirely
tags: prefetch, hardware, stride, pointer-chasing, intrinsics
---

## Hardware Prefetcher Patterns

**Impact: HIGH (pointer chasing and random access defeat hardware prefetch entirely)**

Hardware prefetcher detects: sequential access, constant stride up to 2KB (Intel). Cannot detect: pointer chasing, hash table lookups, random access, variable stride. Use manual prefetch for linked structures. Prefetch distance: 2-4 cache lines ahead for sequential, 1 element ahead for linked lists. Measure: if L1 miss rate drops, prefetch is helping; if it doesn't, remove (wasted instruction).

**Incorrect (pointer chasing with no prefetch — every node is a cache miss):**

```rust
fn sum_linked_list(mut node: Option<&Node>) -> u64 {
    let mut total = 0;
    while let Some(n) = node {
        total += n.value;     // cache miss — next pointer unknown to prefetcher
        node = n.next.as_ref();
    }
    total
}
```

**Correct (manual prefetch one node ahead):**

```rust
use core::arch::x86_64::{_mm_prefetch, _MM_HINT_T0};

fn sum_linked_list(mut node: Option<&Node>) -> u64 {
    let mut total = 0;
    while let Some(n) = node {
        // Prefetch next node while processing current
        if let Some(ref next) = n.next {
            unsafe {
                _mm_prefetch(
                    (next.as_ref() as *const Node).cast::<i8>(),
                    _MM_HINT_T0,
                );
            }
        }
        total += n.value;
        node = n.next.as_ref();
    }
    total
}
```
