---
name: price-handling
description: f64 price patterns with epsilon comparison and tick-size rounding. Use when handling prices, comparisons, rounding, feed parsing, or display formatting.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Price Handling Patterns

f64 price handling rules for trading systems — epsilon comparison, tick-size rounding, feed normalization, and display formatting, prioritized by impact.

## When to Apply

Reference these guidelines when:
- Comparing f64 prices (equality, greater/less than)
- Rounding prices to tick size or validating tick alignment
- Formatting prices for display or broker APIs
- Parsing price data from market feeds
- Designing price-related types or symbol metadata structs
- Deciding whether f64 is the right numeric type for a use case

## Rule Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Core Rules | CRITICAL | `core-` |
| 2 | Boundaries | HIGH | `boundary-` |
| 3 | Type Design | MEDIUM | `type-` |
| 4 | Feed Ingestion | MEDIUM | `feed-` |

## Quick Reference

### 1. Core Rules (CRITICAL)

- `core-f64-for-prices` - IEEE 754 f64 for all prices; no fixed-point unless matching engine or regulatory
- `core-epsilon-comparison` - Never use == on f64 prices; use float_cmp with PRICE_EPSILON (1e-10)
- `core-tick-rounding` - round_to_tick and validate_tick_alignment functions for tick-size math

### 2. Boundaries (HIGH)

- `boundary-order-submission` - All rounding and validation converges at order submission boundary
- `boundary-no-round-market-data` - Never round ticks, bars, or quotes at ingestion; preserve feed precision
- `boundary-display-formatting` - Format prices using symbol display_decimals, never hardcoded decimal places

### 3. Type Design (MEDIUM)

- `type-symbol-metadata` - Tick size and display precision belong in SymbolSpec, not in the price value
- `type-price-newtype` - Optional zero-overhead newtype wrapper that omits PartialEq to prevent accidental ==

### 4. Feed Ingestion (MEDIUM)

- `feed-normalize-at-ingest` - Convert all feed formats (double, string, scaled integer) to f64 at ingest boundary

## How to Use

Read individual rule files for detailed explanations and code examples:

```
rules/core-epsilon-comparison.md
rules/boundary-order-submission.md
```

Each rule file contains:
- Brief explanation of why it matters
- Incorrect code example with explanation (where applicable)
- Correct code example with explanation

## Resources

### ctx7 CLI

| Library | ctx7 ID | Use For |
|---------|---------|---------|
| Rust std | `/websites/doc_rust-lang_stable_std` | f64 methods, parse, formatting |

## Full Compiled Document

For the complete guide with all rules expanded: `AGENTS.md`
