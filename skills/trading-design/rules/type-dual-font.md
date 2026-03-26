---
title: Dual-Font System
impact: CRITICAL
impactDescription: Misaligned numeric columns and inconsistent data presentation
tags: typography, font, monospace, sans-serif, data
---

## Dual-Font System

**Impact: CRITICAL (misaligned numeric columns and inconsistent data presentation)**

Two font categories. No exceptions. No third font.

### The Two Categories

| Category | Font Type | Used For |
|----------|-----------|----------|
| **UI chrome** | Sans-serif | Labels, headers, navigation, panel titles, button text (non-numeric), tooltips |
| **Data** | Monospace | Prices, quantities, P&L, timestamps, order IDs, account numbers, percentages, numeric button content |

### Why This Matters

Monospace fonts guarantee that:
- **Columns align** — 1,234.56 and 9,876.54 occupy identical widths, enabling instant visual scanning
- **Prices are scannable** — decimal points stack vertically, letting traders compare values by position rather than reading each number
- **Changes are detectable** — when a price updates, only the changed digits shift appearance, not the entire number's width

Sans-serif fonts guarantee that:
- **Labels are compact** — proportional width means shorter strings for the same text, saving horizontal space
- **Chrome is subordinate** — the visual difference between sans and mono creates an automatic hierarchy: monospace data feels "heavier" and more important

### Critical: Tabular/Lining Figures

Not all monospace fonts are equal for trading. The monospace font must support:

- **Tabular figures** — all digits the same width (not proportional oldstyle)
- **Lining figures** — digits sit on the baseline and reach cap height (not descending oldstyle figures)
- **Clear zero distinction** — 0 must be visually distinct from O at 11px

Fonts known to work well for dense numeric data: JetBrains Mono, IBM Plex Mono, Iosevka, Berkeley Mono, Cascadia Code. Test at your target size (11-13px) before committing.

### The Line: Where Sans Meets Mono

Some elements exist at the boundary. The rule: **if it contains a number the user needs to compare or scan, it's monospace.** Examples:

| Element | Font | Reasoning |
|---------|------|-----------|
| "Positions" panel title | Sans | UI label, no numeric content |
| "Orders (3)" badge count | The "3" in mono, rest in sans — or all mono if simpler | Numeric content users scan |
| Order quantity "100" | Mono | Numeric, users compare quantities |
| "Cancel" button | Sans | Text action, no numeric content |
| "Buy 5 ES @ 4,512.25" button | Mono | Contains numbers users must verify before clicking |
| Timestamp "14:32:05" | Mono | Numeric sequence users scan |
