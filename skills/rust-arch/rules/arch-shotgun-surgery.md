---
title: Shotgun Surgery
impact: CRITICAL
impactDescription: high change cost across many files
tags: architecture, coupling, cohesion
---

## Shotgun Surgery

**Impact: CRITICAL (high change cost across many files)**

When a single logical change requires edits to many unrelated files, the related logic is scattered. This makes changes error-prone (easy to miss a file) and expensive to review.

**Indicator:** One feature change touches 5+ files across different modules.

**Fix:** Consolidate related logic. Group types and functions that change together into the same module.
