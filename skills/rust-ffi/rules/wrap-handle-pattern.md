---
title: Wrap C Handles in Newtype with Drop
impact: HIGH
tags: handle, newtype, Drop, Send, Sync
---

## Wrap C Handles in Newtype with Drop

**Impact: HIGH (resource leaks and unsound Send/Sync)**

Wrap raw C handles in a newtype struct. Implement `Drop` for cleanup (call C destructor). Don't implement `Clone` unless C supports ref-counting. Implement `Send`/`Sync` ONLY after verifying thread safety of the underlying C library. Mark `!Send`/`!Sync` explicitly with `PhantomData` if unsure.

**Incorrect (raw handle without wrapper):**

```rust
let handle = unsafe { c_lib_open(path.as_ptr()) };
// ... use handle ...
// Forgot to call c_lib_close(handle) — resource leak
// Also: is this handle safe to send to another thread?
```

**Correct (newtype with Drop and explicit thread safety):**

```rust
use std::marker::PhantomData;

pub struct LibHandle {
    raw: *mut ffi::c_lib_handle,
    // Mark as !Send + !Sync until C lib thread safety is verified
    _marker: PhantomData<*mut ()>,
}

impl LibHandle {
    pub fn open(path: &str) -> Result<Self, Error> {
        let c_path = std::ffi::CString::new(path)?;
        let raw = unsafe { ffi::c_lib_open(c_path.as_ptr()) };
        if raw.is_null() {
            return Err(Error::OpenFailed);
        }
        Ok(Self { raw, _marker: PhantomData })
    }
}

impl Drop for LibHandle {
    fn drop(&mut self) {
        // SAFETY: self.raw was created by c_lib_open and hasn't been closed
        unsafe { ffi::c_lib_close(self.raw) };
    }
}

// Only add after verifying the C library is thread-safe:
// unsafe impl Send for LibHandle {}
// unsafe impl Sync for LibHandle {}
```
