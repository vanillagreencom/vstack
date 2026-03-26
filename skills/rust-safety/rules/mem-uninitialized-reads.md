---
title: No Uninitialized Reads
impact: CRITICAL
impactDescription: Reading uninitialized memory is undefined behavior
tags: memory, uninitialized, MaybeUninit, unsafe
---

## No Uninitialized Reads

**Impact: CRITICAL (reading uninitialized memory is undefined behavior)**

Verify that all memory is fully initialized before being read. Use `MaybeUninit` for deferred initialization and call `assume_init()` only after every byte has been written. Never use `mem::uninitialized()` (deprecated and always UB for inhabited types).
