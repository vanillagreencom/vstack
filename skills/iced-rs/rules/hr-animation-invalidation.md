---
title: Animation Invalidation
impact: CRITICAL
impactDescription: Animated geometry changes render correctly but layout stays stale
tags: animation, shell, redraw, invalidate_layout, custom_widget
---

## Animation Invalidation

**Impact: CRITICAL (animated geometry changes render correctly but layout stays stale)**

When a custom widget animates, choose the right shell invalidation:

- **Paint-only animation** (color, opacity, stroke width): `shell.request_redraw()` is sufficient.
- **Layout-affecting animation** (size, position, expand/collapse): require **both** `shell.request_redraw()` **and** `shell.invalidate_layout()`.

Omitting `invalidate_layout()` when geometry changes causes the widget to paint at its new size while surrounding layout retains the old dimensions — producing overlap, clipping, or dead space.

**Incorrect (geometry animation with redraw only):**

```rust
fn on_event(&mut self, ..., shell: &mut Shell<'_, Message>, ...) {
    // Height is animating but only redraw is requested
    self.animated_height = lerp(self.start, self.end, t);
    shell.request_redraw();
}
```

**Correct (geometry animation with both):**

```rust
fn on_event(&mut self, ..., shell: &mut Shell<'_, Message>, ...) {
    self.animated_height = lerp(self.start, self.end, t);
    shell.request_redraw();
    shell.invalidate_layout();
}
```
