---
title: Async Traits
impact: HIGH
tags: async-trait, dyn, dispatch, boxing, generics
---

## Async Traits

**Impact: HIGH (unnecessary heap allocations in hot paths; compile errors from object safety)**

Since Rust 1.75, `async fn` works directly in traits for static dispatch. Dynamic dispatch (`dyn Trait`) still requires boxing the returned future.

**Guidelines:**
- Use native `async fn in trait` (Rust 1.75+) for static dispatch via generics. Zero overhead.
- For `dyn` dispatch: use the `async-trait` crate or manually return `Pin<Box<dyn Future>>`. Both heap-allocate.
- Object-safe async traits require `#[async_trait]` or manual boxing — the compiler cannot determine return future size for vtable dispatch.
- **Hot paths**: prefer generics (static dispatch) to avoid per-call `Box` allocation. Monomorphization eliminates indirection.
- **Plugin/extensibility boundaries**: `dyn` dispatch is acceptable — the allocation cost is negligible compared to the IO/network call.

**Incorrect (async-trait crate in hot path):**

```rust
#[async_trait::async_trait]
trait PriceSource {
    async fn get_price(&self, symbol: &str) -> f64;
}

// Every call to get_price allocates a Box<dyn Future> — in a hot loop this adds up
async fn poll_prices(source: &dyn PriceSource) {
    loop {
        let price = source.get_price("AAPL").await; // Box allocation per call
        process(price);
    }
}
```

**Correct (generic for static dispatch in hot path):**

```rust
trait PriceSource {
    async fn get_price(&self, symbol: &str) -> f64; // Native async fn (1.75+)
}

// Generic — monomorphized at compile time, zero allocation overhead
async fn poll_prices<S: PriceSource>(source: &S) {
    loop {
        let price = source.get_price("AAPL").await; // No boxing
        process(price);
    }
}

// Dyn dispatch OK at plugin boundaries:
#[async_trait::async_trait]
trait Plugin {
    async fn on_event(&self, event: &Event);
}
async fn notify_plugins(plugins: &[Box<dyn Plugin>], event: &Event) {
    for plugin in plugins {
        plugin.on_event(event).await; // Box overhead is fine here
    }
}
```
