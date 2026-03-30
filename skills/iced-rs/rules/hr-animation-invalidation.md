---
title: Animation Invalidation
impact: CRITICAL
impactDescription: Animated geometry changes render correctly but layout stays stale
tags: animation, shell, redraw, invalidate_layout, custom_widget, Animation
---

## Animation Invalidation

**Impact: CRITICAL (animated geometry changes render correctly but layout stays stale)**

When a custom widget animates, choose the right shell invalidation:

- **Paint-only animation** (color, opacity, stroke width, rotation with fixed bounds): `shell.request_redraw()` is sufficient.
- **Layout-affecting animation** (size, position, expand/collapse, clipping bounds, hit region): require **both** `shell.request_redraw()` **and** `shell.invalidate_layout()`.

Omitting `invalidate_layout()` when geometry changes causes the widget to paint at its new size while surrounding layout retains the old dimensions — producing overlap, clipping, or dead space.

**Diagnostic:** if a widget "only updates on the second click," suspect stale layout before suspecting message routing.

### Immediate Transition Start

When a user interaction triggers an animated state change, do not wait for incidental window events to advance the animation. Start the tween and immediately request both a redraw and a scheduled next-frame redraw:

```rust
shell.request_redraw();
shell.request_redraw_at(Instant::now() + FRAME_INTERVAL);
```

If geometry changes, also call `shell.invalidate_layout()`.

### Sustaining the Animation Loop

Handle `RedrawRequested` to keep the animation alive every frame until the tween completes:

```rust
// On state transition (e.g. toggle expand/collapse)
if state.pending_transition {
    shell.invalidate_layout();
    shell.request_redraw();
    shell.request_redraw_at(Instant::now() + FRAME_INTERVAL);
    state.pending_transition = false;
}

// On each frame while animating
if let Event::Window(window::Event::RedrawRequested(now)) = event {
    if state.animation.is_animating(*now) {
        shell.invalidate_layout();
        shell.request_redraw();
        shell.request_redraw_at(*now + FRAME_INTERVAL);
    }
}
```

### Custom Widget Lifecycle

For custom widgets that own an `iced::Animation<bool>`:

1. Store animation state in widget `Tree` state.
2. On state flip, call `animation.go_mut(new_state, Instant::now())`.
3. While animating: request redraws every frame; invalidate layout every frame if geometry changes.
4. In `layout()`, compute animated geometry from the current animation progress.
5. In `draw()`, clip to the animated bounds if only part of the child should be visible.
