---
title: ThreadSanitizer and MemorySanitizer
impact: HIGH
impactDescription: TSan detects data races and MSan detects uninitialized memory reads at runtime
tags: sanitizer, tsan, msan, concurrency, memory, testing
---

## ThreadSanitizer and MemorySanitizer

**Impact: HIGH (TSan detects data races and MSan detects uninitialized memory reads at runtime)**

ThreadSanitizer (TSan) and MemorySanitizer (MSan) are runtime sanitizers that detect concurrency and initialization bugs respectively. Both require nightly Rust and `-Zbuild-std`.

**ThreadSanitizer:**

```bash
RUSTFLAGS="-Zsanitizer=thread" cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu
```

Detects data races — concurrent unsynchronized access where at least one is a write. Overhead: 5-15x slowdown, 5-10x memory increase.

**Important limitation:** TSan cannot verify `atomic::fence` correctness and gives false confidence for lock-free code. Use loom for verifying atomic ordering and fence placement. TSan is for detecting races on non-atomic data.

**MemorySanitizer:**

```bash
RUSTFLAGS="-Zsanitizer=memory" cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu
```

Detects reads of uninitialized memory. Overhead: ~3x slowdown. Tracks initialization status at the bit level.

**Incorrect (relying on TSan for lock-free correctness):**

```rust
// TSan says "no races" — but atomic ordering bugs are NOT detected
use std::sync::atomic::{AtomicBool, Ordering, fence};
static FLAG: AtomicBool = AtomicBool::new(false);
// TSan cannot verify that this fence is correctly placed or sufficient
fence(Ordering::SeqCst);
```

**Correct (TSan for data races, loom for atomics):**

```rust
// Use TSan to find races on non-atomic shared data
// Use loom to verify atomic ordering and fence correctness
#[cfg(loom)]
#[test]
fn test_atomic_ordering() {
    loom::model(|| {
        // loom exhaustively checks all interleavings
    });
}

// TSan catches this kind of bug:
// Two threads writing to the same Vec without synchronization
```
