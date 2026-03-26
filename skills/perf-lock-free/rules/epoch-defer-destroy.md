---
title: Use defer_destroy for Epoch Deallocation
impact: HIGH
impactDescription: Use-after-free or double-free from manual drop mixed with epoch reclamation
tags: crossbeam, epoch, defer_destroy, reclamation
---

## Use defer_destroy for Epoch Deallocation

**Impact: HIGH (use-after-free or double-free from manual drop mixed with epoch reclamation)**

When removing nodes from an epoch-protected data structure, use `defer_destroy()` to schedule safe deallocation. Never mix manual `drop` with epoch reclamation — other threads may still hold references through pinned guards.

**Checklist for epoch-based structures:**

- Every atomic load is preceded by `epoch::pin()`
- Shared references don't escape guard lifetime
- `defer_destroy()` used for safe deallocation
- No mixing of manual drop with epoch reclamation
