---
title: Miri for Compile-Time UB Detection
impact: HIGH
impactDescription: Miri detects undefined behavior including dangling pointers, Stacked Borrows violations, and uninitialized reads
tags: sanitizer, miri, ub, testing, ci
---

## Miri for Compile-Time UB Detection

**Impact: HIGH (Miri detects undefined behavior including dangling pointers, Stacked Borrows violations, and uninitialized reads)**

Miri is an interpreter for Rust's MIR that detects undefined behavior at test time. It catches bugs that ASan and TSan miss, including Stacked Borrows violations and invalid enum discriminants.

**Setup:**

```bash
cargo +nightly miri test
```

**Miri detects:**
- Dangling pointer dereferences
- Invalid enum discriminants
- Uninitialized memory reads
- Stacked Borrows violations (aliasing rule breaches)
- Data races (in concurrent code)

**Key MIRIFLAGS:**

| Flag | Purpose |
|---|---|
| `-Zmiri-strict-provenance` | Enforce strict pointer provenance (no int-to-ptr casts) |
| `-Zmiri-symbolic-alignment-check` | Catch alignment issues that hardware would silently accept |
| `-Zmiri-tree-borrows` | Use Tree Borrows model instead of Stacked Borrows (experimental) |

Miri runs ~100x slower than native execution. Gate tests that do not exercise unsafe code with `#[cfg(not(miri))]` to keep CI time reasonable.

**Incorrect (no Miri gating — slow CI with no benefit):**

```rust
#[test]
fn test_pure_safe_logic() {
    // This test has no unsafe code — running under Miri wastes CI time
    assert_eq!(2 + 2, 4);
}
```

**Correct (gate non-unsafe tests, run Miri on unsafe tests):**

```rust
#[cfg(not(miri))]
#[test]
fn test_pure_safe_logic() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_unsafe_ptr_operations() {
    // This exercises unsafe code — Miri will catch UB here
    let mut val = 42u64;
    let ptr = &mut val as *mut u64;
    unsafe {
        ptr.write(100);
        assert_eq!(ptr.read(), 100);
    }
}
```

**GitHub Actions integration:**

```yaml
- name: Miri
  run: cargo +nightly miri test
  env:
    MIRIFLAGS: "-Zmiri-strict-provenance"
```
