---
title: Hidden format! Allocations
impact: MEDIUM
impactDescription: format! silently allocates a String on every call
tags: format, string, write, allocation
---

## Hidden format! Allocations

**Impact: MEDIUM (format! silently allocates a String on every call)**

`format!` creates a new `String` each invocation. In hot paths, write to a preallocated buffer instead.

**Incorrect (allocates on every call):**

```rust
let msg = format!("Price: {}", price);
```

**Correct (write to preallocated buffer):**

```rust
use std::fmt::Write;
let mut buf = String::with_capacity(100); // Preallocate once
write!(&mut buf, "Price: {}", price).unwrap();
```
