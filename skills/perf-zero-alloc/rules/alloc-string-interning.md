---
title: String Interning
impact: CRITICAL
impactDescription: Repeated string allocations and comparisons in hot paths waste cycles
tags: string, intern, symbol, atom
---

## String Interning

**Impact: CRITICAL (repeated string allocations and comparisons in hot paths waste cycles)**

Intern strings during initialization. After interning, comparisons are integer comparisons and no further allocations occur.

```rust
use string_cache::DefaultAtom as Atom;

pub struct SymbolTable {
    symbols: DashMap<Atom, u32>,
    next_id: AtomicU32,
}

impl SymbolTable {
    pub fn get_or_intern(&self, symbol: &str) -> u32 {
        // Fast path: already interned
        if let Some(id) = self.symbols.get(&Atom::from(symbol)) {
            return *id;
        }
        // Slow path: intern new symbol (happens once per unique symbol)
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        self.symbols.insert(Atom::from(symbol), id);
        id
    }
}
```

**Benefits:**
- Symbol comparison becomes integer comparison
- No string allocations after initial interning
- Ideal for finite symbol sets (e.g., market tickers)
