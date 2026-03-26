---
title: No SeqCst by Default
impact: CRITICAL
impactDescription: Unnecessary MFENCE instructions on hot paths
tags: atomics, seqcst, ordering, performance
---

## No SeqCst by Default

**Impact: CRITICAL (unnecessary MFENCE instructions on hot paths)**

`SeqCst` adds a full memory fence (MFENCE on x86) and is almost never necessary. From "Rust Atomics and Locks" (Mara Bos): "SeqCst ordering is almost never necessary in practice. In nearly all cases, regular acquire and release ordering suffice."

Every atomic operation must have an ordering justification in a comment. Defaulting to `SeqCst` "to be safe" is a code smell that indicates incomplete understanding of the data flow.

**Incorrect (SeqCst without justification):**

```rust
self.tail.store(next, Ordering::SeqCst);  // "just to be safe"
```

**Correct (minimal sufficient ordering with justification):**

```rust
// Release: publish data written to buffer[tail] before consumer sees new tail
self.tail.store(next, Ordering::Release);
```

### When SeqCst Is Actually Needed

SeqCst establishes a **single total order** across ALL SeqCst operations on ALL atomics. This is needed when two or more independent atomics must be observed in a globally consistent order by multiple threads.

**The Dekker-like proof (only SeqCst is correct):**

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

let x = Arc::new(AtomicBool::new(false));
let y = Arc::new(AtomicBool::new(false));

// Thread 1: store x, then read y
// Thread 2: store y, then read x
// Thread 3: read x then y
// Thread 4: read y then x

// With Release/Acquire: Thread 3 can see x=true,y=false
// while Thread 4 sees y=true,x=false — no global order.
// With SeqCst: impossible. All threads agree on one total order.
```

**Use SeqCst when:** Multiple independent atomics need globally consistent ordering (Dekker/Peterson mutex, total-order broadcast, sequence number agreement across unrelated atomics). **Use Acquire/Release when:** Only pairwise producer-consumer synchronization is needed (the vast majority of cases).
