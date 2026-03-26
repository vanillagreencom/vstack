---
title: Dynamic Dispatch in Hot Path
impact: CRITICAL
impactDescription: +5-20ns per call from vtable indirection
tags: performance, hot-path, dispatch, generics
---

## Dynamic Dispatch in Hot Path

**Impact: CRITICAL (+5-20ns per call from vtable indirection)**

`Box<dyn Trait>` and `&dyn Trait` in hot paths prevent inlining and add vtable lookup overhead on every call. In tight loops processing millions of events, this compounds.

**Detection:** `Box<dyn ...>` or `&dyn ...` in data processing paths.

**Fix:** Use generics with static dispatch. Monomorphization eliminates vtable overhead and enables inlining.
