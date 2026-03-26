---
title: Iterator Collect Allocations
impact: MEDIUM
impactDescription: .collect() allocates a new Vec on every call
tags: collect, iterator, vec, allocation
---

## Iterator Collect Allocations

**Impact: MEDIUM (.collect() allocates a new Vec on every call)**

`.collect()` creates a new collection. In hot paths, write filtered results to a preallocated buffer.

**Incorrect (allocates on every call):**

```rust
let evens: Vec<_> = data.iter().filter(|x| *x % 2 == 0).collect();
```

**Correct (write to preallocated buffer):**

```rust
let mut evens = Vec::with_capacity(data.len() / 2);
for x in data.iter().filter(|x| *x % 2 == 0) {
    evens.push(*x);
}
```

Alternatively, reuse the buffer across calls with `.clear()` before refilling.
