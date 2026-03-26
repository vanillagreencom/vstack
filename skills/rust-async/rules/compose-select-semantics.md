---
title: "select! Semantics"
impact: HIGH
tags: select, cancellation, biased, fuse, drop
---

## select! Semantics

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
