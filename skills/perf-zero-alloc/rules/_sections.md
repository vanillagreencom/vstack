# Sections

This file defines all sections, their ordering, impact levels, and descriptions.
The section ID (in parentheses) is the filename prefix used to group rules.

---

## 1. Allocation Elimination (alloc)

**Impact:** CRITICAL
**Description:** Core patterns for eliminating heap allocations in hot paths. Violations cause latency spikes from allocator contention and unpredictable GC-like pauses.

## 2. Data Structures (ds)

**Impact:** HIGH
**Description:** Choosing and configuring bounded collections, queues, and arenas for predictable memory behavior. Wrong choices cause cache misses, false sharing, or silent capacity overflows.

## 3. Verification (verify)

**Impact:** HIGH
**Description:** Techniques for detecting, asserting, and profiling allocations to enforce zero-alloc invariants. Without verification, hidden allocations go undetected until production.

## 4. Pitfalls (pit)

**Impact:** MEDIUM
**Description:** Common Rust idioms that silently allocate. Awareness prevents accidental allocations in code that appears allocation-free.
