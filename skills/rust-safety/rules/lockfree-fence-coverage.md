---
title: No Fence Without Loom Coverage
impact: HIGH
impactDescription: Fences without ordering verification may be insufficient or redundant
tags: lock-free, fence, loom, atomic
---

## No Fence Without Loom Coverage

**Impact: HIGH (fences without ordering verification may be insufficient or redundant)**

Never add an `atomic::fence` without corresponding loom test coverage proving it is both necessary and sufficient. Fences are easy to misplace and their effects are non-local — loom testing is the only reliable way to verify correctness.
