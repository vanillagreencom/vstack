---
name: rust-arch
description: Architecture anti-patterns, review scoring rubrics, error handling, and layered design patterns. Use when evaluating component designs, detecting architectural drift, reviewing PR architecture, or assessing technical debt.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Architecture Patterns

Anti-patterns, scoring rubrics, error handling, and layered design patterns for architecture reviews, prioritized by impact.

## When to Apply

Reference these guidelines when:
- Reviewing new component designs or refactoring proposals
- Evaluating PR architecture for anti-pattern violations
- Assessing technical debt severity and classification
- Checking hot-path code for performance anti-patterns
- Verifying lock-free correctness in concurrent code
- Establishing error handling strategy for a module

## Rule Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Architectural Anti-Patterns | CRITICAL | `arch-` |
| 2 | Performance Anti-Patterns | CRITICAL | `perf-` |
| 3 | Lock-Free Anti-Patterns | HIGH | `lock-` |
| 4 | Error Handling & Data Integrity | HIGH | `err-` |
| 5 | Review Process | MEDIUM | `review-` |
| 6 | UI Anti-Patterns | MEDIUM | `ui-` |

## Quick Reference

### 1. Architectural Anti-Patterns (CRITICAL)

- `arch-god-object` - Struct with 6+ responsibilities; split into focused components
- `arch-giant-file` - >1,500 lines impl; split by responsibility
- `arch-tight-coupling` - Concrete types instead of traits; use dependency injection
- `arch-circular-deps` - Modules depend on each other; enforce layered architecture
- `arch-leaky-abstraction` - Internal details cross boundaries; add facade/interface layer
- `arch-shotgun-surgery` - One change requires many file edits; consolidate related logic
- `arch-feature-envy` - Code uses another module's data heavily; move to data owner
- `arch-layered-architecture` - Dependencies flow DOWN only; modules communicate via interfaces

### 2. Performance Anti-Patterns (CRITICAL)

- `perf-mutex-in-hot-path` - Mutex/RwLock adds +10-100us per contention; use lock-free
- `perf-heap-allocation` - Vec/Box/String adds +50-500ns per alloc; pre-allocate at startup
- `perf-dynamic-dispatch` - Box<dyn>/&dyn adds +5-20ns per call; use generics
- `perf-string-formatting` - format!/to_string adds +100-500ns; use pre-allocated buffers
- `perf-hashmap-lookup` - HashMap::get adds +20-50ns; use array index or perfect hash
- `perf-system-calls` - File I/O, time calls add +100ns-10us; batch and cache

### 3. Lock-Free Anti-Patterns (HIGH)

- `lock-wrong-atomic-ordering` - Relaxed everywhere causes data races; use Acquire/Release
- `lock-missing-fence` - Stale reads from reordering; add fence, verify with loom
- `lock-tsan-for-fences` - TSAN doesn't understand fences; use loom instead
- `lock-escaped-guard` - crossbeam Guard reference escapes scope; process within guard
- `lock-aba-problem` - CAS without generation counter; use tagged pointers

### 4. Error Handling & Data Integrity (HIGH)

- `err-fail-fast` - Fail loudly over silent degradation; exceptions for observability tools
- `err-immutable-after-reception` - Data frozen after normalization; Copy types, new instances
- `err-investigate-before-dismiss` - Trace to source, understand intent, verify impact

### 5. Review Process (MEDIUM)

- `review-scoring-rubric` - Five dimensions, Design Efficiency weighted 2x, pass >=80 overall
- `review-quality-gates` - Layered architecture, no circular deps, no hot-path anti-patterns
- `review-tech-debt` - P1-P4 classification with tracking format

### 6. UI Anti-Patterns (MEDIUM)

- `ui-per-item-update` - Loop with individual updates causes frame drops; batch
- `ui-thread-blocking` - Sync I/O on main thread freezes UI; use async/background
- `ui-unbounded-collection` - Collection growing without limit; use bounded buffer

## How to Use

Read individual rule files for detailed explanations and code examples:

```
rules/arch-layered-architecture.md
rules/perf-mutex-in-hot-path.md
rules/err-fail-fast.md
```

Each rule file contains:
- Brief explanation of why it matters
- Indicators for detection
- Fix recommendations with code examples where applicable

## Resources

Documentation lookup order: local skill files → ctx7 CLI → web fallback.

### ctx7 CLI

| Library | ctx7 ID | Use For |
|---------|---------|---------|
| Rust std | `/websites/doc_rust-lang_stable_std` | Standard library types, traits, atomics |
| crossbeam | `/crossbeam-rs/crossbeam` | Epoch-based reclamation, lock-free structures |
| tokio | `/websites/rs_tokio` | Async runtime, channels, synchronization |

## Full Compiled Document

For the complete guide with all rules expanded, plus the architecture review workflow: `AGENTS.md`
