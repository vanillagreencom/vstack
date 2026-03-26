---
title: Async State Machines
impact: CRITICAL
tags: async, state-machine, allocation, size, yield
---

## Async State Machines

**Impact: CRITICAL (unbounded stack/heap growth from oversized futures)**

Every `async fn` compiles to an anonymous enum state machine. Each `.await` is a yield point that creates a new variant. All variables captured across `.await` points are stored in the future struct.

- Large captured variables = large future. A future holding a `[u8; 65536]` across an `.await` is 64KB+ per instance.
- Use `std::mem::size_of_val(&future)` to profile future sizes.
- Box large futures at creation: `Box::pin(large_async_fn())`.
- Minimize variables held across `.await` — drop or move them before yielding.
- Deeply nested async call chains multiply future sizes (each level wraps the inner).

**Incorrect (large buffer captured across .await):**

```rust
async fn process() {
    let buffer = [0u8; 65536]; // 64KB lives in the future struct
    let result = network_call().await; // buffer held across .await
    use_buffer(&buffer, result);
}

// Spawning 1000 tasks = 64MB just for buffers
for _ in 0..1000 {
    tokio::spawn(process());
}
```

**Correct (scope buffer before .await or box the future):**

```rust
async fn process() {
    let result = network_call().await;
    let buffer = [0u8; 65536]; // Allocated AFTER .await, not captured across yield
    use_buffer(&buffer, result);
}

// Or box the future to move it to heap with a single pointer on the stack
async fn process_boxed() {
    let buffer = [0u8; 65536];
    let fut = Box::pin(async move {
        let result = network_call().await;
        use_buffer(&buffer, result);
    });
    fut.await;
}
```
