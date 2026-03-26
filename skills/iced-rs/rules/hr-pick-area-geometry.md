---
title: Pick Area Geometry
impact: CRITICAL
impactDescription: Pane drag completely disabled
tags: pane_grid, titlebar, pick_area, layout
---

## Pick Area Geometry

**Impact: CRITICAL (pane drag completely disabled)**

TitleBar content must use `Shrink` width so empty space remains for the pick area. `Fill` width on tab row content eliminates the pick area, disabling pane drag entirely.

**Incorrect (Fill width consumes pick area):**

```rust
pane_grid::TitleBar::new(
    row![tabs].width(Length::Fill)  // No pick area left
)
```

**Correct (Shrink width preserves pick area):**

```rust
pane_grid::TitleBar::new(
    row![tabs].width(Length::Shrink)  // Pick area in remaining space
)
```
