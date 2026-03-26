---
title: Feature Envy
impact: CRITICAL
impactDescription: wrong responsibility placement
tags: architecture, responsibility, data-ownership
---

## Feature Envy

**Impact: CRITICAL (wrong responsibility placement)**

When code in one module heavily accesses another module's data — calling multiple getters, destructuring its types, or computing derived values from its fields — the logic belongs in the data owner, not the consumer.

**Indicator:** A function that takes a struct from another module and accesses 3+ of its fields.

**Fix:** Move the computation to the module that owns the data. Expose a method instead of exposing fields.
