---
title: Leaky Abstraction
impact: CRITICAL
impactDescription: breaks encapsulation and couples consumers to internals
tags: architecture, abstraction, encapsulation
---

## Leaky Abstraction

**Impact: CRITICAL (breaks encapsulation and couples consumers to internals)**

When internal implementation details cross module boundaries, consumers become coupled to internals. Any refactoring of the implementation then breaks all consumers.

**Indicator:** Public APIs expose internal types, implementation-specific error variants, or require callers to understand internal data layout.

**Fix:** Add a facade or interface layer. Expose only the contract (traits, public types) — never the mechanism.
