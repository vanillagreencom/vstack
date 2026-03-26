---
title: No Use-After-Free
impact: CRITICAL
impactDescription: Reading freed memory is undefined behavior
tags: memory, use-after-free, lifetime, unsafe
---

## No Use-After-Free

**Impact: CRITICAL (reading freed memory is undefined behavior)**

Verify that every pointer or reference derived from allocated memory is not used after the allocation is freed. Common sources: `Box::into_raw` followed by `Box::from_raw` (consuming the allocation) while a raw pointer copy still exists, or references into a `Vec` that is subsequently reallocated.
