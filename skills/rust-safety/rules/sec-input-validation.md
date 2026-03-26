---
title: Input Validation Before Use
impact: HIGH
impactDescription: Unbounded input enables buffer overflows and memory corruption
tags: security, input, validation, bounds
---

## Input Validation Before Use

**Impact: HIGH (unbounded input enables buffer overflows and memory corruption)**

All inputs from external sources must be bounds-checked before use in unsafe code or allocation. Verify lengths, ranges, and formats at the boundary. Never pass unchecked external data directly to pointer arithmetic, slice construction, or allocation size calculations.
