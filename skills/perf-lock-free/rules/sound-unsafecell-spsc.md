---
title: UnsafeCell Required for SPSC Buffers
impact: CRITICAL
impactDescription: Undefined behavior from mutating through shared references
tags: unsafecell, spsc, undefined_behavior, miri
---

## UnsafeCell Required for SPSC Buffers

**Impact: CRITICAL (undefined behavior from mutating through shared references)**

Casting `&T` to `*mut T` and mutating is undefined behavior. The only sound way to implement interior mutability in SPSC queue buffers is through `UnsafeCell`. MIRI will catch this violation.

For in-process Rust SPSC, prefer `rtrb` crate directly (battle-tested, MIRI-verified) over hand-rolling.

**Incorrect (mutating through shared reference is UB):**

```rust
buffer: Box<[CachePadded<Option<T>>]>,
unsafe {
    let slot = &self.buffer[idx] as *const _ as *mut Option<T>;
    (*slot) = Some(item);  // UB: mutating through shared reference
}
```

**Correct (UnsafeCell opts out of immutability guarantee):**

```rust
buffer: Box<[UnsafeCell<MaybeUninit<T>>]>,
unsafe {
    (*self.buffer[idx].get()).write(item);  // UnsafeCell::get() -> *mut T
}
```
