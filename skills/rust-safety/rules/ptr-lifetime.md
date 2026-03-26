---
title: Pointer Lifetime Guarantee
impact: CRITICAL
impactDescription: Dangling pointer dereference is undefined behavior
tags: pointer, lifetime, dangling, unsafe
---

## Pointer Lifetime Guarantee

**Impact: CRITICAL (dangling pointer dereference is undefined behavior)**

Verify that the pointed-to data outlives every use of the pointer. Common violations: returning a pointer to a local variable, storing a pointer into a collection that outlives the source allocation, or holding a raw pointer across a `Vec` reallocation that invalidates it.
