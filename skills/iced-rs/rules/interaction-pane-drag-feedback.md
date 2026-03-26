---
title: Keep PaneGrid Drag Feedback Internal
impact: MEDIUM
impactDescription: Native Dropped events never arrive
tags: pane_grid, drag, overlay, opaque
---

## Keep PaneGrid Drag Feedback Internal

**Impact: MEDIUM (native Dropped events never arrive)**

If pane dragging uses `pane_grid.on_drag(...)`, keep feedback inside the picked pane subtree or `pane_grid::Style`. `mouse_area`/`opaque` pane-drag overlays are rebuild-sensitive and can prevent native `Dropped` events from arriving.
