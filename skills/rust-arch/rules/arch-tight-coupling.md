---
title: Tight Coupling
impact: CRITICAL
impactDescription: can't swap implementations or test in isolation
tags: architecture, coupling, traits
---

## Tight Coupling

**Impact: CRITICAL (can't swap implementations or test in isolation)**

Using concrete types instead of trait interfaces between modules prevents dependency injection, makes unit testing require full dependency chains, and blocks implementation swaps.

**Incorrect (concrete dependency):**

```rust
struct OrderRouter {
    exchange: BinanceClient,  // Locked to one implementation
}
```

**Correct (trait-based dependency injection):**

```rust
struct OrderRouter<E: Exchange> {
    exchange: E,  // Any implementation works
}
```
