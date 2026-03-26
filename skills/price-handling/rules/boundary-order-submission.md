---
title: Round at Order Submission
impact: HIGH
impactDescription: Rounding elsewhere silently destroys feed precision or submits misaligned prices
tags: order, submission, rounding, validation
---

## Round at Order Submission

**Impact: HIGH (rounding elsewhere silently destroys feed precision or submits misaligned prices)**

All price rounding and validation converges at the order submission boundary (OrderRequest to broker API).

```rust
pub fn prepare_order(
    request: &OrderRequest,
    symbol: &SymbolSpec,
) -> Result<ValidatedOrder, OrderError> {
    // 1. Round price to tick
    let price = symbol.round_price(request.limit_price);

    // 2. Validate alignment (should be no-op after rounding)
    if !validate_tick_alignment(price, symbol.tick_size) {
        return Err(OrderError::InvalidTickSize);
    }

    // 3. Format for broker API (if string required)
    let price_str = symbol.format_price(price);

    Ok(ValidatedOrder { price, price_str, /* ... */ })
}
```
