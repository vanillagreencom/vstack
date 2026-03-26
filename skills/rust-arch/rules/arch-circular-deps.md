---
title: Circular Dependencies
impact: CRITICAL
impactDescription: build issues and tangled logic
tags: architecture, dependencies, layering
---

## Circular Dependencies

**Impact: CRITICAL (build issues and tangled logic)**

Modules that depend on each other create cycles that prevent clean layering, cause build ordering issues, and make it impossible to reason about data flow in isolation.

**Indicator:** Module A imports from B, module B imports from A.

**Fix:** Enforce layered architecture — dependencies flow DOWN only. Extract shared types into a common lower-layer module that both depend on.
