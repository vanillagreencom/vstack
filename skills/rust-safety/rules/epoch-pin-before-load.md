---
title: Pin Before Atomic Load
impact: HIGH
impactDescription: Loading without pin allows reclamation during read
tags: crossbeam, epoch, pin, atomic
---

## Pin Before Atomic Load

**Impact: HIGH (loading without pin allows reclamation during read)**

When using crossbeam epoch, `epoch::pin()` must be called before every atomic load that accesses shared data. The guard returned by `pin()` prevents the current epoch from advancing, ensuring referenced data is not reclaimed while being read.
