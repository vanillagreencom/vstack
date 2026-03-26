---
name: rust-async
description: Rust async internals, runtime patterns, and concurrency composition. Use when writing async Rust code — covers Future/Poll model, Pin/Unpin, spawn_blocking, select!/join!, cancellation safety, tokio task patterns, backpressure, and debugging.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Rust Async Patterns

Async runtime internals, concurrency composition, and task management patterns for Rust, prioritized by impact.

## When to Apply

Reference these guidelines when:
- Implementing or reviewing async Rust functions
- Working with tokio runtime, tasks, or spawn_blocking
- Composing futures with select!, join!, or streams
- Debugging deadlocks, starvation, or cancellation bugs
- Designing backpressure or rate-limiting in async pipelines
- Using async traits or dynamic dispatch with futures

## Rule Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Future & Poll Model | CRITICAL | `future-` |
| 2 | Tokio Runtime | CRITICAL | `tokio-` |
| 3 | Select & Join | HIGH | `compose-` |
| 4 | Async Patterns | HIGH | `pat-` |

## Quick Reference

### 1. Future & Poll Model (CRITICAL)

- `future-poll-contract` - Return Pending only after registering waker; never poll after Ready
- `future-state-machines` - async fn compiles to enum state machine; profile and box large futures

### 2. Tokio Runtime (CRITICAL)

- `tokio-task-cancellation` - Dropping JoinHandle does NOT cancel task; use AbortHandle or CancellationToken

### 3. Select & Join (HIGH)

- `compose-select-semantics` - select! drops losing branches; use biased for priority; pin futures outside loops
- `compose-cancellation-safety` - Only use cancellation-safe futures in select!; wrap unsafe ones in spawn

### 4. Async Patterns (HIGH)

- `pat-structured-concurrency` - Prefer JoinSet over unbounded spawn; propagate panics; nursery pattern
- `pat-async-traits` - async fn in traits (1.75+); Box for dyn dispatch; generics for hot paths
- `pat-stream-processing` - buffered(n) for concurrency; chunks for batching; graceful shutdown on cancel

## How to Use

Read individual rule files for detailed explanations and code examples:

```
rules/future-poll-contract.md
rules/tokio-task-cancellation.md
```

Each rule file contains:
- Brief explanation of why it matters
- Incorrect code example with explanation
- Correct code example with explanation

## Resources

### ctx7 CLI

| Library | ctx7 ID | Use For |
|---------|---------|---------|
| tokio | `/websites/rs_tokio` | Async runtime, tasks, channels |
| futures | `/websites/rs_futures` | Future combinators, streams |
| tokio-util | `/websites/rs_tokio-util` | Codec, framing, compat layers |
| pin-project | `/taiki-e/pin-project` | Safe pin projections |

## Full Compiled Document

For the complete guide with all rules expanded: `AGENTS.md`
