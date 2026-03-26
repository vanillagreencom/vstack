# Rust Async Patterns

**Version 1.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when writing
> async Rust code. Humans may also find it useful, but guidance here
> is optimized for automation and consistency by AI-assisted workflows.

---

## Abstract

Async runtime internals, concurrency composition, and task management patterns for Rust, prioritized by impact from critical (Future/Poll model, tokio runtime) to high (select/join composition, async patterns) to medium (debugging and common pitfalls). Covers the Future::poll contract, Pin/Unpin semantics, spawn_blocking, select!/join! composition, cancellation safety, structured concurrency, backpressure, async traits, stream processing, and tokio-console debugging.

---

## Table of Contents

1. [Future & Poll Model](#1-future--poll-model) — **CRITICAL**
   - 1.1 [Future::poll Contract](#11-futurepoll-contract)
   - 1.2 [Async State Machines](#12-async-state-machines)
2. [Tokio Runtime](#2-tokio-runtime) — **CRITICAL**
   - 2.1 [Task Cancellation Lifecycle](#21-task-cancellation-lifecycle)
3. [Select & Join](#3-select--join) — **HIGH**
   - 3.1 [select! Semantics](#31-select-semantics)
   - 3.2 [Cancellation Safety](#32-cancellation-safety)
4. [Async Patterns](#4-async-patterns) — **HIGH**
   - 4.1 [Structured Concurrency](#41-structured-concurrency)
   - 4.2 [Async Traits](#42-async-traits)
   - 4.3 [Stream Processing](#43-stream-processing)

---

## 1. Future & Poll Model

**Impact: CRITICAL**

Core Future::poll contract, Pin/Unpin semantics, and async state machine internals. Violations cause busy-loops, use-after-move, and unbounded memory growth.

### 1.1 Future::poll Contract

**Impact: CRITICAL (busy-loop or hung task if violated)**

The `Future::poll` contract has three invariants:

1. **Return `Pending` only after registering a waker.** If you return `Poll::Pending` without calling `cx.waker().wake_by_ref()` or storing the waker for later notification, the executor has no way to know when to re-poll. The future is never woken and appears hung.

2. **Never return `Pending` without waker registration.** This causes a busy-loop: the executor keeps polling, gets `Pending`, and has no wake signal, so it either spins or gives up.

3. **Never poll after `Ready`.** Once a future returns `Poll::Ready(value)`, polling again is a logic error. The future may panic or return garbage. Use `FusedFuture` or `.fuse()` if you need safe repeated polling.

**Incorrect (Pending without waker registration):**

```rust
impl Future for MyFuture {
    type Output = i32;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.ready {
            Poll::Ready(42)
        } else {
            // BUG: no waker registered — this future will never wake
            Poll::Pending
        }
    }
}
```

**Correct (waker stored for later notification):**

```rust
impl Future for MyFuture {
    type Output = i32;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.ready {
            Poll::Ready(42)
        } else {
            // Store waker so the background task can call wake() when ready
            self.get_mut().waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
```

### 1.2 Async State Machines

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

---

## 2. Tokio Runtime

**Impact: CRITICAL**

Tokio runtime configuration, spawn_blocking for blocking work, and task lifecycle management. Violations cause runtime stalls, deadlocks, and thread pool exhaustion.

### 2.1 Task Cancellation Lifecycle

**Impact: CRITICAL (resource leaks or orphaned tasks if cancellation is misunderstood)**

Dropping a `JoinHandle` **detaches** the task — it does NOT cancel it. The task continues running in the background, potentially leaking resources, holding connections, or writing to closed channels.

**Cancellation strategies:**
- `AbortHandle::abort()` — forcefully cancels the task at the next `.await` point. The `JoinHandle` returns `JoinError::is_cancelled`.
- `CancellationToken` — cooperative cancellation. The task checks `token.is_cancelled()` or `token.cancelled().await` at strategic points.
- Dropping a future inside `select!` — cancels by dropping at the `.await` yield point. Only safe if the future is cancellation-safe.

**Incorrect (assuming drop cancels):**

```rust
async fn leak_example() {
    let handle = tokio::spawn(async {
        // This task runs FOREVER even after handle is dropped
        loop {
            do_work().await;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
    drop(handle); // BUG: task is detached, not cancelled — runs until process exits
}
```

**Correct (cooperative cancellation with CancellationToken):**

```rust
use tokio_util::sync::CancellationToken;

async fn managed_example() {
    let token = CancellationToken::new();
    let task_token = token.clone();

    let handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = task_token.cancelled() => {
                    // Clean up resources
                    break;
                }
                _ = do_work() => {}
            }
        }
    });

    // Later: signal cancellation
    token.cancel();
    handle.await.unwrap(); // Task exits cleanly
}
```

---

## 3. Select & Join

**Impact: HIGH**

Composing futures with select!, join!, try_join!, and collection types. Violations cause lost data from cancelled branches, starvation, and subtle cancellation bugs.

### 3.1 select! Semantics

**Impact: HIGH (data loss from unexpected branch cancellation)**

`tokio::select!` polls multiple futures and completes when the FIRST one resolves. All other branches are **dropped immediately**, cancelling them mid-execution.

**Key rules:**
- Losing branches are dropped (cancelled) at their last `.await` point. Any partial progress is lost unless the future is cancellation-safe.
- Use `biased` for deterministic priority: without it, branches are polled in random order. With `biased`, the first matching branch always wins.
- Use `.fuse()` on completed futures to prevent re-polling a finished future (returns `Pending` forever after `Ready`).
- In loops, pin futures **outside** the loop. Otherwise they are recreated and restarted on every iteration.

**Incorrect (future recreated each loop iteration):**

```rust
loop {
    tokio::select! {
        // BUG: timeout is recreated every iteration — never fires
        _ = tokio::time::sleep(Duration::from_secs(30)) => {
            println!("timeout");
            break;
        }
        msg = rx.recv() => {
            process(msg);
        }
    }
}
```

**Correct (future pinned outside loop):**

```rust
let timeout = tokio::time::sleep(Duration::from_secs(30));
tokio::pin!(timeout);

loop {
    tokio::select! {
        _ = &mut timeout => {
            println!("timeout");
            break;
        }
        msg = rx.recv() => {
            process(msg);
        }
    }
}
```

### 3.2 Cancellation Safety

**Impact: HIGH (silent data loss from partially completed operations)**

A future is **cancellation-safe** if dropping it at any `.await` point does not lose data or leave state inconsistent. This matters because `select!` drops losing branches immediately.

**Cancellation-safe (safe in select!):**
- `tokio::sync::Mutex::lock` — retries acquisition from scratch on re-poll
- `tokio::sync::mpsc::Receiver::recv` — message stays in channel if dropped before completion
- `tokio::net::TcpListener::accept` — no partial state
- `tokio::time::sleep` — stateless timer

**NOT cancellation-safe (dangerous in select!):**
- `tokio::io::AsyncReadExt::read_exact` — partial read data is lost on drop
- `tokio::io::AsyncWriteExt::write_all` — partial write progress is lost
- `futures::StreamExt::next` on some streams — buffered data may be lost
- Custom futures with internal buffers that accumulate across polls

**Incorrect (unsafe future in select!):**

```rust
loop {
    tokio::select! {
        _ = cancel_token.cancelled() => break,
        // BUG: if cancelled mid-read, partial bytes are lost forever
        result = reader.read_exact(&mut buf) => {
            process(&buf);
        }
    }
}
```

**Correct (wrap non-cancellation-safe future in spawn):**

```rust
loop {
    tokio::select! {
        _ = cancel_token.cancelled() => break,
        // Spawned task runs to completion even if this branch loses
        result = tokio::spawn(async move {
            reader.read_exact(&mut buf).await
        }) => {
            process(&buf);
        }
    }
}

// Or use a cancellation-safe alternative:
loop {
    tokio::select! {
        _ = cancel_token.cancelled() => break,
        // read() is cancellation-safe (returns partial reads)
        n = reader.read(&mut buf) => {
            accumulated.extend_from_slice(&buf[..n?]);
        }
    }
}
```

---

## 4. Async Patterns

**Impact: HIGH**

Structured concurrency, backpressure, async traits, and stream processing. Violations cause resource leaks, OOM under load, and unnecessary allocations in hot paths.

### 4.1 Structured Concurrency

**Impact: HIGH (orphaned tasks, unobserved panics, resource leaks)**

Unstructured `tokio::spawn` scatters tasks with no parent tracking. Panics go unobserved, tasks outlive their logical scope, and cleanup becomes impossible.

**Guidelines:**
- Prefer `JoinSet` over unbounded `spawn`. `JoinSet` tracks all spawned tasks and can abort them on drop.
- Propagate panics: `JoinError::is_panic()` detects panicked tasks. Unwrap or propagate — never silently ignore.
- Use `task::spawn_local` for `!Send` futures (e.g., futures holding `Rc`, `RefCell`).
- **Nursery pattern**: a scope that outlives all child tasks. When the scope exits, all children are joined or cancelled.

**Incorrect (fire-and-forget spawn):**

```rust
async fn handle_connections(listener: TcpListener) {
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        // BUG: no tracking — panics are silent, no way to shut down gracefully
        tokio::spawn(handle_connection(stream));
    }
}
```

**Correct (structured with JoinSet):**

```rust
async fn handle_connections(listener: TcpListener, cancel: CancellationToken) {
    let mut tasks = tokio::task::JoinSet::new();

    loop {
        tokio::select! {
            _ = cancel.cancelled() => break,
            accepted = listener.accept() => {
                let (stream, _) = accepted.unwrap();
                tasks.spawn(handle_connection(stream));
            }
            // Reap completed tasks and propagate panics
            Some(result) = tasks.join_next() => {
                if let Err(e) = result {
                    if e.is_panic() {
                        tracing::error!("task panicked: {e}");
                    }
                }
            }
        }
    }

    // Graceful shutdown: wait for all in-flight tasks
    tasks.shutdown().await;
}
```

### 4.2 Async Traits

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

### 4.3 Stream Processing

**Impact: HIGH (stalled pipelines, lost data on shutdown, unbounded memory)**

Async streams (`Stream` trait) are the async equivalent of `Iterator`. They produce items asynchronously and are composed using `StreamExt` combinators.

**Key patterns:**
- `buffered(n)` — process up to `n` futures concurrently, yielding results **in order**. Use when order matters.
- `buffer_unordered(n)` — process up to `n` futures concurrently, yielding results **as they complete**. Higher throughput when order does not matter.
- `chunks(n)` — batch items into groups of `n` for bulk processing (DB inserts, API calls).
- `tokio::time::timeout` — wrap individual items to prevent one slow item from blocking the pipeline.
- **Graceful shutdown**: break on cancellation token, then drain remaining items from the stream.

**Incorrect (sequential processing of concurrent work):**

```rust
use futures::StreamExt;

let mut stream = futures::stream::iter(urls)
    .map(|url| fetch(url));

// BUG: processes one at a time — no concurrency
while let Some(result) = stream.next().await {
    let response = result.await;
    save(response).await;
}
```

**Correct (concurrent buffered processing with graceful shutdown):**

```rust
use futures::StreamExt;
use tokio_util::sync::CancellationToken;

let token = CancellationToken::new();

let mut stream = futures::stream::iter(urls)
    .map(|url| async move {
        tokio::time::timeout(Duration::from_secs(30), fetch(url)).await
    })
    .buffer_unordered(10)  // 10 concurrent fetches
    .chunks(50);           // Batch into groups of 50

loop {
    tokio::select! {
        _ = token.cancelled() => break,
        Some(batch) = stream.next() => {
            let successful: Vec<_> = batch
                .into_iter()
                .filter_map(|r| r.ok().and_then(|r| r.ok()))
                .collect();
            bulk_save(successful).await;
        }
        else => break, // Stream exhausted
    }
}
```

