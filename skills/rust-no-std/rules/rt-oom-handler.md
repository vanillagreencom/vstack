---
title: OOM Handler
impact: CRITICAL
tags: oom, alloc_error_handler, fallible-allocation, try_reserve
---

## OOM Handler

**Impact: CRITICAL (unhandled OOM panics crash embedded systems)**

When allocation fails, the default behavior is to panic. In embedded systems where memory is scarce, prefer fallible allocation APIs (`Vec::try_reserve`, `Box::try_new`) over infallible ones. On nightly, use `#[alloc_error_handler]` for custom OOM behavior. On stable, use `alloc::alloc::set_alloc_error_hook`.

**Incorrect (ignoring allocation failure):**

```rust
#![no_std]
extern crate alloc;
use alloc::vec::Vec;

fn collect_data(count: usize) -> Vec<u8> {
    // Panics on OOM — unacceptable in safety-critical embedded
    let mut v = Vec::with_capacity(count);
    v.resize(count, 0);
    v
}
```

**Correct (fallible allocation):**

```rust
#![no_std]
extern crate alloc;
use alloc::vec::Vec;

#[derive(Debug)]
pub enum Error {
    OutOfMemory,
}

fn collect_data(count: usize) -> Result<Vec<u8>, Error> {
    let mut v = Vec::new();
    v.try_reserve(count).map_err(|_| Error::OutOfMemory)?;
    v.resize(count, 0);
    Ok(v)
}
```

**Correct (custom alloc error handler on nightly):**

```rust
#![no_std]
#![feature(alloc_error_handler)]
extern crate alloc;

use core::alloc::Layout;

#[alloc_error_handler]
fn oom(_layout: Layout) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
```
