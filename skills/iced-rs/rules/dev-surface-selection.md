---
title: Surface Selection
impact: HIGH
impactDescription: Over-engineered widgets, unnecessary framework coupling, and brittle implementations
tags: widgets, canvas, shader, advanced, architecture
---

## Surface Selection

**Impact: HIGH (over-engineered widgets, unnecessary framework coupling, and brittle implementations)**

Choose the simplest Iced surface that matches the problem:
- Standard UI composition uses normal widgets.
- Custom visuals and dense rendering use `Canvas` or `Shader` first.
- Reserve `iced::advanced` for new control behavior or engine-level hooks the public APIs cannot express cleanly: custom widget semantics, hit-testing, focus, layout/event/runtime plumbing, renderer hooks, or custom subscriptions.

**Incorrect (using `iced::advanced` for a purely visual surface):**

```rust
// Purely visual depth heat strip implemented as a custom advanced widget.
struct DepthHeatStrip;

impl<Message> advanced::Widget<Message, Theme, Renderer> for DepthHeatStrip {
    // custom layout/event/runtime plumbing only to draw colored depth levels
}
```

**Correct (use the public surface that matches the need):**

```rust
// Standard UI composition stays with normal widgets.
row![watchlist, order_entry];

// Dense visual surface uses Canvas/Shader first.
Canvas::new(DepthHeatStripProgram { /* ... */ })

// Reach for iced::advanced only if the surface needs custom hit-testing,
// focus, layout/runtime behavior, or another engine-level hook.
```
