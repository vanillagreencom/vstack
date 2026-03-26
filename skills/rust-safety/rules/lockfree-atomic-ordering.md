---
title: Atomic Ordering Justification
impact: HIGH
impactDescription: Wrong ordering silently permits data races
tags: lock-free, atomic, ordering, memory-model
---

## Atomic Ordering Justification

**Impact: HIGH (wrong ordering silently permits data races)**

Every atomic operation must have its memory ordering documented and justified in a comment. State which happens-before relationships the ordering establishes and why weaker orderings are insufficient. Common mistake: using `Relaxed` where `Release`/`Acquire` pairs are needed to synchronize non-atomic data.
