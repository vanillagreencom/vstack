---
title: No Double-Free
impact: CRITICAL
impactDescription: Freeing memory twice is undefined behavior
tags: memory, double-free, drop, unsafe
---

## No Double-Free

**Impact: CRITICAL (freeing memory twice is undefined behavior)**

Verify that ownership is transferred exactly once. Common sources: calling `Box::from_raw` on the same pointer twice, or manually dropping a value that will also be dropped by its owner. Use `ManuallyDrop` or `mem::forget` when ownership must be surrendered without running the destructor.
