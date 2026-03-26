---
title: Static Dispatch Over Dynamic
impact: CRITICAL
impactDescription: Dynamic dispatch prevents inlining and adds vtable lookup overhead in hot paths
tags: dispatch, generics, dyn, box, closure
---

## Static Dispatch Over Dynamic

**Impact: CRITICAL (dynamic dispatch prevents inlining and adds vtable lookup overhead in hot paths)**

Use generics for static dispatch in hot paths. Reserve `dyn` and `Box<dyn ...>` for cold paths.

**Incorrect (dynamic dispatch with vtable overhead):**

```rust
pub fn process_feed(handler: &dyn FeedHandler, data: &[u8]) {
    handler.parse(data); // Runtime dispatch via vtable
}

pub fn map_transform(data: &mut [f64], f: Box<dyn Fn(f64) -> f64>) {
    for x in data { *x = f(*x); } // Heap-allocated closure
}
```

**Correct (static dispatch, zero overhead):**

```rust
pub fn process_feed<F: FeedHandler>(handler: &F, data: &[u8]) {
    handler.parse(data); // Compile-time dispatch, inlinable
}

pub fn map_transform<F: Fn(f64) -> f64>(data: &mut [f64], f: F) {
    for x in data { *x = f(*x); } // Stack-allocated closure
}
```

**When dynamic dispatch is acceptable:**
- Cold paths (configuration, setup)
- Plugin systems where extensibility matters
- Trait objects stored long-term (one allocation, many uses)
