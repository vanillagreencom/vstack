---
title: SAFETY Comment Standard
impact: CRITICAL
impactDescription: Unsafe blocks without justification are unsound by default
tags: unsafe, safety, comments, documentation
---

## SAFETY Comment Standard

**Impact: CRITICAL (unsafe blocks without justification are unsound by default)**

Every `unsafe` block requires a `// SAFETY:` comment explaining why it is sound. The comment must address all applicable invariants for the operations performed.

### Required Elements

Each SAFETY comment must cover every applicable item:

1. **Validity** — Why is the pointer/reference valid?
2. **Alignment** — How do we know alignment is correct?
3. **Aliasing** — Why are there no conflicting references?
4. **Initialization** — How do we know memory is initialized?
5. **Lifetime** — Why does the data outlive its use?

**Incorrect (missing or incomplete SAFETY comment):**

```rust
unsafe {
    let value = ptr::read(ptr);
}
```

**Correct (complete SAFETY comment with verifiable claims):**

```rust
unsafe {
    // SAFETY:
    // - ptr is valid: checked non-null on line 42
    // - ptr is aligned: guaranteed by allocator (8-byte aligned)
    // - No aliasing: unique ownership via Box::into_raw
    // - Memory initialized: written on line 44
    let value = ptr::read(ptr);
}
```
