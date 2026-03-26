---
title: Missing Fence
impact: HIGH
impactDescription: stale reads causing data corruption
tags: concurrency, fence, memory-model, lock-free
---

## Missing Fence

**Impact: HIGH (stale reads causing data corruption)**

When a memory fence is needed (e.g., between non-atomic writes and an atomic flag), omitting it allows the CPU to reorder operations, causing consumers to read partially-updated data.

**Detection:** Atomic flag patterns without corresponding `fence()` calls where non-atomic data must be visible.

**Fix:** Add appropriate fence. Verify correctness with loom (not TSAN — TSAN does not understand fence-based synchronization).
