# Sections

This file defines all sections, their ordering, impact levels, and descriptions.
The section ID (in parentheses) is the filename prefix used to group rules.

---

## 1. Future & Poll Model (future)

**Impact:** CRITICAL
**Description:** Core Future::poll contract, Pin/Unpin semantics, and async state machine internals. Violations cause busy-loops, use-after-move, and unbounded memory growth.

## 2. Tokio Runtime (tokio)

**Impact:** CRITICAL
**Description:** Tokio runtime configuration, spawn_blocking for blocking work, and task lifecycle management. Violations cause runtime stalls, deadlocks, and thread pool exhaustion.

## 3. Select & Join (compose)

**Impact:** HIGH
**Description:** Composing futures with select!, join!, try_join!, and collection types. Violations cause lost data from cancelled branches, starvation, and subtle cancellation bugs.

## 4. Async Patterns (pat)

**Impact:** HIGH
**Description:** Structured concurrency, backpressure, async traits, and stream processing. Violations cause resource leaks, OOM under load, and unnecessary allocations in hot paths.
