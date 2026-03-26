---
title: No Manual Drop Mixed with Epoch
impact: HIGH
impactDescription: Manual drop bypasses deferred reclamation safety
tags: crossbeam, epoch, drop, manual
---

## No Manual Drop Mixed with Epoch

**Impact: HIGH (manual drop bypasses deferred reclamation safety)**

Never manually drop epoch-protected data. Manually calling `drop()` or using `Box::from_raw()` on epoch-protected pointers bypasses the deferred reclamation mechanism, potentially freeing memory while other threads still hold references through their pinned guards.
