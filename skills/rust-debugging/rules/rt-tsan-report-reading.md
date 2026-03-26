---
title: TSan Report Reading
impact: HIGH
tags: tsan, thread-sanitizer, data-race, unsafe, ffi
---

## TSan Report Reading

**Impact: HIGH (misreading TSan reports leads to ignoring real races or chasing false positives)**

ThreadSanitizer (TSan) reports have a specific structure: first section shows the racing write, second shows the conflicting access, third shows thread creation points. Understanding this structure is essential for diagnosing data races in `unsafe` code and FFI boundaries that Rust's type system cannot prevent.

**Incorrect (ignoring or misinterpreting TSan output):**

```rust
// Running with TSan and seeing output like:
// WARNING: ThreadSanitizer: data race
// Developer ignores it because "Rust prevents data races"
// — but TSan catches races in unsafe blocks and FFI that
// the borrow checker can't see

// Or: developer sees Arc<T> reference counting flagged
// and wastes time investigating a false positive
```

**Correct (systematic TSan report analysis):**

```rust
// TSan report structure:
//
// Section 1 — The racing access:
//   Write of size 8 at 0x7f1234 by thread T2:
//     #0 myapp::engine::update src/engine.rs:45
//     #1 myapp::engine::run    src/engine.rs:120
//
// Section 2 — The conflicting previous access:
//   Previous read of size 8 at 0x7f1234 by thread T1:
//     #0 myapp::engine::read_state src/engine.rs:30
//     #1 myapp::engine::poll       src/engine.rs:88
//
// Section 3 — Thread creation points:
//   Thread T1 (tid=12345, running) created by main thread at:
//     #0 std::thread::spawn ...
//   Thread T2 (tid=12346, running) created by main thread at:
//     #0 std::thread::spawn ...

// Map hex addresses to source lines:
// $ addr2line -e target/debug/myapp 0x55a1234

// Common Rust false positives (safe to suppress):
// - Arc reference counting: TSan doesn't understand atomic guarantees
// - lazy_static initialization: one-time race, harmless
// - std::sync::Once internals

// Suppress known false positives:
// Create tsan.supp:
//   race:std::sync::Arc
//   race:lazy_static
//
// Run with: TSAN_OPTIONS="suppressions=tsan.supp" ./target/debug/myapp

// Build with TSan enabled:
// RUSTFLAGS="-Z sanitizer=thread" cargo +nightly build
// RUSTFLAGS="-Z sanitizer=thread" cargo +nightly test
```

**TSan report cheat sheet:**

| Report Section | What It Shows |
|----------------|---------------|
| First access block | The racing write/read (file:line + thread) |
| Second access block | The conflicting previous access |
| Thread creation | Where each involved thread was spawned |
| `addr2line -e binary 0xaddr` | Map address to source file:line |

**Key principle:** TSan catches races in `unsafe` code and FFI boundaries that Rust's type system cannot prevent. Safe Rust code should never produce real TSan warnings — if it does, there is a compiler or standard library bug.
