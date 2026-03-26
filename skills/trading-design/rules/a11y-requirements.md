---
title: Accessibility and Cross-Platform
impact: MEDIUM
impactDescription: Unusable interface for visually impaired users or on specific platforms
tags: accessibility, contrast, focus, color, cross-platform, dpi
---

## Accessibility and Cross-Platform

**Impact: MEDIUM (unusable interface for visually impaired users or on specific platforms)**

### Contrast Requirements

Dark interfaces must meet contrast ratios carefully — it's easy to fail on a near-black background:

| Element | Minimum Ratio | Standard |
|---------|--------------|----------|
| Body text (< 18px) | 4.5:1 | WCAG AA |
| Large text (>= 18px) | 3:1 | WCAG AA |
| Interactive element boundaries | 3:1 | WCAG 2.1 |
| Focus indicators | 3:1 | WCAG 2.1 |

Test contrast for every text opacity level in your hierarchy. The tertiary/disabled text levels are the most likely to fail. If your "disabled" text is invisible on near-black, it fails accessibility even though "disabled things are supposed to look faded."

### Never Color Alone

This is the most important accessibility rule for trading UI. Directional color (positive/negative) must always be reinforced with a second indicator:

| Color Indicator | Required Reinforcement |
|----------------|----------------------|
| Green price change | Up arrow/triangle icon |
| Red P&L | Down arrow/triangle icon |
| Buy/sell button colors | "Buy"/"Sell" text label |
| Position direction color | "Long"/"Short" text badge |
| Status indicator color | Status text label |

~8% of men have color vision deficiency. A trading interface that relies on color alone for direction makes one in twelve male traders unable to distinguish buy from sell. This is not an edge case — it's a structural failure.

### Focus Indicators

- Every interactive element must have a visible focus indicator
- The focus ring must be visible against both the dark background and any surface level
- Use a high-contrast color for focus (the accent/brand color works well here since it's not directional)
- Focus rings should be 2px minimum width
- Never remove focus indicators for aesthetic reasons — if the default focus ring looks wrong, restyle it, don't hide it

### Cross-Platform Rendering

If your application targets multiple platforms:

- **Design at 1x (96 DPI)** — this is your baseline. All measurements and visual rules apply at 1x.
- **Test at 100%, 125%, 150%, 200% scaling** — the most common DPI settings. Ensure text remains readable, borders remain visible (a 1px border at 150% can become blurry), and spacing scales proportionally.
- **Use vector assets** — raster icons and images will blur at non-integer scale factors. SVG, font icons, or programmatically drawn graphics scale cleanly.
- **Font rendering varies** — the same font at the same size will look different across FreeType (Linux), DirectWrite (Windows), and Core Text (macOS). Test on all target platforms, especially at your small data text sizes (11-13px) where rendering differences are most visible.
- **Custom window chrome** — if using custom title bars, test them on all platforms. Native window management behavior (snap, resize, minimize) must work correctly.
- **Never assume pixel-perfect cross-platform rendering** — design with enough margin that minor rendering differences don't break the layout.
