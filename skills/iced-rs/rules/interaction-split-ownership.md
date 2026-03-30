---
title: Split Interaction Ownership
impact: MEDIUM
impactDescription: Semantic and visual interaction layers diverge, causing ghost clicks or missed events
tags: mouse_area, button, interaction, custom_widget
---

## Split Interaction Ownership

**Impact: MEDIUM (semantic and visual interaction layers diverge, causing ghost clicks or missed events)**

When `mouse_area` handles semantic interaction (press, hover, drag) while a `button` provides visual feedback (styling, states), interaction ownership is split across two widgets. This creates risks:

- Hit areas may differ — `button` respects its own bounds while `mouse_area` covers a potentially different region.
- Event ordering is fragile — both widgets consume the same pointer events, and which wins depends on tree position.
- State can desync — `button` visual state (pressed, hovered) may not reflect `mouse_area` semantic state.

When this pattern appears in code, flag it explicitly. Prefer consolidating interaction into one widget: either a styled `mouse_area` or a `button` with `on_press`. If both are truly needed, document which widget owns which concern and verify hit areas match.
