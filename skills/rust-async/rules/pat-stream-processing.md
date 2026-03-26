---
title: Stream Processing
impact: HIGH
tags: stream, buffered, chunks, timeout, shutdown, drain
---

## Stream Processing

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
