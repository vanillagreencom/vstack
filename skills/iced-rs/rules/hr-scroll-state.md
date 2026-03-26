---
title: Scroll State Initialization
impact: CRITICAL
impactDescription: Initial dimensions never captured
tags: scrollable, sensor, on_scroll, on_show
---

## Scroll State Initialization

**Impact: CRITICAL (initial dimensions never captured)**

`scrollable.on_scroll` fires only after scrolling, not during initial layout. Capture initial dimensions with an explicit measurement step such as `sensor.on_resize`, then combine that with `on_scroll` if you also need ongoing scroll updates.
