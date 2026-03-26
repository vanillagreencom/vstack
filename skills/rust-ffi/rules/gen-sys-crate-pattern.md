---
title: Sys Crate + Safe Wrapper Pattern
impact: HIGH
tags: sys-crate, wrapper, architecture, cargo
---

## Sys Crate + Safe Wrapper Pattern

**Impact: HIGH (mixing raw bindings with safe wrappers makes auditing impossible)**

Split into `mylib-sys` (raw bindings) + `mylib` (safe wrapper). sys crate: `links = "mylib"` in `Cargo.toml`, `build.rs` for linking, raw `extern "C"` declarations. Wrapper crate: safe Rust API, handle `Drop` cleanup, error conversion, lifetime tracking.

**Incorrect (raw bindings and safe API mixed in one crate):**

```rust
// lib.rs — everything in one place, unsafe scattered throughout
extern "C" {
    fn mylib_init() -> *mut Handle;
    fn mylib_process(h: *mut Handle, data: *const u8, len: usize) -> i32;
    fn mylib_free(h: *mut Handle);
}

pub fn process(data: &[u8]) -> Result<(), Error> {
    let h = unsafe { mylib_init() };  // raw FFI mixed with safe code
    // ...
}
```

**Correct (separate sys + wrapper crates):**

```toml
# mylib-sys/Cargo.toml
[package]
name = "mylib-sys"
links = "mylib"
build = "build.rs"
```

```rust
// mylib-sys/src/lib.rs — raw bindings only, no safe wrappers
#![allow(non_camel_case_types)]
extern "C" {
    pub fn mylib_init() -> *mut mylib_handle;
    pub fn mylib_process(h: *mut mylib_handle, data: *const u8, len: usize) -> i32;
    pub fn mylib_free(h: *mut mylib_handle);
}

#[repr(C)]
pub struct mylib_handle {
    _private: [u8; 0],
}
```

```rust
// mylib/src/lib.rs — safe wrapper
use mylib_sys as ffi;

pub struct Handle {
    inner: std::ptr::NonNull<ffi::mylib_handle>,
}

impl Handle {
    pub fn new() -> Result<Self, Error> {
        let ptr = unsafe { ffi::mylib_init() };
        let inner = std::ptr::NonNull::new(ptr).ok_or(Error::InitFailed)?;
        Ok(Self { inner })
    }

    pub fn process(&mut self, data: &[u8]) -> Result<(), Error> {
        let rc = unsafe {
            ffi::mylib_process(self.inner.as_ptr(), data.as_ptr(), data.len())
        };
        if rc != 0 { return Err(Error::from_code(rc)); }
        Ok(())
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { ffi::mylib_free(self.inner.as_ptr()) };
    }
}
```
