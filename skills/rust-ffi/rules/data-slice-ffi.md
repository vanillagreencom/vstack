---
title: Pass Slices as Pointer + Length
impact: CRITICAL
tags: slices, pointer, length, from_raw_parts
---

## Pass Slices as Pointer + Length

**Impact: CRITICAL (wrong length or null pointer causes buffer overflow or UB)**

Slices are `(ptr, len)` — C has no slice type. Pass as separate pointer + length parameters. Validate: non-null, length within allocation, alignment correct. Use `slice::from_raw_parts` with a SAFETY comment documenting all preconditions. For mutable: `slice::from_raw_parts_mut`, ensure exclusive access.

**Incorrect (no validation, missing SAFETY comment):**

```rust
#[no_mangle]
pub extern "C" fn sum_array(data: *const f64, len: usize) -> f64 {
    unsafe {
        // No null check, no SAFETY comment, no length validation
        let slice = std::slice::from_raw_parts(data, len);
        slice.iter().sum()
    }
}
```

**Correct (validated with SAFETY comment):**

```rust
#[no_mangle]
pub extern "C" fn sum_array(data: *const f64, len: usize) -> f64 {
    if data.is_null() || len == 0 {
        return 0.0;
    }

    // SAFETY:
    // - Caller guarantees `data` points to `len` contiguous f64 values
    // - Caller guarantees the memory is valid for the duration of this call
    // - data is non-null (checked above)
    // - f64 has no alignment issues on any supported platform
    let slice = unsafe { std::slice::from_raw_parts(data, len) };
    slice.iter().sum()
}

// Mutable version — caller must guarantee exclusive access:
#[no_mangle]
pub extern "C" fn zero_array(data: *mut f64, len: usize) {
    if data.is_null() || len == 0 {
        return;
    }

    // SAFETY:
    // - Same preconditions as above
    // - Caller guarantees no other references to this memory exist
    let slice = unsafe { std::slice::from_raw_parts_mut(data, len) };
    slice.fill(0.0);
}
```
