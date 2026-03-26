---
title: Reactive Discipline
impact: HIGH
impactDescription: Unnecessary redraws, wasted GPU cycles, janky frame rates
tags: view, redraw, cache, batching
---

## Reactive Discipline

**Impact: HIGH (unnecessary redraws, wasted GPU cycles, janky frame rates)**

Never trigger redraws from `view()`. Invalidate caches explicitly in `update()`. Batch high-frequency data updates into ~16ms windows so `update()` sees bounded work and idle windows cause no redraw.
