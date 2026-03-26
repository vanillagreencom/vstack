---
title: Guard Lifetime Contains All Access
impact: HIGH
impactDescription: Data access outside guard scope is use-after-free
tags: crossbeam, epoch, guard, lifetime
---

## Guard Lifetime Contains All Access

**Impact: HIGH (data access outside guard scope is use-after-free)**

The epoch guard's lifetime must contain all access to epoch-protected data. Never let a reference to epoch-protected data escape the guard's scope — once the guard is dropped, the referenced memory may be reclaimed.

**Incorrect (reference escapes guard scope):**

```rust
let value = {
    let guard = epoch::pin();
    shared.load(Ordering::Acquire, &guard)
}; // guard dropped — value is now dangling
```

**Correct (access within guard scope):**

```rust
let guard = epoch::pin();
let value = shared.load(Ordering::Acquire, &guard);
process(value); // guard still live
drop(guard);    // safe: value no longer used
```
