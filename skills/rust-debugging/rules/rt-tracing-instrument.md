---
title: Tracing #[instrument] Spans
impact: HIGH
tags: tracing, instrument, spans, async, observability
---

## Tracing #[instrument] Spans

**Impact: HIGH (invisible async execution flow without structured spans)**

`#[instrument]` creates a span automatically on function entry, recording arguments as fields. Skip large or sensitive arguments, set appropriate levels, and attach spans to futures for async visibility.

**Incorrect (manual span creation with missing context):**

```rust
async fn process_order(order: Order, db: &Database) {
    // Manual span — verbose, easy to forget fields
    let span = tracing::info_span!("process_order");
    let _guard = span.enter();
    // Also wrong: .enter() doesn't work correctly in async —
    // the guard is held across await points
    let result = db.save(&order).await;
}
```

**Correct (using #[instrument] with appropriate options):**

```rust
// Auto-creates span with function name and args as fields
#[tracing::instrument(
    level = "info",
    skip(self, db),                        // Skip large/sensitive args
    fields(order_id = %order.id)           // Add custom fields
)]
async fn process_order(&self, order: Order, db: &Database) -> Result<()> {
    // Span automatically tracks the entire async execution
    let result = db.save(&order).await?;
    tracing::debug!("order saved successfully");
    Ok(result)
}

// For futures not tied to a function:
use tracing::Instrument;
let future = async_operation()
    .instrument(tracing::info_span!("background_task", task_id = %id));
tokio::spawn(future);
```
