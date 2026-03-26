---
title: Overlay Starvation
impact: MEDIUM
impactDescription: Drag targets silently stop receiving events
tags: mouse_area, overlay, interaction, drag
---

## Overlay Starvation

**Impact: MEDIUM (drag targets silently stop receiving events)**

Stacked `mouse_area(...).interaction(...)` layers can stop underlying hover/move handlers from receiving events, even without `opaque(...)`. Prefer setting `Interaction::Grabbing` on the real drag target widgets instead of adding a global cursor layer. Use `opaque(...)` only for true capture zones (app-edge drop zones).
