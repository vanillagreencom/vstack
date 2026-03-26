# Safety Audit Patterns

**Version 1.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when auditing,
> reviewing, or writing unsafe Rust code. Humans may also find it useful,
> but guidance here is optimized for automation and consistency by
> AI-assisted workflows.

---

## Abstract

Checklists and rules for auditing unsafe Rust code, prioritized by impact from critical (SAFETY comments, memory safety, pointer invariants) to high (lock-free structures, crossbeam epoch, security) to medium (severity classification). Each rule includes detailed explanations and, where applicable, incorrect vs. correct code examples.

---

## Table of Contents

1. [SAFETY Comments](#1-safety-comments) — **CRITICAL**
   - 1.1 [SAFETY Comment Standard](#11-safety-comment-standard)
   - 1.2 [SAFETY Comment Audit Questions](#12-safety-comment-audit-questions)
2. [Unsafe Block Audit](#2-unsafe-block-audit) — **CRITICAL**
   - 2.1 [Unsafe Block Checklist](#21-unsafe-block-checklist)
3. [Memory Safety](#3-memory-safety) — **CRITICAL**
   - 3.1 [No Use-After-Free](#31-no-use-after-free)
   - 3.2 [No Double-Free](#32-no-double-free)
   - 3.3 [No Uninitialized Reads](#33-no-uninitialized-reads)
   - 3.4 [No Out-of-Bounds Access](#34-no-out-of-bounds-access)
   - 3.5 [No Data Races](#35-no-data-races)
4. [Raw Pointer Audit](#4-raw-pointer-audit) — **CRITICAL**
   - 4.1 [Pointer Provenance Tracking](#41-pointer-provenance-tracking)
   - 4.2 [Pointer Validity Before Dereference](#42-pointer-validity-before-dereference)
   - 4.3 [Pointer Alignment Verification](#43-pointer-alignment-verification)
   - 4.4 [Pointer Lifetime Guarantee](#44-pointer-lifetime-guarantee)
   - 4.5 [Pointer Aliasing Rules](#45-pointer-aliasing-rules)
5. [Lock-Free Structures](#5-lock-free-structures) — **HIGH**
   - 5.1 [Loom Test Coverage](#51-loom-test-coverage)
   - 5.2 [Atomic Ordering Justification](#52-atomic-ordering-justification)
   - 5.3 [No Fence Without Loom Coverage](#53-no-fence-without-loom-coverage)
   - 5.4 [Safe Memory Reclamation](#54-safe-memory-reclamation)
   - 5.5 [ABA Problem Prevention](#55-aba-problem-prevention)
6. [Crossbeam Epoch](#6-crossbeam-epoch) — **HIGH**
   - 6.1 [Pin Before Atomic Load](#61-pin-before-atomic-load)
   - 6.2 [Guard Lifetime Contains All Access](#62-guard-lifetime-contains-all-access)
   - 6.3 [Deferred Destruction for Cleanup](#63-deferred-destruction-for-cleanup)
   - 6.4 [No Manual Drop Mixed with Epoch](#64-no-manual-drop-mixed-with-epoch)
   - 6.5 [No Shared Reference Scope Escape](#65-no-shared-reference-scope-escape)
7. [Security](#7-security) — **HIGH**
   - 7.1 [Input Validation Before Use](#71-input-validation-before-use)
   - 7.2 [No Panics on Malformed Data](#72-no-panics-on-malformed-data)
   - 7.3 [DoS Prevention](#73-dos-prevention)
   - 7.4 [Error Propagation](#74-error-propagation)
   - 7.5 [Checked Arithmetic for Overflow](#75-checked-arithmetic-for-overflow)
8. [Severity Classification](#8-severity-classification) — **MEDIUM**
   - 8.1 [Severity Classification](#81-severity-classification)
9. [Sanitizers](#9-sanitizers) — **HIGH**
   - 9.1 [ThreadSanitizer and MemorySanitizer](#91-threadsanitizer-and-memorysanitizer)
   - 9.2 [Miri for Compile-Time UB Detection](#92-miri-for-compile-time-ub-detection)
   - 9.3 [Sanitizer CI Integration](#93-sanitizer-ci-integration)
10. [Fuzzing](#10-fuzzing) — **HIGH**
    - 10.1 [Fuzz Target Selection](#101-fuzz-target-selection)
11. [Supply Chain](#11-supply-chain) — **MEDIUM**
    - 11.1 [Dependency Hygiene](#111-dependency-hygiene)

---

## 1. SAFETY Comments

**Impact: CRITICAL**

Standards for documenting unsafe code. Every unsafe block requires a SAFETY comment proving soundness — covering validity, alignment, aliasing, initialization, and lifetime.

### 1.1 SAFETY Comment Standard

**Impact: CRITICAL (unsafe blocks without justification are unsound by default)**

Every `unsafe` block requires a `// SAFETY:` comment explaining why it is sound. The comment must address all applicable invariants for the operations performed.

#### Required Elements

Each SAFETY comment must cover every applicable item:

1. **Validity** — Why is the pointer/reference valid?
2. **Alignment** — How do we know alignment is correct?
3. **Aliasing** — Why are there no conflicting references?
4. **Initialization** — How do we know memory is initialized?
5. **Lifetime** — Why does the data outlive its use?

**Incorrect (missing or incomplete SAFETY comment):**

```rust
unsafe {
    let value = ptr::read(ptr);
}
```

**Correct (complete SAFETY comment with verifiable claims):**

```rust
unsafe {
    // SAFETY:
    // - ptr is valid: checked non-null on line 42
    // - ptr is aligned: guaranteed by allocator (8-byte aligned)
    // - No aliasing: unique ownership via Box::into_raw
    // - Memory initialized: written on line 44
    let value = ptr::read(ptr);
}
```

### 1.2 SAFETY Comment Audit Questions

**Impact: CRITICAL (unverified safety claims mask unsoundness)**

When reviewing SAFETY comments, verify each claim against the code:

- Is each claim verifiable from the surrounding code?
- Are line number references accurate and up to date?
- Do invariants hold across ALL call sites, not just the obvious one?
- What happens if preconditions are violated — panic, UB, or graceful error?

Every SAFETY comment must be provably correct from local context. If a claim requires understanding distant code, the invariant should be enforced closer to the unsafe block (e.g., a wrapper type with a validity invariant).

---

## 2. Unsafe Block Audit

**Impact: CRITICAL**

Per-block checklist for auditing unsafe Rust code. Covers SAFETY comments, pointer validation, UB analysis, test coverage, and panic-path safety.

### 2.1 Unsafe Block Checklist

**Impact: CRITICAL (missed unsafe blocks are unaudited attack surface)**

For each `unsafe` block in the codebase:

- [ ] SAFETY comment present and complete (covers all applicable invariants)
- [ ] All pointer operations validated (null checks, bounds checks)
- [ ] No undefined behavior possible (verified by analysis or MIRI)
- [ ] MIRI test coverage exists (only meaningful if the block exercises unsafe operations)
- [ ] Panic paths don't leave invalid state (no partial writes visible after unwind)

Run an unsafe inventory tool to enumerate all unsafe blocks and verify none are missed during audit.

---

## 3. Memory Safety

**Impact: CRITICAL**

Memory safety invariants that must hold for all unsafe code — no use-after-free, double-free, uninitialized reads, out-of-bounds access, or data races.

### 3.1 No Use-After-Free

**Impact: CRITICAL (reading freed memory is undefined behavior)**

Verify that every pointer or reference derived from allocated memory is not used after the allocation is freed. Common sources: `Box::into_raw` followed by `Box::from_raw` (consuming the allocation) while a raw pointer copy still exists, or references into a `Vec` that is subsequently reallocated.

### 3.2 No Double-Free

**Impact: CRITICAL (freeing memory twice is undefined behavior)**

Verify that ownership is transferred exactly once. Common sources: calling `Box::from_raw` on the same pointer twice, or manually dropping a value that will also be dropped by its owner. Use `ManuallyDrop` or `mem::forget` when ownership must be surrendered without running the destructor.

### 3.3 No Uninitialized Reads

**Impact: CRITICAL (reading uninitialized memory is undefined behavior)**

Verify that all memory is fully initialized before being read. Use `MaybeUninit` for deferred initialization and call `assume_init()` only after every byte has been written. Never use `mem::uninitialized()` (deprecated and always UB for inhabited types).

### 3.4 No Out-of-Bounds Access

**Impact: CRITICAL (out-of-bounds access is undefined behavior)**

Verify that all pointer arithmetic and slice indexing stays within allocated bounds. Check `offset()`, `add()`, `sub()` calls against the allocation size. For slices created from raw parts (`slice::from_raw_parts`), verify the length does not exceed the allocation.

### 3.5 No Data Races

**Impact: CRITICAL (concurrent unsynchronized access is undefined behavior)**

Verify that shared mutable state is protected by proper synchronization: mutex, atomic operations with correct ordering, or verified lock-free algorithms (loom-tested). Two threads accessing the same memory where at least one is writing without synchronization is always undefined behavior, even if it "works" in practice.

---

## 4. Raw Pointer Audit

**Impact: CRITICAL**

Per-pointer checklist for raw pointer usage — provenance tracking, validity, alignment, lifetime, and aliasing rules.

### 4.1 Pointer Provenance Tracking

**Impact: CRITICAL (lost provenance causes undefined behavior under strict provenance rules)**

For each raw pointer, track where it came from (its provenance). A pointer must be derived from a valid allocation and must not be fabricated from an integer without `with_addr()` or `from_exposed_addr()`. Document the provenance chain in the SAFETY comment.

### 4.2 Pointer Validity Before Dereference

**Impact: CRITICAL (dereferencing an invalid pointer is undefined behavior)**

Every raw pointer must be validated before dereference. Check: non-null, within allocated object, not dangling (allocation still live). For pointers received from external code (FFI, callbacks), validate at the boundary before any use.

### 4.3 Pointer Alignment Verification

**Impact: CRITICAL (misaligned pointer dereference is undefined behavior)**

Verify that the pointer's alignment matches the pointee type's alignment requirement. Common sources of misalignment: casting between pointer types with different alignment (`*const u8` to `*const u64`), pointer arithmetic that breaks alignment, and packed struct field references.

### 4.4 Pointer Lifetime Guarantee

**Impact: CRITICAL (dangling pointer dereference is undefined behavior)**

Verify that the pointed-to data outlives every use of the pointer. Common violations: returning a pointer to a local variable, storing a pointer into a collection that outlives the source allocation, or holding a raw pointer across a `Vec` reallocation that invalidates it.

### 4.5 Pointer Aliasing Rules

**Impact: CRITICAL (aliasing violations cause undefined behavior under Stacked Borrows)**

Verify the single-writer-or-multiple-readers invariant: at any point, either exactly one `*mut T` is writing, OR one or more `*const T` are reading — never both simultaneously. Creating `&mut T` from a raw pointer invalidates all other pointers to the same memory under Stacked Borrows. Use `UnsafeCell` when interior mutability through shared references is required.

---

## 5. Lock-Free Structures

**Impact: HIGH**

Audit rules for lock-free data structures — loom testing, atomic ordering justification, fence coverage, memory reclamation, and ABA prevention.

### 5.1 Loom Test Coverage

**Impact: HIGH (untested lock-free code has undetectable ordering bugs)**

Every lock-free data structure (SPSC queues, ring buffers, atomic structures) must have loom tests that pass. Loom exhaustively explores thread interleavings to find ordering bugs that are nearly impossible to reproduce with standard testing.

### 5.2 Atomic Ordering Justification

**Impact: HIGH (wrong ordering silently permits data races)**

Every atomic operation must have its memory ordering documented and justified in a comment. State which happens-before relationships the ordering establishes and why weaker orderings are insufficient. Common mistake: using `Relaxed` where `Release`/`Acquire` pairs are needed to synchronize non-atomic data.

### 5.3 No Fence Without Loom Coverage

**Impact: HIGH (fences without ordering verification may be insufficient or redundant)**

Never add an `atomic::fence` without corresponding loom test coverage proving it is both necessary and sufficient. Fences are easy to misplace and their effects are non-local — loom testing is the only reliable way to verify correctness.

### 5.4 Safe Memory Reclamation

**Impact: HIGH (incorrect reclamation causes use-after-free in concurrent code)**

Lock-free structures that remove nodes must use a safe reclamation scheme: epoch-based (crossbeam), hazard pointers, or exclusive ownership transfer. Verify that no reader can hold a reference to memory being reclaimed. Document the chosen scheme in the SAFETY comment.

### 5.5 ABA Problem Prevention

**Impact: HIGH (ABA causes silent corruption in CAS-based structures)**

For structures using compare-and-swap on pointers, verify that the ABA problem is addressed. A pointer value can be reused after free, causing a CAS to succeed when the underlying data has changed. Mitigation strategies: tagged pointers (generation counters), epoch-based reclamation (prevents reuse during active epoch), or hazard pointers.

---

## 6. Crossbeam Epoch

**Impact: HIGH**

Audit rules specific to crossbeam epoch-based memory reclamation — pin before load, guard lifetime, deferred destruction, and scope escapes.

### 6.1 Pin Before Atomic Load

**Impact: HIGH (loading without pin allows reclamation during read)**

When using crossbeam epoch, `epoch::pin()` must be called before every atomic load that accesses shared data. The guard returned by `pin()` prevents the current epoch from advancing, ensuring referenced data is not reclaimed while being read.

### 6.2 Guard Lifetime Contains All Access

**Impact: HIGH (data access outside guard scope is use-after-free)**

The epoch guard's lifetime must contain all access to epoch-protected data. Never let a reference to epoch-protected data escape the guard's scope — once the guard is dropped, the referenced memory may be reclaimed.

**Incorrect (reference escapes guard scope):**

```rust
let value = {
    let guard = epoch::pin();
    shared.load(Ordering::Acquire, &guard)
}; // guard dropped — value is now dangling
```

**Correct (access within guard scope):**

```rust
let guard = epoch::pin();
let value = shared.load(Ordering::Acquire, &guard);
process(value); // guard still live
drop(guard);    // safe: value no longer used
```

### 6.3 Deferred Destruction for Cleanup

**Impact: HIGH (immediate drop of shared data causes use-after-free)**

Use `defer_destroy()` (or equivalent deferred cleanup) for epoch-protected data that is being removed. Do not mix manual `drop` with epoch reclamation — deferred destruction ensures all current readers have exited their critical sections before memory is freed.

### 6.4 No Manual Drop Mixed with Epoch

**Impact: HIGH (manual drop bypasses deferred reclamation safety)**

Never manually drop epoch-protected data. Manually calling `drop()` or using `Box::from_raw()` on epoch-protected pointers bypasses the deferred reclamation mechanism, potentially freeing memory while other threads still hold references through their pinned guards.

### 6.5 No Shared Reference Scope Escape

**Impact: HIGH (escaped references become dangling after guard drop)**

Shared references obtained through an epoch guard must not escape the guard's scope. Collect owned copies of the data you need before dropping the guard. If you need to return data from an epoch-protected section, clone or copy it into owned storage within the guard's lifetime.

---

## 7. Security

**Impact: HIGH**

Security audit rules for code handling external input or exposed via unsafe interfaces — input validation, panic prevention, DoS mitigation, and checked arithmetic.

### 7.1 Input Validation Before Use

**Impact: HIGH (unbounded input enables buffer overflows and memory corruption)**

All inputs from external sources must be bounds-checked before use in unsafe code or allocation. Verify lengths, ranges, and formats at the boundary. Never pass unchecked external data directly to pointer arithmetic, slice construction, or allocation size calculations.

### 7.2 No Panics on Malformed Data

**Impact: HIGH (panics on external input enable denial of service)**

Code handling external input must return `Result`, never `unwrap()` or `expect()` on data that could be malformed. A panic triggered by crafted input is a denial-of-service vulnerability. Use `unwrap()` only on invariants proven by prior validation in the same function.

### 7.3 DoS Prevention

**Impact: HIGH (unbounded resources enable denial of service)**

Enforce resource limits on all external-facing interfaces: rate limiting on request handlers, bounded queue sizes, maximum allocation sizes, and timeout values. An attacker must not be able to exhaust memory, CPU, or file descriptors through crafted input.

### 7.4 Error Propagation

**Impact: HIGH (silently ignored errors mask security-critical failures)**

Errors must be propagated, never silently ignored. A silently swallowed error in validation, authentication, or authorization code can make an entire security check a no-op. Use `?` for propagation or explicitly handle and log every error branch.

### 7.5 Checked Arithmetic for Overflow

**Impact: HIGH (integer overflow in unsafe contexts causes memory corruption)**

Use checked, saturating, or wrapping arithmetic where overflow is possible — especially in allocation size calculations, pointer offset computations, and array index derivations. In debug builds Rust panics on overflow, but release builds silently wrap, which can cause undersized allocations and buffer overflows.

**Incorrect (unchecked multiplication for allocation):**

```rust
let size = count * elem_size; // wraps silently in release
let ptr = alloc::alloc(Layout::from_size_align(size, align)?);
```

**Correct (checked arithmetic prevents undersized allocation):**

```rust
let size = count.checked_mul(elem_size).ok_or(AllocError::Overflow)?;
let ptr = alloc::alloc(Layout::from_size_align(size, align)?);
```

---

## 8. Severity Classification

**Impact: MEDIUM**

Definitions for classifying audit findings by severity and the corresponding merge-blocking actions.

### 8.1 Severity Classification

**Impact: MEDIUM (inconsistent classification delays critical fixes)**

Classify every audit finding using these severity levels:

| Severity | Definition | Action |
|----------|------------|--------|
| CRITICAL | UB in production code path, exploitable | BLOCK merge, immediate fix |
| HIGH | UB in edge case, potential crash | BLOCK merge, fix required |
| MEDIUM | Unsafe code without SAFETY comment, unclear invariants | May merge with follow-up issue |
| LOW | Style issues, missing docs, non-blocking | Create follow-up issue |

CRITICAL and HIGH findings must block merge. MEDIUM findings may proceed with a tracked follow-up. LOW findings are advisory.

---

## 9. Sanitizers

**Impact: HIGH**

Runtime and compile-time sanitizers for detecting memory errors, data races, and undefined behavior — AddressSanitizer, ThreadSanitizer, MemorySanitizer, and Miri with CI integration.

### 9.1 ThreadSanitizer and MemorySanitizer

**Impact: HIGH (TSan detects data races and MSan detects uninitialized memory reads at runtime)**

ThreadSanitizer (TSan) and MemorySanitizer (MSan) are runtime sanitizers that detect concurrency and initialization bugs respectively. Both require nightly Rust and `-Zbuild-std`.

**ThreadSanitizer:**

```bash
RUSTFLAGS="-Zsanitizer=thread" cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu
```

Detects data races — concurrent unsynchronized access where at least one is a write. Overhead: 5-15x slowdown, 5-10x memory increase.

**Important limitation:** TSan cannot verify `atomic::fence` correctness and gives false confidence for lock-free code. Use loom for verifying atomic ordering and fence placement. TSan is for detecting races on non-atomic data.

**MemorySanitizer:**

```bash
RUSTFLAGS="-Zsanitizer=memory" cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu
```

Detects reads of uninitialized memory. Overhead: ~3x slowdown. Tracks initialization status at the bit level.

**Incorrect (relying on TSan for lock-free correctness):**

```rust
// TSan says "no races" — but atomic ordering bugs are NOT detected
use std::sync::atomic::{AtomicBool, Ordering, fence};
static FLAG: AtomicBool = AtomicBool::new(false);
// TSan cannot verify that this fence is correctly placed or sufficient
fence(Ordering::SeqCst);
```

**Correct (TSan for data races, loom for atomics):**

```rust
// Use TSan to find races on non-atomic shared data
// Use loom to verify atomic ordering and fence correctness
#[cfg(loom)]
#[test]
fn test_atomic_ordering() {
    loom::model(|| {
        // loom exhaustively checks all interleavings
    });
}

// TSan catches this kind of bug:
// Two threads writing to the same Vec without synchronization
```

### 9.2 Miri for Compile-Time UB Detection

**Impact: HIGH (Miri detects undefined behavior including dangling pointers, Stacked Borrows violations, and uninitialized reads)**

Miri is an interpreter for Rust's MIR that detects undefined behavior at test time. It catches bugs that ASan and TSan miss, including Stacked Borrows violations and invalid enum discriminants.

**Setup:**

```bash
cargo +nightly miri test
```

**Miri detects:**
- Dangling pointer dereferences
- Invalid enum discriminants
- Uninitialized memory reads
- Stacked Borrows violations (aliasing rule breaches)
- Data races (in concurrent code)

**Key MIRIFLAGS:**

| Flag | Purpose |
|---|---|
| `-Zmiri-strict-provenance` | Enforce strict pointer provenance (no int-to-ptr casts) |
| `-Zmiri-symbolic-alignment-check` | Catch alignment issues that hardware would silently accept |
| `-Zmiri-tree-borrows` | Use Tree Borrows model instead of Stacked Borrows (experimental) |

Miri runs ~100x slower than native execution. Gate tests that do not exercise unsafe code with `#[cfg(not(miri))]` to keep CI time reasonable.

**Incorrect (no Miri gating — slow CI with no benefit):**

```rust
#[test]
fn test_pure_safe_logic() {
    // This test has no unsafe code — running under Miri wastes CI time
    assert_eq!(2 + 2, 4);
}
```

**Correct (gate non-unsafe tests, run Miri on unsafe tests):**

```rust
#[cfg(not(miri))]
#[test]
fn test_pure_safe_logic() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_unsafe_ptr_operations() {
    // This exercises unsafe code — Miri will catch UB here
    let mut val = 42u64;
    let ptr = &mut val as *mut u64;
    unsafe {
        ptr.write(100);
        assert_eq!(ptr.read(), 100);
    }
}
```

**GitHub Actions integration:**

```yaml
- name: Miri
  run: cargo +nightly miri test
  env:
    MIRIFLAGS: "-Zmiri-strict-provenance"
```

### 9.3 Sanitizer CI Integration

**Impact: HIGH (sanitizers must run in CI to catch memory and concurrency bugs before merge)**

Sanitizers should be integrated into CI as a matrix of checks. Miri and ASan are blocking (pre-merge). TSan is advisory (non-blocking) due to false positives in lock-free code.

**CI policy:**

| Sanitizer | Gate | Rationale |
|---|---|---|
| Miri | Blocking pre-merge | Catches UB with zero false positives |
| ASan | Blocking pre-merge | Catches memory errors with near-zero false positives |
| TSan | Non-blocking (advisory) | False positives on atomics; use loom for lock-free verification |

Run sanitizers only on crates with unsafe code to save CI time. Use a workspace filter or crate list.

**GitHub Actions workflow with matrix strategy:**

```yaml
name: Sanitizers
on: [pull_request]

jobs:
  sanitizers:
    runs-on: ubuntu-latest
    strategy:
      fail-fast: false
      matrix:
        sanitizer: [miri, asan, tsan]
        include:
          - sanitizer: miri
            command: cargo +nightly miri test
            env_flags: "-Zmiri-strict-provenance"
            blocking: true
          - sanitizer: asan
            command: >-
              cargo +nightly test -Zbuild-std
              --target x86_64-unknown-linux-gnu
            env_flags: "-Zsanitizer=address"
            blocking: true
          - sanitizer: tsan
            command: >-
              cargo +nightly test -Zbuild-std
              --target x86_64-unknown-linux-gnu
            env_flags: "-Zsanitizer=thread"
            blocking: false
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@nightly
        with:
          components: miri, rust-src
      - name: Run ${{ matrix.sanitizer }}
        run: ${{ matrix.command }}
        env:
          RUSTFLAGS: ${{ matrix.env_flags }}
          MIRIFLAGS: ${{ matrix.sanitizer == 'miri' && matrix.env_flags || '' }}
        continue-on-error: ${{ !matrix.blocking }}
```

**Incorrect (no sanitizer CI — bugs reach production):**

```yaml
# Only running cargo test — no sanitizer coverage
- run: cargo test
```

**Correct (sanitizer matrix with blocking policy):**

```yaml
# Miri and ASan block merge, TSan is advisory
# See full workflow above
- name: Run ${{ matrix.sanitizer }}
  run: ${{ matrix.command }}
  continue-on-error: ${{ !matrix.blocking }}
```

---

## 10. Fuzzing

**Impact: HIGH**

Coverage-guided fuzz testing with cargo-fuzz and libFuzzer — target selection, structured fuzzing, corpus management, and ASan integration.

### 10.1 Fuzz Target Selection

**Impact: HIGH (fuzzing the wrong targets wastes CI time while leaving attack surface untested)**

Fuzz testing is most effective on code that processes untrusted or complex input. Prioritize targets by attack surface and complexity.

**Fuzz these (high value):**
- Parsers (file formats, network protocols, configuration)
- Deserializers (serde implementations, custom binary formats)
- FFI boundaries (data crossing language boundaries)
- Unsafe code (pointer arithmetic, slice construction from raw parts)
- Protocol handlers (message framing, state machines)
- Codec implementations (compression, encryption, encoding)

**Do not fuzz (low value):**
- Pure business logic with bounded, well-typed inputs
- UI code and rendering
- Trivially correct code (simple getters, field access)
- Code with no unsafe and no external input

**Prioritization rule:** Anything processing untrusted input is a fuzz target. If an attacker controls the bytes, fuzz it.

**Coverage-guided fuzzing:** libFuzzer tracks code coverage to explore new execution paths. It mutates inputs that trigger new branches, progressively reaching deeper code paths. This makes it far more effective than random input generation.

**Seed corpus for faster results:**

```bash
# Add known edge cases as seed inputs
mkdir -p corpus/my_target
echo -n "" > corpus/my_target/empty
echo -n "valid_input" > corpus/my_target/basic
cp test_fixtures/edge_case.bin corpus/my_target/
```

Seed corpus inputs give the fuzzer a starting point with known coverage, dramatically reducing time to find new paths compared to starting from empty input.

**Incorrect (fuzzing trivial safe code):**

```rust
// Wasted effort — this code cannot crash or have memory errors
fuzz_target!(|data: &[u8]| {
    let x: u32 = data.len() as u32;
    let _ = x.saturating_add(1);
});
```

**Correct (fuzzing a parser that processes untrusted input):**

```rust
fuzz_target!(|data: &[u8]| {
    // Parser processes untrusted network input — high fuzz value
    let _ = my_crate::protocol::parse_message(data);
});
```

---

## 11. Supply Chain

**Impact: MEDIUM**

Supply chain security for Rust dependencies — advisory database checks, peer review of third-party code, and dependency hygiene practices.

### 11.1 Dependency Hygiene

**Impact: MEDIUM (unnecessary dependencies increase attack surface and build times)**

Fewer dependencies mean a smaller attack surface, faster builds, and less maintenance burden. Every dependency is code you did not write and must trust.

**Minimal dependency principle:** Every dependency should be justified. If a crate provides a single utility function you could write in 20 lines, do not depend on it.

**Tools for dependency hygiene:**

| Tool | Purpose | Command |
|---|---|---|
| `cargo-machete` | Find unused dependencies | `cargo machete` |
| `cargo tree -d` | Find duplicate dependency versions | `cargo tree -d` |
| `cargo fetch --locked` | Verify `Cargo.lock` matches `Cargo.toml` | `cargo fetch --locked` |
| `cargo deny check` | Unified policy (advisories, licenses, duplicates) | `cargo deny check` |

**CI practices:**
- Pin dependency versions in CI with `--locked` to prevent silent updates
- Run `cargo machete` to detect and remove unused deps
- Run `cargo tree -d` to identify duplicate versions and unify them
- Require justification for new dependencies in PR descriptions

**Incorrect (unnecessary dependency, no lockfile verification):**

```toml
[dependencies]
# left-pad equivalent — one function, pulled in an entire crate
is-even = "1.0"
```

```yaml
# CI does not verify lockfile — deps can silently change
- run: cargo build
```

**Correct (minimal deps, lockfile pinned, unused deps detected):**

```toml
[dependencies]
# Only deps that provide substantial value and are well-maintained
serde = { version = "1", features = ["derive"] }
```

```yaml
# CI verifies lockfile integrity and checks for unused deps
- run: cargo fetch --locked
- run: cargo machete --with-metadata
- run: cargo build --locked
```
