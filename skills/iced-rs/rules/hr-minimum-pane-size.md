---
title: Minimum Pane Size
impact: CRITICAL
impactDescription: Panes collapse or ignore per-pane minimums
tags: pane_grid, min_size, resize, container
---

## Minimum Pane Size

**Impact: CRITICAL (panes collapse or ignore per-pane minimums)**

`PaneGrid::min_size` sets a uniform minimum for all panes. If different panes need different minimums, enforce them in the pane content or in your split/resize state instead of assuming `PaneGrid` tracks them per pane.
