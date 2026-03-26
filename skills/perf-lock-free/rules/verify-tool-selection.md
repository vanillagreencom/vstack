---
title: Verification Tool Selection Matrix
impact: CRITICAL
impactDescription: Wrong tool gives false confidence in safety
tags: miri, loom, tsan, asan, valgrind, verification
---

## Verification Tool Selection Matrix

**Impact: CRITICAL (wrong tool gives false confidence in safety)**

Each verification tool has blind spots. Use the correct tool for each code category.

| Tool | Use For | Limitation | Command |
|------|---------|------------|---------|
| MIRI | Aliasing, UB, memory | No foreign code, no threads | `MIRIFLAGS="-Zmiri-disable-isolation" cargo +nightly miri test --lib` |
| Loom | Lock-free correctness | Requires test setup, slow | `LOOM_MAX_PREEMPTIONS=2 RUSTFLAGS="--cfg loom" cargo test --features loom --release` |
| TSAN | Data races (mutex-based) | **No atomic fences** | `RUSTFLAGS="-Z sanitizer=thread" cargo +nightly test --lib` |
| ASAN | Memory errors | ~2x slowdown, no Windows | `RUSTFLAGS="-Zsanitizer=address" cargo +nightly test -Zbuild-std --target <host> --lib` |
| Valgrind | Leaks, detailed | ~20x slowdown | `valgrind --leak-check=full ./target/release/<binary>` |

**Scope division:**

| Code Category | MIRI | ASAN | Loom | Notes |
|--------------|------|------|------|-------|
| Ring buffer atomics | Partial | Yes | Yes | MIRI limited to single-thread paths |
| Raw pointer paths | No | Yes | No | MIRI can't test syscalls or foreign code |
| Syscalls (mlockall, etc.) | No | Yes | No | MIRI has no shims for these |
| Pure Rust aliasing | Yes | No | No | MIRI's unique strength |
| Lock-free with fences | No | No | Yes | Loom is the only option |

**MIRI notes:** Only valuable on code paths exercising `unsafe` blocks. Safe Rust is compiler-guaranteed UB-free. Gate non-unsafe tests with `#[cfg(all(test, not(miri)))]`.

**ASAN notes:** Use `--target <host>` and `-Zbuild-std` to prevent proc-macro poisoning. LSAN is automatic on Linux; on macOS add `ASAN_OPTIONS=detect_leaks=1`.
