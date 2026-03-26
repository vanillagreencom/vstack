---
title: Acquire/Release Pattern for SPSC Queues
impact: HIGH
impactDescription: Data races on ARM64 or stale reads on weakly-ordered architectures
tags: ordering, acquire, release, spsc, arm64
---

## Acquire/Release Pattern for SPSC Queues

**Impact: HIGH (data races on ARM64 or stale reads on weakly-ordered architectures)**

SPSC queues need exactly three ordering levels: Relaxed for own-index loads, Acquire to read the other thread's progress, Release to publish data. This pattern is sufficient for correctness and avoids unnecessary memory fences.

```rust
// Producer
let tail = self.tail.load(Ordering::Relaxed);    // Own index — no sync needed
if next != self.head.load(Ordering::Acquire) {   // Read consumer's progress
    buffer[tail].write(item);
    self.tail.store(next, Ordering::Release);     // Publish data
}

// Consumer
let head = self.head.load(Ordering::Relaxed);    // Own index — no sync needed
if head != self.tail.load(Ordering::Acquire) {   // Read producer's progress
    let item = buffer[head].read();
    self.head.store(next, Ordering::Release);     // Signal slot free
}
```

**Quick reference:**

| Ordering | Use Case | Notes |
|----------|----------|-------|
| `Relaxed` | Counters, own-index loads in SPSC | No happens-before |
| `Acquire` | Load before reading shared data | Pairs with Release |
| `Release` | Store after writing shared data | Pairs with Acquire |
| `AcqRel` | Read-modify-write (CAS) | Both Acquire + Release |
| `SeqCst` | **Almost never needed** | Adds MFENCE, rarely justified |
