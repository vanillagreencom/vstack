# Lock-Free Safety Patterns

**Version 1.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when implementing,
> verifying, or reviewing lock-free data structures in Rust. Humans may also
> find it useful, but guidance here is optimized for automation and
> consistency by AI-assisted workflows.

---

## Nomenclature

- **SPSC** - Single-Producer, Single-Consumer queue
- **UB** - Undefined Behavior
- **MIRI** - Mid-level IR Interpreter (detects UB in unsafe Rust)
- **Loom** - Concurrency permutation testing framework
- **TSAN** - ThreadSanitizer (detects data races in mutex-based code)
- **ASAN** - AddressSanitizer (detects memory errors)
- **Epoch reclamation** - Deferred memory deallocation via crossbeam-epoch

---

## Abstract

Rules and patterns for building correct lock-free data structures in Rust, covering soundness requirements (UnsafeCell, ordering justification), verification tool selection (loom vs MIRI vs TSAN vs ASAN), atomic memory ordering (Acquire/Release for SPSC, ARM64 testing), epoch-based memory reclamation (crossbeam-epoch guard lifetimes), and loom test design. Each rule includes impact assessment and, where applicable, incorrect vs. correct code examples.

---

## Table of Contents

1. [Soundness](#1-soundness) — **CRITICAL**
   - 1.1 [UnsafeCell Required for SPSC Buffers](#11-unsafecell-required-for-spsc-buffers)
   - 1.2 [No SeqCst by Default](#12-no-seqcst-by-default)
2. [Verification](#2-verification) — **CRITICAL**
   - 2.1 [ThreadSanitizer Cannot Verify Atomic Fences](#21-threadsanitizer-cannot-verify-atomic-fences)
   - 2.2 [Verification Tool Selection Matrix](#22-verification-tool-selection-matrix)
   - 2.3 [Loom Tests Required for All Lock-Free Structures](#23-loom-tests-required-for-all-lock-free-structures)
3. [Ordering](#3-ordering) — **HIGH**
   - 3.1 [Acquire/Release Pattern for SPSC Queues](#31-acquirerelease-pattern-for-spsc-queues)
   - 3.2 [Test on ARM64 to Catch Weak-Memory Bugs](#32-test-on-arm64-to-catch-weak-memory-bugs)
   - 3.3 [Fence Batching for Multiple Stores](#33-fence-batching-for-multiple-stores)
4. [Epoch Reclamation](#4-epoch-reclamation) — **HIGH**
   - 4.1 [Pin Epoch Before Atomic Load](#41-pin-epoch-before-atomic-load)
   - 4.2 [Use defer_destroy for Epoch Deallocation](#42-use-defer_destroy-for-epoch-deallocation)
5. [Testing](#5-testing) — **MEDIUM**
   - 5.1 [Loom Model Design Best Practices](#51-loom-model-design-best-practices)
   - 5.2 [Scope MIRI to Unsafe Code Paths](#52-scope-miri-to-unsafe-code-paths)
6. [Quality Gates](#6-quality-gates)

---

## 1. Soundness

**Impact: CRITICAL**

Violations of these rules are undefined behavior. The compiler and sanitizers may not catch them, and they can cause silent data corruption or security vulnerabilities.

### 1.1 UnsafeCell Required for SPSC Buffers

**Impact: CRITICAL (undefined behavior from mutating through shared references)**

Casting `&T` to `*mut T` and mutating is undefined behavior. The only sound way to implement interior mutability in SPSC queue buffers is through `UnsafeCell`. MIRI will catch this violation.

For in-process Rust SPSC, prefer `rtrb` crate directly (battle-tested, MIRI-verified) over hand-rolling.

**Incorrect (mutating through shared reference is UB):**

```rust
buffer: Box<[CachePadded<Option<T>>]>,
unsafe {
    let slot = &self.buffer[idx] as *const _ as *mut Option<T>;
    (*slot) = Some(item);  // UB: mutating through shared reference
}
```

**Correct (UnsafeCell opts out of immutability guarantee):**

```rust
buffer: Box<[UnsafeCell<MaybeUninit<T>>]>,
unsafe {
    (*self.buffer[idx].get()).write(item);  // UnsafeCell::get() -> *mut T
}
```

### 1.2 No SeqCst by Default

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

#### When SeqCst Is Actually Needed

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

---

## 2. Verification

**Impact: CRITICAL**

Rules for choosing and applying the correct verification tool. Using the wrong tool gives false confidence — e.g., TSAN cannot verify atomic fences.

### 2.1 ThreadSanitizer Cannot Verify Atomic Fences

**Impact: CRITICAL (false confidence in lock-free correctness)**

ThreadSanitizer does NOT understand `std::sync::atomic::fence`. For any lock-free code using fences (SPSC queues, ring buffers, custom atomics), TSAN will report no errors even when bugs exist. Use loom testing instead.

TSAN is still valid for mutex-based synchronization and standard library channel primitives.

```
Is it lock-free code with atomic fences?
+-- YES -> Use Loom (TSAN won't catch issues)
+-- NO
    +-- Does it use syscalls or foreign code?
    |   +-- YES -> Use ASAN (MIRI can't test those)
    |   +-- NO  -> Use MIRI first, then ASAN
    +-- Is it mutex-based threading?
        +-- YES -> TSAN is reliable
        +-- NO  -> Evaluate case by case
```

### 2.2 Verification Tool Selection Matrix

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

**Tiered verification:**

**Tier 1 (Pre-Merge, BLOCKING):** Scope to changed modules using `git diff --name-only main...HEAD`.

```bash
# UB detection (scoped)
MIRIFLAGS="-Zmiri-disable-isolation" cargo +nightly miri test --lib <module_filter>

# Memory errors + leaks (scoped) — use --target to prevent proc-macro poisoning
RUSTFLAGS="-Zsanitizer=address" cargo +nightly test \
  -Zbuild-std --target <host-target> --lib <module_filter>
```

**Tier 2 (Comprehensive, NON-BLOCKING):**

```bash
# Full MIRI coverage
MIRIFLAGS="-Zmiri-disable-isolation" cargo +nightly miri test --lib

# Tree Borrows (experimental, stricter)
MIRIFLAGS="-Zmiri-disable-isolation -Zmiri-tree-borrows" cargo +nightly miri test --lib

# Lock-free verification
LOOM_MAX_PREEMPTIONS=2 RUSTFLAGS="--cfg loom" cargo test --features loom --release
```

**MIRI notes:** Only valuable on code paths exercising `unsafe` blocks. Safe Rust is compiler-guaranteed UB-free. Gate non-unsafe tests with `#[cfg(all(test, not(miri)))]`.

**ASAN notes:** Use `--target <host>` and `-Zbuild-std` to prevent proc-macro poisoning. LSAN is automatic on Linux; on macOS add `ASAN_OPTIONS=detect_leaks=1`. Not supported on Windows.

### 2.3 Loom Tests Required for All Lock-Free Structures

**Impact: CRITICAL (concurrency bugs undetectable by other tools)**

Every lock-free structure (SPSC queues, ring buffers, atomic state machines) MUST have loom tests. Loom is the only tool that can verify correctness of atomic fence patterns by exploring all possible thread interleavings.

Use `LOOM_MAX_PREEMPTIONS=2` for CI (sufficient for 2-thread SPSC). Use 3 for deep runs on critical structures.

```bash
# CI standard
LOOM_MAX_PREEMPTIONS=2 RUSTFLAGS="--cfg loom" cargo test --features loom --release

# Thorough (for critical structures)
LOOM_MAX_PREEMPTIONS=3 RUSTFLAGS="--cfg loom" cargo test --features loom --release

# Debug failures
LOOM_LOG=trace LOOM_MAX_PREEMPTIONS=2 RUSTFLAGS="--cfg loom" cargo test --features loom
```

---

## 3. Ordering

**Impact: HIGH**

Atomic memory ordering rules. Incorrect ordering causes data races on weakly-ordered architectures (ARM64) that may not manifest on x86.

### 3.1 Acquire/Release Pattern for SPSC Queues

**Impact: HIGH (data races on ARM64 or stale reads on weakly-ordered architectures)**

SPSC queues need exactly three ordering levels: Relaxed for own-index loads, Acquire to read the other thread's progress, Release to publish data. This pattern is sufficient for correctness and avoids unnecessary memory fences.

```rust
// Producer
let tail = self.tail.load(Ordering::Relaxed);    // Own index — no sync needed
if next != self.head.load(Ordering::Acquire) {   // Read consumer's progress
    buffer[tail].write(item);
    self.tail.store(next, Ordering::Release);     // Publish data
}

// Consumer
let head = self.head.load(Ordering::Relaxed);    // Own index — no sync needed
if head != self.tail.load(Ordering::Acquire) {   // Read producer's progress
    let item = buffer[head].read();
    self.head.store(next, Ordering::Release);     // Signal slot free
}
```

**Quick reference:**

| Ordering | Use Case | Notes |
|----------|----------|-------|
| `Relaxed` | Counters, own-index loads in SPSC | No happens-before |
| `Acquire` | Load before reading shared data | Pairs with Release |
| `Release` | Store after writing shared data | Pairs with Acquire |
| `AcqRel` | Read-modify-write (CAS) | Both Acquire + Release |
| `SeqCst` | **Almost never needed** | Adds MFENCE, rarely justified |

### 3.2 Test on ARM64 to Catch Weak-Memory Bugs

**Impact: HIGH (ordering bugs hidden by x86 strong memory model)**

x86 has a strong memory model (TSO) that masks many ordering bugs. ARM64 (Apple Silicon, AWS Graviton) has a weakly-ordered memory model where missing Acquire/Release fences cause real failures. Always test lock-free code on ARM64 in addition to x86.

Loom's model checker does explore weak-memory orderings, but running real tests on ARM64 hardware catches issues in library code and compiler code generation that loom's model doesn't cover.

#### Platform Barrier Cost

Memory ordering has different hardware costs per architecture:

| Architecture | Relaxed | Acquire | Release | AcqRel | SeqCst |
|-------------|---------|---------|---------|--------|--------|
| x86/x86_64 (TSO) | free | free | free | free | `mfence` or `lock` prefix |
| ARM64 | free | `dmb ishld` | `dmb ish` | `dmb ish` | `dmb ish` + `dmb ish` |
| POWER | free | `lwsync` + `isync` | `lwsync` | `lwsync` | `sync` |
| RISC-V (RVWMO) | free | `fence r,rw` | `fence rw,w` | `fence.tso` | `fence rw,rw` |

x86 TSO gives Acquire/Release for free — the hardware enforces store-to-load ordering by default. Only SeqCst requires an explicit `mfence`. This means ordering bugs on x86 are **silent** until deployed on ARM64 or tested under loom.

This is why ARM64 testing is not optional — it is the only way to surface ordering bugs that x86 TSO masks.

### 3.3 Fence Batching for Multiple Stores

**Impact: HIGH (unnecessary per-store Release ordering adds barrier overhead)**

When publishing multiple related values, use Relaxed stores followed by a single `fence(Release)` and a Relaxed sentinel store, instead of making every store Release. This is semantically equivalent but can be more efficient — one barrier instead of N.

**Incorrect (per-store Release — redundant barriers):**

```rust
use std::sync::atomic::{AtomicU64, Ordering};

static DATA_A: AtomicU64 = AtomicU64::new(0);
static DATA_B: AtomicU64 = AtomicU64::new(0);
static DATA_C: AtomicU64 = AtomicU64::new(0);
static READY: AtomicU64 = AtomicU64::new(0);

// Publisher: 3 Release stores + 1 Release store = 4 barriers on ARM64
fn publish(a: u64, b: u64, c: u64) {
    DATA_A.store(a, Ordering::Release);
    DATA_B.store(b, Ordering::Release);
    DATA_C.store(c, Ordering::Release);
    READY.store(1, Ordering::Release);
}
```

**Correct (fence batching — single barrier):**

```rust
use std::sync::atomic::{self, AtomicU64, Ordering};

static DATA_A: AtomicU64 = AtomicU64::new(0);
static DATA_B: AtomicU64 = AtomicU64::new(0);
static DATA_C: AtomicU64 = AtomicU64::new(0);
static READY: AtomicU64 = AtomicU64::new(0);

// Publisher: 3 Relaxed stores + 1 fence + 1 Relaxed sentinel = 1 barrier
fn publish(a: u64, b: u64, c: u64) {
    DATA_A.store(a, Ordering::Relaxed);
    DATA_B.store(b, Ordering::Relaxed);
    DATA_C.store(c, Ordering::Relaxed);
    atomic::fence(Ordering::Release);
    READY.store(1, Ordering::Relaxed);
}

// Consumer: Acquire fence after sentinel check
fn consume() -> Option<(u64, u64, u64)> {
    if READY.load(Ordering::Relaxed) == 1 {
        atomic::fence(Ordering::Acquire);
        Some((
            DATA_A.load(Ordering::Relaxed),
            DATA_B.load(Ordering::Relaxed),
            DATA_C.load(Ordering::Relaxed),
        ))
    } else {
        None
    }
}
```

The fence ensures all prior Relaxed stores are visible to any thread that observes the sentinel via an Acquire fence. On x86 this has no measurable difference (TSO makes Release free), but on ARM64 it reduces N `dmb ish` barriers to 1.

**When to use:** Batched updates (multiple fields published together), ring buffer metadata updates, snapshot publishing. **When NOT to use:** Single publish (just use Release directly), unclear ordering requirements (prefer explicit per-variable ordering for clarity).

**Requirement:** Every fence-based pattern must have loom test coverage per `verify-tsan-no-fences` — TSAN cannot verify fences.

---

## 4. Epoch Reclamation

**Impact: HIGH**

Rules for safe use of epoch-based memory reclamation (crossbeam-epoch). Violations cause use-after-free in lock-free data structures.

### 4.1 Pin Epoch Before Atomic Load

**Impact: HIGH (use-after-free from reading reclaimed memory)**

Every atomic load from a crossbeam-epoch `Atomic<T>` must be preceded by `epoch::pin()`. The returned `Guard` keeps the current epoch alive, preventing reclamation of data you're reading. References must not escape the guard's lifetime.

**Incorrect (reference escapes guard lifetime):**

```rust
fn unsafe_read<T>(atomic: &Atomic<T>) -> Option<&T> {
    let guard = epoch::pin();
    let shared = atomic.load(Ordering::Acquire, &guard);
    // WRONG: Reference escapes guard lifetime — data may be reclaimed
    shared.as_ref()
}
```

**Correct (process within guard scope, return owned data):**

```rust
fn safe_read<T>(atomic: &Atomic<T>) -> Option<ProcessedResult> {
    let guard = epoch::pin();  // Pin BEFORE reading
    let shared = atomic.load(Ordering::Acquire, &guard);
    // Process WITHIN guard scope — return owned result
    shared.as_ref().map(|data| process(data))
    // Guard dropped here, data may be reclaimed after
}
```

### 4.2 Use defer_destroy for Epoch Deallocation

**Impact: HIGH (use-after-free or double-free from manual drop mixed with epoch reclamation)**

When removing nodes from an epoch-protected data structure, use `defer_destroy()` to schedule safe deallocation. Never mix manual `drop` with epoch reclamation — other threads may still hold references through pinned guards.

**Checklist for epoch-based structures:**

- Every atomic load is preceded by `epoch::pin()`
- Shared references don't escape guard lifetime
- `defer_destroy()` used for safe deallocation
- No mixing of manual drop with epoch reclamation

---

## 5. Testing

**Impact: MEDIUM**

Best practices for loom model testing. Poor test design leads to state space explosion or insufficient coverage.

### 5.1 Loom Model Design Best Practices

**Impact: MEDIUM (state space explosion or insufficient coverage from poor test design)**

Loom explores all thread interleavings, which grows exponentially. Design models carefully to keep the state space tractable while still covering critical properties.

**Best practices:**

1. **Always use `loom::thread::yield_now()`** in spin/retry loops — loom needs explicit yield points to explore interleavings
2. **Keep models small** — loom explores all interleavings exponentially; use small buffer sizes (e.g., 4 slots) to force edge cases within exploration budget
3. **Test one property per model** — easier to debug failures and limits state space
4. **Use `LOOM_LOG=trace`** for debugging failed models
5. **Test on ARM64** (Apple Silicon) — x86's strong ordering hides weak-memory bugs that loom's model may not cover

**Key properties to test:**

- **Wraparound**: Head/tail indices wrap around ring buffer capacity boundary
- **Sequential ordering**: FIFO preserved — push N items, pop N, assert order matches
- **Multi-variant payloads**: Enum payloads (different sizes, discriminants) transit correctly through ordering boundary

**SPSC model skeleton:**

```rust
use loom::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use loom::sync::Arc;
use loom::thread;

#[test]
fn loom_spsc_ordering() {
    loom::model(|| {
        let head = Arc::new(AtomicUsize::new(0));
        let tail = Arc::new(AtomicUsize::new(0));
        let data: Arc<[AtomicU64; 4]> = Arc::new(
            std::array::from_fn(|_| AtomicU64::new(0))
        );

        // Producer: write data, Release-store tail
        // Consumer: Acquire-load tail, read data, Release-store head
    });
}
```

Loom cannot intercept third-party crate internals (e.g., `rtrb`). Test the Acquire/Release ordering pattern through simplified in-memory models that mirror the real implementation's synchronization.

### 5.2 Scope MIRI to Unsafe Code Paths

**Impact: MEDIUM (wasted CI time running MIRI on safe code that cannot have UB)**

MIRI is only valuable on code paths exercising `unsafe` blocks. Safe Rust is compiler-guaranteed UB-free; MIRI adds nothing there. Running MIRI is ~1000x slower than native execution, so blanket `--lib` is prohibitive.

Scope MIRI runs to changed modules using `git diff` filters. Gate tests that don't reach unsafe code:

```rust
#[cfg(all(test, not(miri)))]
mod safe_only_tests {
    // These tests don't exercise unsafe code — skip under MIRI
}
```

**MIRI catches:** uninitialized memory, out-of-bounds access, use-after-free, aliasing violations (Stacked/Tree Borrows).

**MIRI cannot test:** syscalls, foreign code (no shims), multithreaded code (use loom instead).

```bash
# Scoped to changed modules (recommended)
MIRIFLAGS="-Zmiri-disable-isolation" cargo +nightly miri test --lib <module_filter>

# Tree Borrows (experimental, stricter aliasing model)
MIRIFLAGS="-Zmiri-disable-isolation -Zmiri-tree-borrows" cargo +nightly miri test --lib
```

---

## 6. Quality Gates

Checklist for lock-free code before merge:

- [ ] Uses `UnsafeCell<MaybeUninit<T>>` for SPSC buffers (not raw pointer casts)
- [ ] Loom tests pass (`LOOM_MAX_PREEMPTIONS=2` for CI)
- [ ] Tests pass on ARM64 — catches weak-memory bugs
- [ ] Crossbeam epoch Guard lifetimes verified
- [ ] No `atomic::fence` without loom coverage
- [ ] Ordering justification in comments (why not SeqCst?)
- [ ] MIRI passes on unsafe code paths
