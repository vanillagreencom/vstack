---
title: Cancellation Safety
impact: HIGH
tags: cancellation, select, safety, drop, partial-progress
---

## Cancellation Safety

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
