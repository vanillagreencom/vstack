---
title: Scope MIRI to Unsafe Code Paths
impact: MEDIUM
impactDescription: Wasted CI time running MIRI on safe code that cannot have UB
tags: miri, unsafe, testing, ci
---

## Scope MIRI to Unsafe Code Paths

**Impact: MEDIUM (wasted CI time running MIRI on safe code that cannot have UB)**

MIRI is only valuable on code paths exercising `unsafe` blocks. Safe Rust is compiler-guaranteed UB-free; MIRI adds nothing there. Running MIRI is ~1000x slower than native execution, so blanket `--lib` is prohibitive.

Scope MIRI runs to changed modules using `git diff` filters. Gate tests that don't reach unsafe code:

```rust
#[cfg(all(test, not(miri)))]
mod safe_only_tests {
    // These tests don't exercise unsafe code — skip under MIRI
}
```

**MIRI catches:** uninitialized memory, out-of-bounds access, use-after-free, aliasing violations (Stacked/Tree Borrows).

**MIRI cannot test:** syscalls, foreign code (no shims), multithreaded code (use loom instead).

```bash
# Scoped to changed modules (recommended)
MIRIFLAGS="-Zmiri-disable-isolation" cargo +nightly miri test --lib <module_filter>

# Tree Borrows (experimental, stricter aliasing model)
MIRIFLAGS="-Zmiri-disable-isolation -Zmiri-tree-borrows" cargo +nightly miri test --lib
```
