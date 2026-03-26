---
title: Two-Hue Directional Color
impact: CRITICAL
impactDescription: Visual noise and ambiguous directional cues when multiple hues compete
tags: color, palette, hue, direction, buy, sell
---

## Two-Hue Directional Color

**Impact: CRITICAL (visual noise and ambiguous directional cues when multiple hues compete)**

The base palette uses exactly two chromatic hues: one for positive direction (buy/bid/profit/long) and one for negative direction (sell/ask/loss/short). Conventionally green and red, but the principle is the constraint — not the specific hues.

All other visual variation comes from a single neutral (typically white) at graduated opacities. No blue, orange, yellow, purple, or other chromatic colors in the base palette.

### Why Two Hues

Trading interfaces have one overriding semantic axis: **direction**. Up/down, buy/sell, profit/loss, bid/ask. Two hues map perfectly to this axis. Every additional hue dilutes the directional signal:

- **Three hues** — the third hue has no clear directional meaning; users must learn an arbitrary mapping
- **Four+ hues** — the interface becomes a dashboard, not a trading tool; color loses its instant-read quality
- **One hue** — direction becomes invisible; the most important information channel is eliminated

Two hues mean a trader can glance at any part of the screen and instantly know direction from color alone (reinforced by icons/text per accessibility requirements).

### Neutral Variation

Everything that isn't directional uses the neutral at different opacities:

| Opacity Level | Role |
|--------------|------|
| 100% | Primary text, most important non-directional data |
| 70-80% | Secondary text, labels, headers |
| 40-50% | Tertiary text, timestamps, metadata |
| 20-30% | Disabled text, subtle indicators |
| 8-15% | Borders, dividers, row hover tints |
| 3-6% | Subtle background differentiation |

This creates a rich visual hierarchy from a single neutral — no need for gray, silver, slate, or other named neutrals. One color, many opacities.

### When You Need a Third Color

Occasionally a non-directional semantic is unavoidable (warnings, informational states, brand accent). Handle this by:

1. **Prefer neutral treatment first** — can you communicate this with opacity, icons, or position instead of a new hue?
2. **If a hue is truly needed** — use it extremely sparingly, at low saturation, and never in contexts where it could be confused with directional color
3. **Never more than one additional hue** — the accent/warning budget is one hue total, not one per semantic
