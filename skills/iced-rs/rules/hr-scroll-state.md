---
title: Scroll State Initialization
impact: CRITICAL
impactDescription: Initial dimensions never captured
tags: scrollable, sensor, on_scroll, on_show
---

## Scroll State Initialization

**Impact: CRITICAL (initial dimensions never captured)**

`scrollable.on_scroll` fires only on user-initiated scroll events, never on initial layout. Use `sensor.on_show` to capture initial dimensions, then combine with `on_scroll` for ongoing tracking.
