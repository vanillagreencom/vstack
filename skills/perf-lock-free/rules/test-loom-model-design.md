---
title: Loom Model Design Best Practices
impact: MEDIUM
impactDescription: State space explosion or insufficient coverage from poor test design
tags: loom, testing, model, interleaving
---

## Loom Model Design Best Practices

**Impact: MEDIUM (state space explosion or insufficient coverage from poor test design)**

Loom explores all thread interleavings, which grows exponentially. Design models carefully to keep the state space tractable while still covering critical properties.

**Best practices:**

1. **Always use `loom::thread::yield_now()`** in spin/retry loops — loom needs explicit yield points to explore interleavings
2. **Keep models small** — loom explores all interleavings exponentially; use small buffer sizes (e.g., 4 slots) to force edge cases within exploration budget
3. **Test one property per model** — easier to debug failures and limits state space
4. **Use `LOOM_LOG=trace`** for debugging failed models
5. **Test on ARM64** (Apple Silicon) — x86's strong ordering hides weak-memory bugs that loom's model may not cover

**Key properties to test:**

- **Wraparound**: Head/tail indices wrap around ring buffer capacity boundary
- **Sequential ordering**: FIFO preserved — push N items, pop N, assert order matches
- **Multi-variant payloads**: Enum payloads (different sizes, discriminants) transit correctly through ordering boundary

**SPSC model skeleton:**

```rust
use loom::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use loom::sync::Arc;
use loom::thread;

#[test]
fn loom_spsc_ordering() {
    loom::model(|| {
        let head = Arc::new(AtomicUsize::new(0));
        let tail = Arc::new(AtomicUsize::new(0));
        let data: Arc<[AtomicU64; 4]> = Arc::new(
            std::array::from_fn(|_| AtomicU64::new(0))
        );

        // Producer: write data, Release-store tail
        // Consumer: Acquire-load tail, read data, Release-store head
    });
}
```

Loom cannot intercept third-party crate internals (e.g., `rtrb`). Test the Acquire/Release ordering pattern through simplified in-memory models that mirror the real implementation's synchronization.
