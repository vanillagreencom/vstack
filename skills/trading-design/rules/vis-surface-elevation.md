---
title: Surface Elevation Model
impact: CRITICAL
impactDescription: Flat UI with no visual hierarchy between layers
tags: surface, elevation, depth, layering, background
---

## Surface Elevation Model

**Impact: CRITICAL (flat UI with no visual hierarchy between layers)**

Depth in a professional trading UI comes from a structured elevation system — not shadows, not gradients, not distinct background colors. Each elevation level is a controlled brightness increment from the near-black foundation.

### The Elevation Ladder

Define 5 elevation levels. Each level has one job:

| Level | Name | Purpose |
|-------|------|---------|
| 0 | Base | App background — the deepest layer, visible between panels |
| 1 | Panel | Primary content containers — where data lives |
| 2 | Raised | Cards, menus, dropdowns, popovers — elements above panels |
| 3 | Hover | Interactive feedback — hover state on any surface |
| 4 | Active | Selected/pressed state — highest emphasis background |

### Design Principles

- **No custom backgrounds** — every background color in the application should map to one of these five levels. If you're defining a background that doesn't fit, the elevation system needs extension, not circumvention.
- **No shadows** — in a near-black system, drop shadows are invisible or require unrealistic spread/opacity. Elevation through brightness is cleaner and more performant.
- **No gradients on surfaces** — gradients introduce visual noise. Flat surfaces at consistent elevation levels are easier to scan.
- **Borders do the rest** — where brightness difference alone isn't enough to separate adjacent same-level elements, use a low-opacity neutral border (8-15%). This is cheaper than adding elevation levels.

### Elevation in Context

| UI Element | Elevation Level |
|-----------|----------------|
| App background (gaps between panels) | 0 (Base) |
| Panel content area | 1 (Panel) |
| Header bar, status bar | 1 (Panel) |
| Dropdown menus, context menus | 2 (Raised) |
| Tooltips, popovers | 2 (Raised) |
| Dialog/modal backdrop | Overlay (semi-transparent black over everything) |
| Dialog/modal content | 2 (Raised) |
| Row on hover | 3 (Hover) |
| Selected row, active tab | 4 (Active) |
| Pressed button | 4 (Active) |

### Implementation Note

Your design system should define these as semantic surface tokens, not raw color values. Components reference "surface-panel" or "surface-hover," not a specific brightness value. This allows the entire elevation system to be tuned in one place and supports theme adaptation (dark, light, or community palettes like Catppuccin or Tokyo Night) without touching component code.

The elevation ladder is theme-independent. In dark themes, elevation increases brightness. In light themes, elevation may decrease brightness or add subtle shadows. The semantic names and the 5-level structure remain constant — only the values change per theme.
