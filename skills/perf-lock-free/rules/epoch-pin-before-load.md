---
title: Pin Epoch Before Atomic Load
impact: HIGH
impactDescription: Use-after-free from reading reclaimed memory
tags: crossbeam, epoch, guard, pin
---

## Pin Epoch Before Atomic Load

**Impact: HIGH (use-after-free from reading reclaimed memory)**

Every atomic load from a crossbeam-epoch `Atomic<T>` must be preceded by `epoch::pin()`. The returned `Guard` keeps the current epoch alive, preventing reclamation of data you're reading. References must not escape the guard's lifetime.

**Incorrect (reference escapes guard lifetime):**

```rust
fn unsafe_read<T>(atomic: &Atomic<T>) -> Option<&T> {
    let guard = epoch::pin();
    let shared = atomic.load(Ordering::Acquire, &guard);
    // WRONG: Reference escapes guard lifetime — data may be reclaimed
    shared.as_ref()
}
```

**Correct (process within guard scope, return owned data):**

```rust
fn safe_read<T>(atomic: &Atomic<T>) -> Option<ProcessedResult> {
    let guard = epoch::pin();  // Pin BEFORE reading
    let shared = atomic.load(Ordering::Acquire, &guard);
    // Process WITHIN guard scope — return owned result
    shared.as_ref().map(|data| process(data))
    // Guard dropped here, data may be reclaimed after
}
```
