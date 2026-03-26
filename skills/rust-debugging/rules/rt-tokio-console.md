---
title: tokio-console Async Debugging
impact: HIGH
tags: tokio, console, async, runtime, task-starvation
---

## tokio-console Async Debugging

**Impact: HIGH (async bugs invisible without runtime introspection)**

tokio-console provides a top-like view of async tasks — poll durations, waker counts, and resource contention. Essential for diagnosing slow polls, task starvation, and waker storms that are invisible to traditional debuggers.

**Incorrect (diagnosing async issues with println):**

```rust
// Scattering println! to find slow tasks — no aggregate view,
// floods logs, can't see waker/poll relationships
async fn handle_request(req: Request) -> Response {
    println!("start handling request");  // Which task? When?
    let data = fetch_data().await;
    println!("data fetched");            // How long was the poll?
    process(data)
}
```

**Correct (using tokio-console for runtime visibility):**

```toml
# Cargo.toml
[dependencies]
console-subscriber = "0.4"
tokio = { version = "1", features = ["full", "tracing"] }
```

```rust
// main.rs — initialize console subscriber
fn main() {
    console_subscriber::init();  // Replaces default tracing subscriber

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async { run_server().await });
}

// Build with tokio_unstable cfg flag:
// RUSTFLAGS="--cfg tokio_unstable" cargo build

// Run tokio-console in another terminal:
// $ tokio-console

// Diagnose:
// - Slow polls (>1ms) — blocking work on async thread
// - Task starvation — tasks waiting too long for poll
// - Waker storms — excessive wake notifications
// - Resource contention — mutex/semaphore bottlenecks
```
