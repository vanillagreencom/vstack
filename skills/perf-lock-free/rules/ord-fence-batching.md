---
title: Fence Batching for Multiple Stores
impact: HIGH
tags: fence, ordering, release, relaxed, optimization, batching
---

## Fence Batching for Multiple Stores

**Impact: HIGH (unnecessary per-store Release ordering adds barrier overhead)**

When publishing multiple related values, use Relaxed stores followed by a single `fence(Release)` and a Relaxed sentinel store, instead of making every store Release. This is semantically equivalent but can be more efficient — one barrier instead of N.

**Incorrect (per-store Release — redundant barriers):**

```rust
use std::sync::atomic::{AtomicU64, Ordering};

static DATA_A: AtomicU64 = AtomicU64::new(0);
static DATA_B: AtomicU64 = AtomicU64::new(0);
static DATA_C: AtomicU64 = AtomicU64::new(0);
static READY: AtomicU64 = AtomicU64::new(0);

// Publisher: 3 Release stores + 1 Release store = 4 barriers on ARM64
fn publish(a: u64, b: u64, c: u64) {
    DATA_A.store(a, Ordering::Release);
    DATA_B.store(b, Ordering::Release);
    DATA_C.store(c, Ordering::Release);
    READY.store(1, Ordering::Release);
}
```

**Correct (fence batching — single barrier):**

```rust
use std::sync::atomic::{self, AtomicU64, Ordering};

static DATA_A: AtomicU64 = AtomicU64::new(0);
static DATA_B: AtomicU64 = AtomicU64::new(0);
static DATA_C: AtomicU64 = AtomicU64::new(0);
static READY: AtomicU64 = AtomicU64::new(0);

// Publisher: 3 Relaxed stores + 1 fence + 1 Relaxed sentinel = 1 barrier
fn publish(a: u64, b: u64, c: u64) {
    DATA_A.store(a, Ordering::Relaxed);
    DATA_B.store(b, Ordering::Relaxed);
    DATA_C.store(c, Ordering::Relaxed);
    atomic::fence(Ordering::Release);
    READY.store(1, Ordering::Relaxed);
}

// Consumer: Acquire fence after sentinel check
fn consume() -> Option<(u64, u64, u64)> {
    if READY.load(Ordering::Relaxed) == 1 {
        atomic::fence(Ordering::Acquire);
        Some((
            DATA_A.load(Ordering::Relaxed),
            DATA_B.load(Ordering::Relaxed),
            DATA_C.load(Ordering::Relaxed),
        ))
    } else {
        None
    }
}
```

The fence ensures all prior Relaxed stores are visible to any thread that observes the sentinel via an Acquire fence. On x86 this has no measurable difference (TSO makes Release free), but on ARM64 it reduces N `dmb ish` barriers to 1.

**When to use:** Batched updates (multiple fields published together), ring buffer metadata updates, snapshot publishing. **When NOT to use:** Single publish (just use Release directly), unclear ordering requirements (prefer explicit per-variable ordering for clarity).

**Requirement:** Every fence-based pattern must have loom test coverage per `verify-tsan-no-fences` — TSAN cannot verify fences.
