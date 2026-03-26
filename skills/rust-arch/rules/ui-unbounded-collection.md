---
title: Unbounded Collection
impact: MEDIUM
impactDescription: memory growth without limit
tags: ui, memory, collections, bounded
---

## Unbounded Collection

**Impact: MEDIUM (memory growth without limit)**

Collections that grow without bounds (log buffers, event histories, undo stacks) eventually consume all available memory, causing OOM or degraded performance from allocation pressure.

**Detection:** `Vec`, `VecDeque`, or other collections with `push` but no eviction or capacity limit.

**Fix:** Use bounded buffers with a maximum capacity. Evict oldest entries when full (ring buffer pattern).
