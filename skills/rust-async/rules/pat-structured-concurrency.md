---
title: Structured Concurrency
impact: HIGH
tags: joinset, spawn, panic, structured, nursery
---

## Structured Concurrency

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
