---
name: perf-lock-free
description: Lock-free correctness verification with loom, atomic patterns, and crossbeam-epoch safety. Use when implementing SPSC queues, verifying atomics, debugging concurrency issues, or choosing safety verification tools.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Lock-Free Safety Patterns

Verification patterns and correctness rules for lock-free data structures, atomic orderings, and epoch-based memory reclamation.

## When to Apply

Reference these guidelines when:
- Implementing or modifying SPSC queues, ring buffers, or lock-free data structures
- Writing or reviewing code with `unsafe` blocks involving atomics or raw pointers
- Choosing between MIRI, loom, TSAN, and ASAN for verification
- Working with crossbeam-epoch for memory reclamation
- Debugging concurrency issues on ARM64 or weakly-ordered architectures
- Auditing atomic ordering choices (Relaxed, Acquire, Release, SeqCst)

## Nomenclature

- **SPSC** - Single-Producer, Single-Consumer queue
- **UB** - Undefined Behavior
- **MIRI** - Mid-level IR Interpreter (detects UB in unsafe Rust)
- **Loom** - Concurrency permutation testing framework
- **TSAN** - ThreadSanitizer (detects data races in mutex-based code)
- **ASAN** - AddressSanitizer (detects memory errors)
- **Epoch reclamation** - Deferred memory deallocation via crossbeam-epoch

## Rule Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Soundness | CRITICAL | `sound-` |
| 2 | Verification | CRITICAL | `verify-` |
| 3 | Ordering | HIGH | `ord-` |
| 4 | Epoch Reclamation | HIGH | `epoch-` |
| 5 | Testing | MEDIUM | `test-` |

## Quick Reference

### 1. Soundness (CRITICAL)

- `sound-unsafecell-spsc` - Use UnsafeCell<MaybeUninit<T>> for SPSC buffers; casting &T to *mut T is UB
- `sound-no-seqcst-default` - Never default to SeqCst; use minimal sufficient ordering with comment justification

### 2. Verification (CRITICAL)

- `verify-tsan-no-fences` - TSAN cannot verify atomic fences; use loom for lock-free code
- `verify-tool-selection` - Use correct verification tool per code category (MIRI/loom/TSAN/ASAN)
- `verify-loom-for-lockfree` - Every lock-free structure must have loom tests

### 3. Ordering (HIGH)

- `ord-acquire-release-spsc` - Acquire/Release pattern for SPSC: Relaxed own-index, Acquire other-thread, Release publish
- `ord-arm64-testing` - Test on ARM64 to catch ordering bugs hidden by x86 strong memory model
- `ord-fence-batching` - Fence batching: Relaxed stores + single fence(Release) + Relaxed sentinel instead of N Release stores

### 4. Epoch Reclamation (HIGH)

- `epoch-pin-before-load` - Always pin epoch before atomic load; references must not escape guard lifetime
- `epoch-defer-destroy` - Use defer_destroy for deallocation; never mix manual drop with epoch reclamation

### 5. Testing (MEDIUM)

- `test-loom-model-design` - Keep loom models small, one property per model, yield in spin loops
- `test-miri-unsafe-only` - Scope MIRI to unsafe code paths; gate safe-only tests with #[cfg(not(miri))]

## Quality Gates

- [ ] Uses `UnsafeCell<MaybeUninit<T>>` for SPSC buffers (not raw pointer casts)
- [ ] Loom tests pass (`LOOM_MAX_PREEMPTIONS=2` for CI)
- [ ] Tests pass on ARM64 — catches weak-memory bugs
- [ ] Crossbeam epoch Guard lifetimes verified
- [ ] No `atomic::fence` without loom coverage
- [ ] Ordering justification in comments (why not SeqCst?)
- [ ] MIRI passes on unsafe code paths

## How to Use

Read individual rule files for detailed explanations and code examples:

```
rules/sound-unsafecell-spsc.md
rules/verify-tool-selection.md
rules/ord-acquire-release-spsc.md
```

Each rule file contains:
- Brief explanation of why it matters
- Incorrect code example with explanation (where applicable)
- Correct code example with explanation

## Resources

Documentation lookup order: local skill files -> ctx7 CLI -> web fallback.

### ctx7 CLI

| Library | ctx7 ID | Use For |
|---------|---------|---------|
| Rust std | `/websites/doc_rust-lang_stable_std` | unsafe semantics, atomics, sync primitives |
| crossbeam-channel | `/websites/rs_crossbeam-channel` | MPMC channels, select |
| parking_lot | `/websites/rs_parking_lot` | Mutex/RwLock primitives |
| dashmap | `/websites/rs_dashmap` | Concurrent hashmap |

### Web

| Library | URL | Use For |
|---------|-----|---------|
| crossbeam-epoch | `https://docs.rs/crossbeam-epoch/latest/crossbeam_epoch/` | Lock-free memory reclamation |
| crossbeam-utils | `https://docs.rs/crossbeam-utils/latest/crossbeam_utils/` | CachePadded, Backoff, scoped threads |
| loom | `https://docs.rs/loom/latest/loom/` | Concurrency permutation testing |
| Rust Atomics and Locks | `https://marabos.nl/atomics/` | Mara Bos book — authoritative reference |

## Full Compiled Document

For the complete guide with all rules expanded inline: `AGENTS.md`
