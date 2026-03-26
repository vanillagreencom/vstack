---
title: PhantomData Lifetime Binding for Borrowed Handles
impact: HIGH
tags: lifetime, PhantomData, borrow, use-after-free
---

## PhantomData Lifetime Binding for Borrowed Handles

**Impact: HIGH (use-after-free when borrowed handle outlives parent)**

Use `PhantomData<&'a ()>` to tie wrapper lifetime to parent. Borrowed handles: `struct Ref<'a> { ptr: *const T, _marker: PhantomData<&'a T> }`. This prevents use-after-free at compile time. For callbacks: ensure the closure outlives the C registration.

**Incorrect (borrowed handle without lifetime binding):**

```rust
pub struct DatabaseRef {
    ptr: *const ffi::db_ref,
}

impl Database {
    pub fn get_ref(&self) -> DatabaseRef {
        DatabaseRef { ptr: unsafe { ffi::db_get_ref(self.raw) } }
    }
}

// BUG: db_ref can outlive Database — use-after-free
let db_ref = {
    let db = Database::open("test.db").unwrap();
    db.get_ref() // db dropped here, db_ref now dangling
};
```

**Correct (lifetime-bound borrowed handle):**

```rust
use std::marker::PhantomData;

pub struct DatabaseRef<'a> {
    ptr: *const ffi::db_ref,
    _marker: PhantomData<&'a Database>,
}

impl Database {
    pub fn get_ref(&self) -> DatabaseRef<'_> {
        DatabaseRef {
            ptr: unsafe { ffi::db_get_ref(self.raw) },
            _marker: PhantomData,
        }
    }
}

// Compile error: db_ref borrows db, so it can't outlive it
// let db_ref = {
//     let db = Database::open("test.db").unwrap();
//     db.get_ref() // ERROR: db does not live long enough
// };
```
