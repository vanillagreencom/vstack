---
title: Custom Panic Hooks
impact: HIGH
tags: panic, hooks, tracing, backtrace, production
---

## Custom Panic Hooks

**Impact: HIGH (panics in production vanish without structured logging)**

Use `std::panic::set_hook` for structured panic handling. Log thread name, backtrace, and panic location to tracing or error reporting before aborting. In production, always abort after logging to prevent undefined state.

**Incorrect (default panic handler with no structured logging):**

```rust
fn main() {
    // Default handler prints to stderr — lost in container logs,
    // no structured fields, no integration with error tracking
    run_server();
}
```

**Correct (custom panic hook with full context):**

```rust
fn main() {
    std::panic::set_hook(Box::new(|info| {
        let thread = std::thread::current();
        let thread_name = thread.name().unwrap_or("<unnamed>");
        let backtrace = std::backtrace::Backtrace::force_capture();

        eprintln!("PANIC in thread '{thread_name}': {info}");
        eprintln!("{backtrace}");

        // In production: log to tracing/sentry before abort
        // tracing::error!(%thread_name, %info, %backtrace, "panic");
    }));

    // Always abort in production — unwinding after panic leaves
    // undefined state
    // std::process::abort();

    run_server();
}
```
