---
title: Task Cancellation Lifecycle
impact: CRITICAL
tags: tokio, cancellation, abort, joinhandle, cancellation-token
---

## Task Cancellation Lifecycle

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
