---
title: Box::into_raw/from_raw for Ownership Transfer
impact: CRITICAL
tags: ownership, Box, memory, allocator
---

## Box::into_raw/from_raw for Ownership Transfer

**Impact: CRITICAL (double-free or use-after-free from mismatched allocators)**

Use `Box::into_raw` to give ownership to C (Rust stops managing memory). Use `Box::from_raw` to reclaim (C must not have freed it). Never free Rust memory with C's `free()` or vice versa — allocators are incompatible. Document ownership in API: "caller must call `X_free()` to release". Pattern: constructor returns `*mut T`, destructor takes `*mut T`.

**Incorrect (freeing Rust allocation with C free):**

```rust
#[no_mangle]
pub extern "C" fn create_config() -> *mut Config {
    Box::into_raw(Box::new(Config::default()))
}

// In C code:
// Config* cfg = create_config();
// free(cfg); // BUG: C's free() doesn't know about Rust's allocator
```

**Correct (paired constructor/destructor):**

```rust
/// Creates a new Config. Caller must call `config_free()` to release.
#[no_mangle]
pub extern "C" fn config_new() -> *mut Config {
    Box::into_raw(Box::new(Config::default()))
}

/// Frees a Config created by `config_new()`. Passing null is a no-op.
#[no_mangle]
pub extern "C" fn config_free(ptr: *mut Config) {
    if ptr.is_null() {
        return;
    }
    // SAFETY: ptr was created by Box::into_raw in config_new()
    // and has not been freed yet (caller's contract)
    unsafe {
        drop(Box::from_raw(ptr));
    }
}
```
