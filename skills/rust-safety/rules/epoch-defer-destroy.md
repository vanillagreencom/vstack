---
title: Deferred Destruction for Cleanup
impact: HIGH
impactDescription: Immediate drop of shared data causes use-after-free
tags: crossbeam, epoch, defer, destroy
---

## Deferred Destruction for Cleanup

**Impact: HIGH (immediate drop of shared data causes use-after-free)**

Use `defer_destroy()` (or equivalent deferred cleanup) for epoch-protected data that is being removed. Do not mix manual `drop` with epoch reclamation — deferred destruction ensures all current readers have exited their critical sections before memory is freed.
