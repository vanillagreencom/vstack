---
title: Trading Data Display Conventions
impact: HIGH
impactDescription: Misaligned prices, ambiguous direction, or missing context misleads traders
tags: data, price, position, order, pnl, display
---

## Trading Data Display Conventions

**Impact: HIGH (misaligned prices, ambiguous direction, or missing context misleads traders)**

Every data type in a trading application has specific display conventions. These are not suggestions — they are the industry-standard patterns professional traders expect.

### Price Display

- **Always monospace** — prices are the most frequently scanned numbers in the interface
- **Direction indicator: icon + color** — triangle up/down (or arrow) plus directional semantic color. Never color alone.
- **Show both absolute and percentage change** — "$4,512.50 +12.25 (+0.27%)" gives both magnitude and relative context
- **Current price is primary, change is secondary** — use size and opacity hierarchy to subordinate the change to the current price
- **Decimal alignment in columns** — all prices in a column must align on the decimal point
- **Consistent decimal places per instrument** — ES futures always show 2 decimals, BTC might show 1. Don't truncate or pad inconsistently.

### Position Display

- **Direction badge** — a compact indicator showing "Long" or "Short" in the appropriate directional color
- **Quantity in monospace** — right-aligned, tabular figures
- **Entry price in secondary text** — visible but subordinate to current P&L
- **P&L with directional color + icon** — the most important number in the position row. Show both unrealized and realized when applicable.
- **Row tint** — the entire position row gets a subtle directional background tint (5-10% opacity of the position's directional color). This makes scanning a list of mixed long/short positions instant.

### Order Display

- **Side indicator** — clearly shows Buy or Sell with appropriate directional color
- **All prices and quantities in monospace** — right-aligned, decimal-aligned
- **Status coloring** — pending (neutral/warning), filled (positive flash then neutral), rejected (negative), cancelled (dimmed)
- **Cancel action always visible** — on active (working) orders, the cancel button is always visible without hovering or expanding. Traders need to cancel orders instantly.
- **Time priority visible** — show order time in secondary text. Traders need to know how long an order has been working.

### P&L Display

- **Directional color + icon** — profit in positive color with up indicator, loss in negative color with down indicator
- **Monospace, right-aligned** — like all numeric data
- **Show currency symbol or unit** — "$+1,234.56" not just "+1,234.56"
- **Realized vs unrealized distinction** — when showing both, clearly label which is which. Don't rely on position alone.
- **Daily/total toggle** — traders often need to switch between session P&L and total P&L

### Alerts and Notifications

| Severity | Behavior | Dismissal | Position |
|----------|----------|-----------|----------|
| **Info/fills** | Transient, appears at screen edge | Auto-dismiss (3-5s) | Toast position, doesn't interrupt workflow |
| **Warning** | Visible but non-blocking | Timed (10s) or manual | Toast position, slightly more prominent |
| **Error** | Prominent, demands attention | Manual dismiss required | Inline in affected panel or prominent toast |
| **Persistent** | Inline with affected content | Until condition resolves | Inline banner in the relevant panel |

Errors must never auto-dismiss. A trader must acknowledge an error. Fills and info can auto-dismiss because they are confirmations, not problems.

### General Data Rules

- **Stale data** — gray out with "Last update: HH:MM:SS" timestamp. Never show stale data at full opacity.
- **Empty columns** — show a dash "—" not blank space. Blank cells are ambiguous (loading? empty? error?).
- **Loading** — skeleton shimmer for known layouts, spinner for unknown. Never show a blank panel while loading.
- **Watchlist rows** — compact: symbol (left-aligned), last price (right-aligned), change indicator (right-aligned). Row interactions: click (select), double-click (open/action), right-click (context menu), drag (reorder).
- **Confirmation dialogs for high-stakes actions** — required for orders above a configurable size threshold. Show full details: side, quantity, symbol, price, estimated cost. Primary action button uses directional color. Cancel is always available and clearly labeled.
