---
title: ThreadSanitizer Cannot Verify Atomic Fences
impact: CRITICAL
impactDescription: False confidence in lock-free correctness
tags: tsan, fence, verification, loom
---

## ThreadSanitizer Cannot Verify Atomic Fences

**Impact: CRITICAL (false confidence in lock-free correctness)**

ThreadSanitizer does NOT understand `std::sync::atomic::fence`. For any lock-free code using fences (SPSC queues, ring buffers, custom atomics), TSAN will report no errors even when bugs exist. Use loom testing instead.

TSAN is still valid for mutex-based synchronization and standard library channel primitives.

```
Is it lock-free code with atomic fences?
+-- YES -> Use Loom (TSAN won't catch issues)
+-- NO
    +-- Does it use syscalls or foreign code?
    |   +-- YES -> Use ASAN (MIRI can't test those)
    |   +-- NO  -> Use MIRI first, then ASAN
    +-- Is it mutex-based threading?
        +-- YES -> TSAN is reliable
        +-- NO  -> Evaluate case by case
```
