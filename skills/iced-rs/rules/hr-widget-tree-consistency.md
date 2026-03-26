---
title: Widget Tree Consistency
impact: CRITICAL
impactDescription: Widgets silently stop responding to events
tags: widget, tree, conditional, mouse_area
---

## Widget Tree Consistency

**Impact: CRITICAL (widgets silently stop responding to events)**

Iced tracks widgets by tree position. Conditionally wrapping widgets based on interaction state changes the tree structure, breaking event tracking.

**Incorrect (conditional wrapping changes tree shape):**

```rust
if dragging {
    mouse_area(label).into()
} else {
    label.into()
}
```

**Correct (always wrap, conditionally enable):**

```rust
mouse_area(label)
    .on_press_maybe(if enable { Some(msg) } else { None })
```
