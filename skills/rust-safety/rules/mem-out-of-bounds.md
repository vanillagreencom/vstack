---
title: No Out-of-Bounds Access
impact: CRITICAL
impactDescription: Out-of-bounds access is undefined behavior
tags: memory, bounds, pointer, unsafe
---

## No Out-of-Bounds Access

**Impact: CRITICAL (out-of-bounds access is undefined behavior)**

Verify that all pointer arithmetic and slice indexing stays within allocated bounds. Check `offset()`, `add()`, `sub()` calls against the allocation size. For slices created from raw parts (`slice::from_raw_parts`), verify the length does not exceed the allocation.
