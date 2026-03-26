---
name: perf-zero-alloc
description: Zero-allocation patterns for Rust hot paths. Use when optimizing hot paths, eliminating allocations, implementing object pools, selecting lock-free queues, or verifying allocation-free execution.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Zero-Allocation Rust Patterns

Patterns for eliminating heap allocations in performance-critical Rust hot paths, achieving sub-500us latency.

## When to Apply

Reference these guidelines when:
- Implementing or optimizing hot-path code that must not allocate
- Choosing between object pools, arenas, bounded collections, or preallocated buffers
- Selecting SPSC, MPSC, or MPMC queue implementations
- Replacing dynamic dispatch or boxed closures with static alternatives
- Adding allocation verification tests or profiling allocations
- Reviewing code for hidden allocations (format!, collect, to_string)

## Core Principle

**Never allocate after startup in hot paths.** Pre-allocate all memory during initialization.

## Dev Tools

| Tool | Purpose | Install |
|------|---------|---------|
| `heaptrack` | Full-program heap profiling with GUI | System package manager |
| `dhat` | Rust-native allocation attribution | `cargo add --dev dhat` |
| `assert_no_alloc` | CI-gate allocation assertions | `cargo add --dev assert_no_alloc` |

## Rule Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Allocation Elimination | CRITICAL | `alloc-` |
| 2 | Data Structures | HIGH | `ds-` |
| 3 | Verification | HIGH | `verify-` |
| 4 | Pitfalls | MEDIUM | `pit-` |

## Quick Reference

### 1. Allocation Elimination (CRITICAL)

- `alloc-object-pools` - Pre-allocate reusable objects via slab, free-list, or SPSC pool
- `alloc-preallocated-buffers` - Allocate buffers at startup, reuse via .clear()
- `alloc-static-dispatch` - Use generics over dyn/Box for zero-overhead dispatch in hot paths
- `alloc-string-interning` - Intern strings once, compare as integers thereafter

### 2. Data Structures (HIGH)

- `ds-bounded-collections` - Use ArrayVec for stack-allocated fixed-capacity collections
- `ds-arena-allocators` - Use bumpalo for temporary allocations freed in bulk
- `ds-queue-selection` - SPSC/MPSC/MPMC queue crate selection with performance baselines
- `ds-cache-line-padding` - 128-byte padding for cross-thread atomics; power-of-two capacity

### 3. Verification (HIGH)

- `verify-assert-no-alloc` - CI-gate hot paths with assert_no_alloc global allocator
- `verify-profiling` - Profile with dhat/heaptrack to attribute allocation sources

### 4. Pitfalls (MEDIUM)

- `pit-hidden-format` - format! silently allocates a String on every call
- `pit-iterator-collect` - .collect() allocates a new collection on every call
- `pit-recursive-box` - Box in recursive structures allocates per node; use arenas
- `pit-string-operations` - to_uppercase/to_string create new String allocations
- `pit-vec-push-growth` - Vec::push may reallocate when capacity is exhausted

## How to Use

Read individual rule files for detailed explanations and code examples:

```
rules/alloc-object-pools.md
rules/ds-queue-selection.md
rules/verify-assert-no-alloc.md
```

Each rule file contains:
- Brief explanation of why it matters
- Code examples (incorrect vs. correct where applicable)

## Resources

Documentation lookup order: local skill files -> ctx7 CLI -> web fallback.

### ctx7 CLI

| Library | ctx7 ID | Use For |
|---------|---------|---------|
| Rust std | `/websites/doc_rust-lang_stable_std` | mem, alloc, ptr, collections |
| serde | `/websites/serde_rs` | Zero-copy deserialization |
| windows-rs | `/microsoft/windows-rs` | Win32 API, windows-sys bindings |

### Web

| Source | URL | Use For |
|--------|-----|---------|
| rtrb | `https://docs.rs/rtrb/latest/rtrb/` | Real-time SPSC ring buffer |
| slab | `https://docs.rs/slab/latest/slab/` | Pre-allocated arena storage |
| ringbuf | `https://docs.rs/ringbuf` | Production SPSC ring buffer |
| crossbeam | `https://docs.rs/crossbeam` | Concurrent data structures |
| bumpalo | `https://docs.rs/bumpalo` | Arena allocator |
| The Rust Performance Book | `https://nnethercote.github.io/perf-book/` | General Rust perf guidance |
| LMAX Disruptor | `https://lmax-exchange.github.io/disruptor/` | Original disruptor pattern |

## Full Compiled Document

For the complete guide with all rules expanded, queue selection tables, and full code examples: `AGENTS.md`
