---
title: Display Formatting via Symbol Metadata
impact: HIGH
impactDescription: Hardcoded decimal places show wrong precision for different instruments
tags: display, formatting, decimals, symbol
---

## Display Formatting via Symbol Metadata

**Impact: HIGH (hardcoded decimal places show wrong precision for different instruments)**

Format prices using the symbol's `display_decimals`, never hardcoded decimal places. Different instruments have different display precision (e.g., EURUSD: 5, AAPL: 2, BTC: 8).

**Incorrect (hardcoded decimals):**

```rust
format!("{:.2}", price)
```

**Correct (symbol-driven precision):**

```rust
format!("{:.1$}", price, symbol.display_decimals as usize)
```
