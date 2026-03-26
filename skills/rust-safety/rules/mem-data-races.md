---
title: No Data Races
impact: CRITICAL
impactDescription: Concurrent unsynchronized access is undefined behavior
tags: memory, data-race, concurrency, atomics, unsafe
---

## No Data Races

**Impact: CRITICAL (concurrent unsynchronized access is undefined behavior)**

Verify that shared mutable state is protected by proper synchronization: mutex, atomic operations with correct ordering, or verified lock-free algorithms (loom-tested). Two threads accessing the same memory where at least one is writing without synchronization is always undefined behavior, even if it "works" in practice.
