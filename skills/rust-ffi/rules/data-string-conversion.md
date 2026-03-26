---
title: CStr/CString for C String Conversion
impact: CRITICAL
tags: strings, CStr, CString, null-terminated
---

## CStr/CString for C String Conversion

**Impact: CRITICAL (missing null terminator causes buffer overread)**

C strings are null-terminated, Rust strings are not. Use `CStr::from_ptr` for C-to-Rust (borrowed, zero-copy). Use `CString::new` for Rust-to-C (allocates, adds null terminator). `CString::new` can fail if input contains an interior null byte. Never use `str::as_ptr` for FFI — the resulting pointer is not null-terminated.

**Incorrect (using str::as_ptr for FFI):**

```rust
let name = "hello";
unsafe {
    // BUG: str::as_ptr is NOT null-terminated — C will read past the end
    c_set_name(name.as_ptr() as *const std::ffi::c_char);
}
```

**Correct (CStr for receiving, CString for sending):**

```rust
use std::ffi::{CStr, CString, c_char};

// C → Rust (borrowed, zero-copy):
unsafe fn read_c_string(ptr: *const c_char) -> Result<&str, std::str::Utf8Error> {
    // SAFETY: ptr is non-null and points to a valid null-terminated C string
    let cstr = unsafe { CStr::from_ptr(ptr) };
    cstr.to_str()
}

// Rust → C (allocates, adds null terminator):
fn send_to_c(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let c_name = CString::new(name)?; // Fails if name contains \0
    unsafe {
        c_set_name(c_name.as_ptr());
    }
    // c_name must live until C is done with the pointer
    Ok(())
}
```
