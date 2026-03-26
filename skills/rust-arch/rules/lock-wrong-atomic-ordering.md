---
title: Wrong Atomic Ordering
impact: HIGH
impactDescription: data races that only manifest under load
tags: concurrency, atomics, ordering, lock-free
---

## Wrong Atomic Ordering

**Impact: HIGH (data races that only manifest under load)**

Using `Relaxed` ordering everywhere is a common shortcut that causes data races on non-x86 architectures and can produce stale reads even on x86 under contention.

**Detection:** All atomics using `Ordering::Relaxed` without analysis of happens-before requirements.

**Fix:** Use proper Acquire/Release pairs. Producer stores with `Release`, consumer loads with `Acquire`. Use `SeqCst` only when total ordering across multiple atomics is required.
