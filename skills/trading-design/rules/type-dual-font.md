---
title: Dual-Font System
impact: CRITICAL
impactDescription: Misaligned numeric columns and inconsistent data presentation
tags: typography, font, monospace, sans-serif, data
---

## Dual-Font System

**Impact: CRITICAL (misaligned numeric columns and inconsistent data presentation)**

Two font categories. No exceptions. No third font. Monospace is the dominant voice — the product's identity. Sans-serif is reserved for extended prose.

### The Two Categories

| Category | Font Type | Used For |
|----------|-----------|----------|
| **Structural / Identity** | Monospace | Headings, section labels, navigation, button text, status indicators, form labels, panel titles, badges, all numeric data (prices, quantities, P&L, timestamps, order IDs, percentages) |
| **Prose / Description** | Sans-serif | Body paragraphs, descriptive copy, tooltips, help text, long-form explanations — anything that reads as a sentence |

### Why Monospace Dominates

This is infrastructure software, not a consumer app. Monospace carries authority. It signals precision, systems thinking, and technical seriousness. When a user sees monospace headings and labels, they subconsciously read "this was built by engineers for engineers." Sans-serif headlines read as marketing.

Monospace also guarantees that:
- **Columns align** — 1,234.56 and 9,876.54 occupy identical widths, enabling instant visual scanning
- **Prices are scannable** — decimal points stack vertically, letting traders compare values by position rather than reading each number
- **Changes are detectable** — when a price updates, only the changed digits shift appearance, not the entire number's width
- **Identity is consistent** — the same typeface in headings, labels, and data creates a unified visual language that reads as one coherent system

Sans-serif is used where readability at length matters:
- **Body copy flows better** — proportional width is easier to read in paragraphs
- **Descriptions stay subordinate** — the visual difference between mono headings and sans body creates automatic hierarchy without needing size jumps

### Critical: Tabular/Lining Figures

Not all monospace fonts are equal for trading. The monospace font must support:

- **Tabular figures** — all digits the same width (not proportional oldstyle)
- **Lining figures** — digits sit on the baseline and reach cap height (not descending oldstyle figures)
- **Clear zero distinction** — 0 must be visually distinct from O at 11px

Fonts known to work well for dense numeric data: JetBrains Mono, IBM Plex Mono, Iosevka, Berkeley Mono, Cascadia Code. Test at your target size (11-13px) before committing.

### The Line: Where Mono Meets Sans

The boundary is prose. **If it reads as a sentence or paragraph, it's sans. Everything else is mono.** Examples:

| Element | Font | Reasoning |
|---------|------|-----------|
| "Positions" panel title | Mono | Structural label, part of the interface chrome |
| "LIVE MARKET DATA" header | Mono | Status/identity label |
| "Cancel" button | Mono | Action label, part of the interface |
| "Create Account" button | Mono | Action label |
| "Buy 5 ES @ 4,512.25" button | Mono | Action with numeric content |
| Section label "PLATFORM CAPABILITIES" | Mono | Structural label, uppercase tracked |
| Order quantity "100" | Mono | Numeric data |
| Timestamp "14:32:05" | Mono | Numeric sequence |
| "Connect your existing brokerage accounts..." | Sans | Descriptive paragraph, reads as prose |
| Tooltip explanation text | Sans | Help prose |
| Error message body | Sans | Explanatory text |

### Monospace Typography Conventions

Monospace is not one-size-fits-all. Use weight, size, tracking, and case to create hierarchy within the mono system:

| Role | Treatment | Example |
|------|-----------|---------|
| Page/section headings | Mono, bold, tight tracking (tracking-tighter) | `font-mono font-bold tracking-tighter` |
| Structural labels | Mono, uppercase, wide tracking, small size | `font-mono text-[11px] tracking-widest uppercase` |
| Button text | Mono, medium weight, slightly tracked | `font-mono font-medium tracking-wide` |
| Data values | Mono, regular or bold for emphasis, tabular figures | `font-mono` at base size |
| Form labels | Mono, uppercase, small, tracked | `font-mono text-xs tracking-wide uppercase` |
