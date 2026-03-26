---
title: Catch Panics at FFI Callback Boundary
impact: HIGH
tags: callback, panic, unwind, catch_unwind
---

## Catch Panics at FFI Callback Boundary

**Impact: HIGH (unwinding through C frames is undefined behavior)**

C callbacks must be `extern "C" fn`. Use `Box::into_raw` for closure context, `Box::from_raw` in destructor. Catch panics at FFI boundary: `std::panic::catch_unwind` — unwinding through C frames is UB. Always register an unwind destructor.

**Incorrect (panic can unwind through C):**

```rust
extern "C" fn my_callback(ctx: *mut std::ffi::c_void) {
    let data = unsafe { &*(ctx as *const MyData) };
    // If this panics, the unwind crosses C frames — UB
    data.process().unwrap();
}
```

**Correct (panic caught at boundary):**

```rust
extern "C" fn my_callback(ctx: *mut std::ffi::c_void) -> i32 {
    let result = std::panic::catch_unwind(|| {
        // SAFETY: ctx was created by Box::into_raw in register_callback
        let data = unsafe { &*(ctx as *const MyData) };
        data.process()
    });

    match result {
        Ok(Ok(())) => 0,      // Success
        Ok(Err(_)) => -1,     // Application error
        Err(_) => {
            // Panic caught — log and return error code
            eprintln!("panic in FFI callback");
            -2
        }
    }
}

// Register with closure context:
pub fn register_callback(data: MyData) {
    let boxed = Box::new(data);
    let ctx = Box::into_raw(boxed) as *mut std::ffi::c_void;
    unsafe { ffi::set_callback(Some(my_callback), ctx) };
}

// Destructor to reclaim the context:
extern "C" fn destroy_callback_ctx(ctx: *mut std::ffi::c_void) {
    if !ctx.is_null() {
        // SAFETY: ctx was created by Box::into_raw in register_callback
        unsafe { drop(Box::from_raw(ctx as *mut MyData)) };
    }
}
```
