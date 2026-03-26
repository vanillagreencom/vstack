---
title: View Is Pure
impact: CRITICAL
impactDescription: Hidden state corruption and non-deterministic rendering
tags: view, state, purity, side_effects
---

## View Is Pure

**Impact: CRITICAL (hidden state corruption and non-deterministic rendering)**

`view()` must be a pure function of `State`. No side effects, no memoization that depends on call frequency, no hidden state. All mutable state lives in `State` and changes only in `update()`.
