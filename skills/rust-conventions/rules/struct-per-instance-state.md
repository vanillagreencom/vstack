---
title: Per-Instance State
impact: HIGH
tags: state, global, instances
---

## Per-Instance State

**Impact: HIGH (global field silently returns wrong data)**

When different instances (panes, tabs, widgets) have different dimensions/state, store per-instance. A single global field silently returns wrong data for the non-last-updated instance.
