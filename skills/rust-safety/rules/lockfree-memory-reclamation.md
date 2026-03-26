---
title: Safe Memory Reclamation
impact: HIGH
impactDescription: Incorrect reclamation causes use-after-free in concurrent code
tags: lock-free, memory, reclamation, epoch, hazard-pointer
---

## Safe Memory Reclamation

**Impact: HIGH (incorrect reclamation causes use-after-free in concurrent code)**

Lock-free structures that remove nodes must use a safe reclamation scheme: epoch-based (crossbeam), hazard pointers, or exclusive ownership transfer. Verify that no reader can hold a reference to memory being reclaimed. Document the chosen scheme in the SAFETY comment.
