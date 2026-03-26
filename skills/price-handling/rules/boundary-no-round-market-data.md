---
title: Never Round Market Data
impact: HIGH
impactDescription: Rounding at ingest destroys feed precision needed for analytics
tags: market-data, rounding, precision, feed
---

## Never Round Market Data

**Impact: HIGH (rounding at ingest destroys feed precision needed for analytics)**

Market data prices must preserve full feed precision. Do not round ticks, bars, or quotes at ingestion. Rounding belongs at the order submission boundary and display formatting only.

**Incorrect (rounding at ingest):**

```rust
tick.price = round_to_tick(raw, tick_size);
```

**Correct (preserve raw precision):**

```rust
tick.price = raw;  // Preserve feed precision
```

**Where to round:**
- Order submission (OrderRequest to broker API)
- Display formatting

**Where NOT to round:**
- Market data ingestion (preserve feed precision)
- P&L calculations (use raw values)
