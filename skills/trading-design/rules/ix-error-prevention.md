---
title: Error Prevention and Confirmation
impact: HIGH
impactDescription: Accidental orders or unintended trades cause financial loss
tags: interaction, error, confirmation, safety, orders
---

## Error Prevention and Confirmation

**Impact: HIGH (accidental orders or unintended trades cause financial loss)**

Trading UI handles real money. Error prevention is not a nice-to-have — it's a core design requirement. The interface should make it difficult to do the wrong thing accidentally and easy to recover when mistakes happen.

### Confirmation Flow Requirements

High-stakes actions require confirmation. At minimum:

| Action | Confirmation Required | Details in Confirmation |
|--------|----------------------|------------------------|
| Order placement (above threshold) | Yes | Side, quantity, symbol, price, order type, estimated cost |
| Position close/flatten | Yes | Symbol, current P&L, quantity being closed |
| Cancel all orders | Yes | Count of orders being cancelled, symbols affected |
| Modify working order | Context-dependent | Original vs new values highlighted |

### Confirmation Dialog Design

- **Show full details** — the trader must see exactly what will happen. "Are you sure?" with no context is useless.
- **Primary action button matches direction** — a buy confirmation's primary button uses the positive directional color. A sell confirmation uses the negative directional color. This provides one more visual check.
- **Cancel is always available** — prominent, keyboard-accessible, and never hidden
- **No nested confirmations** — one confirmation per action. "Are you really sure?" after "Are you sure?" is hostile UX
- **Configurable thresholds** — what constitutes a "large" order varies by trader and instrument. The size threshold that triggers confirmation should be configurable.

### Prevention Over Confirmation

Better than confirming a mistake is preventing it:

- **Quantity validation** — reject obviously wrong quantities (10x the typical size, negative numbers, zero)
- **Price validation** — warn when limit price is far from market (potential fat-finger)
- **Symbol verification** — highlight when the order symbol doesn't match the currently viewed chart
- **Side verification** — visually emphasize buy vs sell throughout the order entry process so the trader always knows which direction they're trading

### Recovery

When mistakes happen:
- **Cancel is always one action away** — on every working order row, cancel is visible and clickable without expanding or hovering
- **Undo where possible** — if an order hasn't been sent to the exchange yet, allow undo
- **Clear error messages** — when an order is rejected, show why in plain language with the rejected order details
