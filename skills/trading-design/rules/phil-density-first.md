---
title: Density First
impact: CRITICAL
impactDescription: Wasted screen real estate in a data-intensive application
tags: philosophy, density, spacing, layout, information
---

## Density First

**Impact: CRITICAL (wasted screen real estate in a data-intensive application)**

Density is the primary design constraint. Trading demands maximum data per pixel. Every element must earn its screen space.

### Principles

- **Default compact, scale up only for readability** — start at the smallest comfortable size for each element. If a label, gap, or padding can be smaller without harming legibility, make it smaller.
- **Pixel accountability** — decorative elements (gradients, shadows, rounded corners, excessive padding) are costs that cannot be justified. Zero radius on all elements. Zero shadows. Zero gradients on surfaces. These are not stylistic preferences — they are density requirements. Every rounded corner steals sub-pixel space from data. Every shadow adds visual noise that competes with content.
- **Simultaneous visibility** — traders configure their workspace to see everything at once. A panel that requires scrolling to show its core content has failed the density test.
- **Compact does not mean cramped** — density requires deliberate spacing at small scales. A 4px base unit with consistent multiples creates rhythm even at tight spacing. The goal is scannable density, not a wall of text.

### Density Benchmarks

| Element | Target | Rationale |
|---------|--------|-----------|
| Table row height | 20-28px | Enough for single-line data with small padding |
| Panel padding | 4-8px | Minimal chrome between content and border |
| Inter-element gap | 2-4px | Tight but distinguishable |
| Font size (data) | 11-13px | Readable monospace at typical viewing distance |
| Font size (labels) | 10-12px | Secondary to data, clearly subordinate |
| Icon size | 12-16px | Inline with text, not dominant |

These are guidelines, not absolutes. The right density depends on the data and the trader's viewing distance, but they establish the baseline expectation: this is not a consumer app.

### Information Hierarchy Through Density

Not all data is equal. Use density itself as a hierarchy tool:

- **Primary data** (price, P&L) — slightly larger, full opacity, prominent position
- **Secondary data** (labels, quantities, timestamps) — standard density, reduced opacity
- **Tertiary data** (metadata, IDs) — smallest size, lowest opacity, available but not competing

The hierarchy should be readable from arm's length: the most important numbers jump out even when everything is dense.
