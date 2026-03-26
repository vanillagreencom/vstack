---
title: Required Panel States
impact: HIGH
impactDescription: Blank panels, missing error feedback, or invisible data staleness
tags: panel, state, loading, error, empty, disconnected
---

## Required Panel States

**Impact: HIGH (blank panels, missing error feedback, or invisible data staleness)**

Every panel must implement all five states. A panel is not complete until each state renders correctly and communicates the right information.

### The Five States

| State | Visual Pattern | Purpose |
|-------|---------------|---------|
| **Loading (known layout)** | Skeleton shimmer matching expected content shape | User sees that data is coming and knows roughly what to expect |
| **Loading (unknown)** | Centered spinner with context text ("Connecting to feed...") | User knows something is happening, even if layout can't be predicted |
| **Empty** | Centered icon + helpful text ("No positions. Place an order to get started.") | User knows the panel is working but has no data to show — and knows how to change that |
| **Error** | Inline banner with actionable message ("Feed disconnected. Reconnecting in 5s..." or "Failed to load orders. [Retry]") | User knows what went wrong and what they can do about it |
| **Disconnected** | Last data shown but grayed/dimmed with stale warning and timestamp. Order entry and modifications disabled. | User sees the last known state but is clearly warned it may be outdated. Prevents trading on stale data. |

### Why Every State Matters

| Missing State | Consequence |
|--------------|-------------|
| No loading state | Panel is blank; user doesn't know if it's broken or loading |
| No empty state | Panel is blank; user doesn't know if it's loading or has nothing to show |
| No error state | Panel is blank or shows stale data; user doesn't know something is wrong |
| No disconnected state | User sees stale prices at full opacity and may trade on outdated data |

In trading, each of these consequences can lead to financial loss. Blank panels with no explanation are never acceptable.

### Disconnected State: Special Attention

The disconnected state deserves particular focus because it's the most dangerous for the trader:

- All data remains visible but at **reduced opacity** — clearly dimmed compared to live data
- A **stale data warning** is visible without scrolling — timestamp of last update, reconnection status
- **Order entry is disabled** — the most important safety measure. Cannot submit orders on stale data.
- **Order modification/cancellation remains enabled** — cancelling existing orders on stale data is safer than leaving them active
- The transition from live to disconnected should be **instant and obvious** — no subtle fade, no delay

### Transition Between States

State transitions should be immediate. No fade animations between states. When data arrives, the loading state is instantly replaced by data. When connection drops, the display immediately enters the disconnected state. Speed of state communication is a safety feature.
