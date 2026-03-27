---
title: Signal Through Noise
impact: CRITICAL
impactDescription: Traders miss critical information when decorative elements compete with data
tags: philosophy, signal, noise, information, focus
---

## Signal Through Noise

**Impact: CRITICAL (traders miss critical information when decorative elements compete with data)**

In a professional trading interface, the signal is price action, position state, and order status. Everything else is noise. The entire visual system exists to maximize the signal-to-noise ratio.

### The Noise Budget

Every visual element draws from a finite attention budget. Borders, shadows, background variations, animations, icons, and color all cost attention. Spend this budget exclusively on information-carrying elements.

**High-value attention spend:**
- Directional color on P&L and price changes (instant understanding)
- Stale data indicators (prevents acting on old information)
- Error and disconnect states (prevents catastrophic trades)
- Direction icons alongside color (redundant encoding = faster processing)

**Low-value attention spend (minimize or eliminate):**
- Decorative borders between panels (a 1px line at low opacity is enough)
- Box shadows for "depth" (near-black elevation handles this)
- Animated transitions between states (instant state changes are faster to process)
- Color-coded categories that aren't directional (adds hues without adding meaning)
- Gradient text effects (decorative, adds no information, signals consumer product)
- Centered layouts (force the eye to find the start of each line; left-alignment is faster to scan)
- Rounded corners (visual softening that consumes sub-pixel space and signals approachability over precision)

### Earned Pixels

Every pixel of color, every point of padding, every border must answer: "What information does this convey?" If the answer is "it looks nice" or "it's conventional" — remove it. Professional traders don't want nice-looking software. They want software that makes them faster.

### State Communication

The interface must immediately communicate these states without the user searching:

| State | Must Be Obvious Because |
|-------|------------------------|
| Connected / disconnected | Trading on stale data causes losses |
| Position direction and P&L | Core awareness at all times |
| Order status | Knowing if orders are working, filled, or rejected |
| Data freshness | Stale prices look identical to live prices without explicit indication |
| Error conditions | Hidden errors lead to missed trades or worse |

### Animation Philosophy

Animation in trading UI is almost always noise. Use it only when it communicates information that would otherwise be missed:

- **Acceptable:** Brief flash on a price update to indicate the tick direction. Skeleton shimmer to indicate loading (vs. empty).
- **Not acceptable:** Panel slide-in transitions, fade-in effects for data, animated charts of non-real-time data, loading spinners when a skeleton would work.

Transitions should be instant or near-instant (under 100ms). If a user can perceive the animation as an animation rather than a state change, it's too slow.
