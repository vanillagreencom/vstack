---
title: Pointer Aliasing Rules
impact: CRITICAL
impactDescription: Aliasing violations cause undefined behavior under Stacked Borrows
tags: pointer, aliasing, mutable, unsafe
---

## Pointer Aliasing Rules

**Impact: CRITICAL (aliasing violations cause undefined behavior under Stacked Borrows)**

Verify the single-writer-or-multiple-readers invariant: at any point, either exactly one `*mut T` is writing, OR one or more `*const T` are reading — never both simultaneously. Creating `&mut T` from a raw pointer invalidates all other pointers to the same memory under Stacked Borrows. Use `UnsafeCell` when interior mutability through shared references is required.
