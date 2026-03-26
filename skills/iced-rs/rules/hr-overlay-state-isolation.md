---
title: Overlay State Isolation
impact: CRITICAL
impactDescription: Base layer widgets break when overlays change
tags: stack, overlay, widget_tree
---

## Overlay State Isolation

**Impact: CRITICAL (base layer widgets break when overlays change)**

Overlay layers (stack children beyond the base) must not affect the base layer's widget structure. Add/remove overlay layers freely, but never change how base-layer widgets are constructed based on overlay presence.
