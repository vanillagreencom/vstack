---
title: Component Design Approach
impact: MEDIUM
impactDescription: Inconsistent components across panels, or over-engineering where simplicity suffices
tags: component, widget, composition, architecture
---

## Component Design Approach

**Impact: MEDIUM (inconsistent components across panels, or over-engineering where simplicity suffices)**

Trading UI components fall into two categories. Choose the right approach for each.

### Two Approaches

**Composed components** — assembled from existing primitives (text, row, column, button, input). Use for most trading widgets.

- PriceDisplay, PositionBadge, PnlDisplay, AlertBanner, NumericStepper, SymbolSearch, StatusIndicator
- Faster to build, automatically inherit framework accessibility and interaction patterns
- Easier to maintain consistency because they use the same primitives as everything else

**Custom-rendered components** — drawn directly via canvas, WebGL, GPU primitives, or equivalent. Use only for performance-critical visualization.

- Charts (candlestick, depth, time & sales)
- DOM/order book with high-frequency updates
- Heatmaps, volume profiles
- Any component that needs to render 1000+ data points at 60fps

The threshold is simple: if a composed component can maintain 60fps with your data volume, use composition. If it can't, drop to custom rendering for that specific component.

### Component Density: The ShadCN Model, Compressed

ShadCN/ui demonstrates excellent component design principles: consistent tokens, composable primitives, clear visual hierarchy. For trading, apply the same principles but at higher density:

| ShadCN Pattern | Trading Adaptation |
|---------------|-------------------|
| Generous padding (px-4 py-2) | Minimal padding (px-2 py-1 or less) |
| Comfortable line-height | Tight line-height (1.2-1.3) |
| Standard 14-16px body text | 11-13px data text |
| Card-based layouts with gaps | Edge-to-edge panels with minimal gaps |
| Rounded corners (radius-md) | Minimal or no rounding (0-2px) |
| Prominent hover states | Subtle hover (opacity shift, not color change) |

The principle is the same (consistent, composable, token-driven) but the spatial budget is dramatically smaller.

### Trading Widget Patterns

Standard widgets every trading interface needs. These are patterns, not implementations — build them in whatever stack you're using:

| Widget | Purpose | Key Design Requirements |
|--------|---------|------------------------|
| **PriceDisplay** | Current price + change | Monospace, directional color + icon, absolute and percentage change |
| **PositionBadge** | Compact position indicator | Direction (long/short), quantity, P&L — all in one dense row |
| **PnlDisplay** | P&L with breakdown | Monospace, directional color + icon, realized/unrealized distinction |
| **NumericStepper** | Precise numeric input | Step size tied to instrument tick size, keyboard + scroll support, min/max bounds |
| **SymbolSearch** | Instrument lookup | Type-ahead, fuzzy match, recent history, compact results list |
| **StatusIndicator** | Connection/system health | Color-coded dot + label, semantic color from tokens |
| **AlertBanner** | Dismissible notification | Severity levels, inline or toast, manual or auto-dismiss per severity |
| **OrderTicket** | Order entry form | Side toggle, quantity stepper, price input, order type selector, submit with directional color |

### Component Checklist

Before any component is considered complete:

- [ ] All visual values from semantic tokens (no hardcoded colors, sizes, fonts)
- [ ] Numeric data in monospace with tabular figures
- [ ] Directional data has both color and icon/text indicator
- [ ] All five panel states handled (loading, empty, error, disconnected, data)
- [ ] Keyboard accessible with visible focus indicator
- [ ] Tooltips on all icon-only interactive elements
- [ ] Tested at target density (not just "looks good in isolation")
