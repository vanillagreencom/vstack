---
title: Semantic Token Architecture
impact: CRITICAL
impactDescription: Hardcoded values bypass theming, break consistency, and cause visual drift
tags: tokens, semantic, design-system, theming, abstraction
---

## Semantic Token Architecture

**Impact: CRITICAL (hardcoded values bypass theming, break consistency, and cause visual drift)**

No component should contain a raw color value, pixel measurement, or font specification. All visual properties reference semantic tokens defined in a central design system. This is a structural requirement, not a stylistic preference.

### Token Categories

Your design system must define tokens in these categories:

| Category | Examples | Why Tokens |
|----------|----------|-----------|
| **Directional colors** | positive, negative (bid/ask, buy/sell, profit/loss) | Directional meaning must be consistent everywhere |
| **Surface levels** | surface-base, surface-panel, surface-raised, surface-hover, surface-active | Elevation system is architectural |
| **Text hierarchy** | text-primary, text-secondary, text-tertiary, text-disabled | Information hierarchy must be consistent |
| **Borders** | border-default, border-subtle | Panel structure consistency |
| **Spacing** | space-xs, space-sm, space-md, space-lg, space-xl | Layout rhythm |
| **Typography** | font-ui, font-data, size-primary, size-standard, size-secondary | Font system enforcement |

### Why This Matters for Trading UI

Trading applications have unusually strict consistency requirements:

1. **Directional color must be identical everywhere** — if "green" means "profit" in the P&L panel but a slightly different green means "buy" in order entry, the user's subconscious pattern-matching breaks
2. **Density requires precision** — at 4px spacing units, a hardcoded "5px" gap is visibly wrong. Tokens enforce the grid.
3. **Multi-panel layouts amplify inconsistency** — traders see 6-12 panels simultaneously. A color mismatch between panels is immediately obvious.
4. **Theming is not optional** — different markets use different directional conventions (red/green in US, green/red in some Asian markets). Traders expect to customize their workspace with established palettes (Tokyo Night, Catppuccin, Dracula, Nord, etc.). Tokens make all of this a configuration change, not a code change.

### Naming Convention

Name tokens by their **semantic role**, not their visual appearance:

| Wrong | Right | Why |
|-------|-------|-----|
| `green-500` | `color-positive` | What if positive is red in another market? |
| `dark-bg` | `surface-base` | "Dark" describes appearance, not role |
| `small-text` | `text-secondary` | "Small" is relative and not semantic |
| `gray-border` | `border-default` | Gray is a color, not a meaning |

### Enforcement

In code review, any hardcoded color value, pixel measurement, or font name in component code is a defect — not a style issue. Treat it with the same severity as a logic bug. The only place raw values should exist is in the design system's token definition file.
