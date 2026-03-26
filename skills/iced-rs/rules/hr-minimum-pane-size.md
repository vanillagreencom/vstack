---
title: Minimum Pane Size
impact: CRITICAL
impactDescription: Panes collapse or ignore per-pane minimums
tags: pane_grid, min_size, resize, container
---

## Minimum Pane Size

**Impact: CRITICAL (panes collapse or ignore per-pane minimums)**

`PaneGrid::min_size` sets a uniform minimum for ALL panes. For per-pane minimums, wrap panel content in `container` with `min_width`/`min_height` and clamp resize ratios (0.15-0.85).
