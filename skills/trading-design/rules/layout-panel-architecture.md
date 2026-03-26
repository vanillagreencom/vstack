---
title: Panel Architecture and Docking
impact: HIGH
impactDescription: Bad layouts waste screen density or hide critical information
tags: layout, panels, docking, tiling, priority, modular
---

## Panel Architecture and Docking

**Impact: HIGH (bad layouts waste screen density or hide critical information)**

Professional trading interfaces are modular panel systems, not page layouts. Every piece of functionality lives in a discrete, dockable panel that can be arranged, resized, and collapsed by the trader.

### Core Layout Principles

- **Tiling, not floating** — panels tile to fill available space with no gaps. Floating windows waste the background underneath. The layout should behave like a tiling window manager: every pixel is owned by a panel.
- **Priority-based space allocation** — the chart (or primary data visualization) gets remaining space after all other panels claim their minimums. This ensures the highest-value content always has the largest area.
- **Defined minimums** — every panel has a minimum useful size. Below that size, the panel collapses rather than rendering unusably. Minimums are part of the panel's specification, not an afterthought.
- **User-controlled layout** — traders arrange their workspace once and expect it to persist. Layout state (which panels are visible, their positions, sizes) is saved and restored reliably.

### Panel Priority Ordering

Panels have a collapse priority. When viewport space shrinks, the lowest-priority panel collapses first:

| Priority | Panels | Collapse Behavior |
|----------|--------|-------------------|
| Never collapse | Chart, order entry | These are the reason the application exists |
| Last to collapse | Positions, active orders | Critical active-state awareness |
| Early collapse | Watchlist, account info, alerts | Important but not moment-to-moment critical |
| First to collapse | Settings, logs, analytics | Reference panels the trader checks periodically |

When a panel collapses, it should show a compact indicator with key counts (e.g., "Orders (3)", "Positions (2)") so the trader knows there's active data even when the panel isn't visible.

### Shell Structure

The application shell consists of:

1. **Header bar** (fixed) — symbol, account selector, connection status, global controls
2. **Content area** (flexible) — the dockable panel grid, resizable and rearrangeable
3. **Status bar** (fixed) — system status, connection latency, clock, global state indicators

The header and status bar are thin (24-32px each) and fixed. They never compete with the content area for space. The content area handles all the docking, splitting, and resizing.

### Panel Composition

Each panel follows a consistent internal structure:

1. **Panel header** (thin, 20-28px) — panel title, key action buttons, collapse/close controls
2. **Panel content** — the panel's primary function
3. **Panel footer** (optional, only if needed) — summary data, status

Keep panel chrome (header + footer) as thin as possible. The content area should dominate. A panel that's 50% chrome and 50% content has failed the density test.
