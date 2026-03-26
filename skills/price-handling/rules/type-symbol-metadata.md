---
title: Symbol Metadata Owns Precision
impact: MEDIUM
impactDescription: Embedding precision in price type couples display to data
tags: symbol, metadata, tick-size, precision
---

## Symbol Metadata Owns Precision

**Impact: MEDIUM (embedding precision in price type couples display to data)**

Tick size and display precision belong in a per-symbol metadata struct, not in the price value itself.

**Incorrect (precision embedded in price):**

```rust
struct Price { value: f64, decimals: u8 }
```

**Correct (precision in symbol spec):**

```rust
#[derive(Clone, Copy)]
pub struct SymbolSpec {
    pub symbol_id: u32,
    pub tick_size: f64,         // Minimum price increment
    pub display_decimals: u8,   // Decimal places for UI
    pub lot_size: f64,          // Minimum quantity
}

impl SymbolSpec {
    pub fn round_price(&self, price: f64) -> f64 {
        round_to_tick(price, self.tick_size)
    }

    pub fn format_price(&self, price: f64) -> String {
        format!("{:.1$}", price, self.display_decimals as usize)
    }
}
```

Symbol table characteristics:
- Loaded at subscription setup (cold path)
- Keyed by symbol ID for O(1) lookup
- Re-synced on reconnect or symbol list change
