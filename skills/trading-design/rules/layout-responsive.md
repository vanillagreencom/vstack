---
title: Responsive Collapse Strategy
impact: HIGH
impactDescription: Panels overlap or become unusable at constrained viewport sizes
tags: responsive, collapse, breakpoint, layout, priority
---

## Responsive Collapse Strategy

**Impact: HIGH (panels overlap or become unusable at constrained viewport sizes)**

Trading applications typically run on large monitors (often multiple), but the layout must degrade gracefully when viewport space is constrained.

### Collapse Sequence

When the viewport shrinks below the combined minimum sizes of all visible panels:

1. **Collapse lowest-priority panels first** — following the panel priority ordering, collapse the least critical panels into compact indicators
2. **Stack remaining panels vertically** — when horizontal space can no longer accommodate side-by-side panels, stack them
3. **Switch to tabbed view** — at the smallest viable size, show one panel at a time with tab navigation between them

### Rules

- **Chart and order entry never collapse** — regardless of viewport size, these panels are always visible. They are the core function.
- **Breakpoints are defined in the design system** — not hardcoded in components. A single source of truth for layout breakpoints ensures consistent behavior.
- **Collapsed panels show data counts** — "Orders (3)" or "Positions (2 active)" so the trader maintains awareness even when the panel is collapsed.
- **Transitions are instant** — no slide animations when panels collapse or expand. The layout change should be immediate.
- **User can override** — if a trader explicitly forces a panel to stay visible, respect that even if it means other panels collapse earlier.

### Minimum Panel Sizes

Define minimum sizes as part of each panel's specification:

| Panel Type | Minimum Width | Minimum Height | Rationale |
|-----------|--------------|----------------|-----------|
| Chart | 400px | 300px | Usable price action visibility |
| Order entry | 250px | 200px | All fields visible without scrolling |
| Positions | 300px | 100px | At least one row plus header visible |
| Order book | 200px | 200px | Enough depth levels to be useful |
| Watchlist | 200px | 100px | At least 3-4 rows visible |

These are reference values. Actual minimums depend on your specific panel content, but the principle is: define them explicitly, don't let panels render at sizes where they're unusable.
