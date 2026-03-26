---
title: Pointer Validity Before Dereference
impact: CRITICAL
impactDescription: Dereferencing an invalid pointer is undefined behavior
tags: pointer, validity, null, dereference, unsafe
---

## Pointer Validity Before Dereference

**Impact: CRITICAL (dereferencing an invalid pointer is undefined behavior)**

Every raw pointer must be validated before dereference. Check: non-null, within allocated object, not dangling (allocation still live). For pointers received from external code (FFI, callbacks), validate at the boundary before any use.
