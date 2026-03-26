---
title: Vec Push Beyond Capacity
impact: MEDIUM
impactDescription: Vec::push may silently reallocate when capacity is exhausted
tags: vec, push, capacity, reallocation
---

## Vec Push Beyond Capacity

**Impact: MEDIUM (Vec::push may silently reallocate when capacity is exhausted)**

`Vec::push()` will reallocate if `len == capacity`. In hot paths, always verify remaining capacity or use bounded alternatives.

**Incorrect (may reallocate):**

```rust
vec.push(item);
```

**Correct (verify capacity first):**

```rust
assert!(vec.len() < vec.capacity());
vec.push(item);
```

For production hot paths, prefer `ArrayVec::try_push()` which returns an error instead of reallocating, or pre-size the Vec and guard against overflow.
