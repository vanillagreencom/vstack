---
title: String Operation Allocations
impact: MEDIUM
impactDescription: Common string methods silently create new String allocations
tags: string, to_uppercase, to_string, allocation
---

## String Operation Allocations

**Impact: MEDIUM (common string methods silently create new String allocations)**

Methods like `to_uppercase()`, `to_lowercase()`, and `to_string()` create new `String` allocations. Use in-place alternatives when possible.

**Incorrect (creates new String):**

```rust
let upper = s.to_uppercase();
```

**Correct (modify in place):**

```rust
s.make_ascii_uppercase();
```

Note: `make_ascii_uppercase()` only works for ASCII. For Unicode, the allocation from `to_uppercase()` is unavoidable -- keep it off the hot path.
