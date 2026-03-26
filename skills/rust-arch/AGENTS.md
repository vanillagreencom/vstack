# Architecture Patterns

**Version 1.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when reviewing
> architecture, detecting anti-patterns, or evaluating technical debt.
> Humans may also find it useful, but guidance here is optimized for
> automation and consistency by AI-assisted workflows.

---

## Abstract

Anti-patterns, scoring rubrics, error handling, and layered design patterns for architecture reviews, prioritized by impact from critical (structural and performance violations) through high (concurrency and error handling) to medium (review process and UI patterns). Each rule includes detection indicators and fix recommendations.

---

## Table of Contents

1. [Architectural Anti-Patterns](#1-architectural-anti-patterns) — **CRITICAL**
   - 1.1 [God Object](#11-god-object)
   - 1.2 [Giant File](#12-giant-file)
   - 1.3 [Tight Coupling](#13-tight-coupling)
   - 1.4 [Circular Dependencies](#14-circular-dependencies)
   - 1.5 [Leaky Abstraction](#15-leaky-abstraction)
   - 1.6 [Shotgun Surgery](#16-shotgun-surgery)
   - 1.7 [Feature Envy](#17-feature-envy)
   - 1.8 [Layered Architecture](#18-layered-architecture)
2. [Performance Anti-Patterns](#2-performance-anti-patterns) — **CRITICAL**
   - 2.1 [Mutex in Hot Path](#21-mutex-in-hot-path)
   - 2.2 [Heap Allocation in Hot Path](#22-heap-allocation-in-hot-path)
   - 2.3 [Dynamic Dispatch in Hot Path](#23-dynamic-dispatch-in-hot-path)
   - 2.4 [String Formatting in Hot Path](#24-string-formatting-in-hot-path)
   - 2.5 [HashMap Lookup in Hot Path](#25-hashmap-lookup-in-hot-path)
   - 2.6 [System Calls in Hot Path](#26-system-calls-in-hot-path)
3. [Lock-Free Anti-Patterns](#3-lock-free-anti-patterns) — **HIGH**
   - 3.1 [Wrong Atomic Ordering](#31-wrong-atomic-ordering)
   - 3.2 [Missing Fence](#32-missing-fence)
   - 3.3 [TSAN for Fence-Based Code](#33-tsan-for-fence-based-code)
   - 3.4 [Escaped Guard Lifetime](#34-escaped-guard-lifetime)
   - 3.5 [ABA Problem](#35-aba-problem)
4. [Error Handling & Data Integrity](#4-error-handling--data-integrity) — **HIGH**
   - 4.1 [Fail Fast Over Silent Degradation](#41-fail-fast-over-silent-degradation)
   - 4.2 [Data Immutability After Reception](#42-data-immutability-after-reception)
   - 4.3 [Investigate Errors Before Dismissal](#43-investigate-errors-before-dismissal)
5. [Review Process](#5-review-process) — **MEDIUM**
   - 5.1 [Review Scoring Rubric](#51-review-scoring-rubric)
   - 5.2 [Quality Gates](#52-quality-gates)
   - 5.3 [Technical Debt Classification](#53-technical-debt-classification)
6. [UI Anti-Patterns](#6-ui-anti-patterns) — **MEDIUM**
   - 6.1 [Per-Item UI Update](#61-per-item-ui-update)
   - 6.2 [UI Thread Blocking](#62-ui-thread-blocking)
   - 6.3 [Unbounded Collection](#63-unbounded-collection)
7. [Architecture Review Workflow](#7-architecture-review-workflow)

---

## 1. Architectural Anti-Patterns

**Impact: CRITICAL**

Structural violations that degrade maintainability, testability, and scalability. Detect during design reviews and reject in PRs.

### 1.1 God Object

**Impact: CRITICAL (hard to test, modify, or reason about)**

A struct with 6+ distinct responsibilities violates single-responsibility principle. It becomes a change magnet — every feature touches it, making parallel development and testing difficult.

**Indicator:** Struct with methods spanning unrelated domains (parsing, routing, rendering, persistence).

**Fix:** Split into focused components, each owning one responsibility. Connect via trait interfaces.

### 1.2 Giant File

**Impact: CRITICAL (blocks tooling and navigation)**

Files exceeding ~1,500 lines of implementation (or ~2,000 with tests) become difficult to navigate and can exceed tool context limits. They also tend to accumulate unrelated responsibilities.

**Indicator:** File line count growing beyond limits; multiple unrelated impl blocks.

**Fix:** Split by responsibility. Each module gets one clear responsibility and a minimal public API.

### 1.3 Tight Coupling

**Impact: CRITICAL (can't swap implementations or test in isolation)**

Using concrete types instead of trait interfaces between modules prevents dependency injection, makes unit testing require full dependency chains, and blocks implementation swaps.

**Incorrect (concrete dependency):**

```rust
struct OrderRouter {
    exchange: BinanceClient,  // Locked to one implementation
}
```

**Correct (trait-based dependency injection):**

```rust
struct OrderRouter<E: Exchange> {
    exchange: E,  // Any implementation works
}
```

### 1.4 Circular Dependencies

**Impact: CRITICAL (build issues and tangled logic)**

Modules that depend on each other create cycles that prevent clean layering, cause build ordering issues, and make it impossible to reason about data flow in isolation.

**Indicator:** Module A imports from B, module B imports from A.

**Fix:** Enforce layered architecture — dependencies flow DOWN only. Extract shared types into a common lower-layer module that both depend on.

### 1.5 Leaky Abstraction

**Impact: CRITICAL (breaks encapsulation and couples consumers to internals)**

When internal implementation details cross module boundaries, consumers become coupled to internals. Any refactoring of the implementation then breaks all consumers.

**Indicator:** Public APIs expose internal types, implementation-specific error variants, or require callers to understand internal data layout.

**Fix:** Add a facade or interface layer. Expose only the contract (traits, public types) — never the mechanism.

### 1.6 Shotgun Surgery

**Impact: CRITICAL (high change cost across many files)**

When a single logical change requires edits to many unrelated files, the related logic is scattered. This makes changes error-prone (easy to miss a file) and expensive to review.

**Indicator:** One feature change touches 5+ files across different modules.

**Fix:** Consolidate related logic. Group types and functions that change together into the same module.

### 1.7 Feature Envy

**Impact: CRITICAL (wrong responsibility placement)**

When code in one module heavily accesses another module's data — calling multiple getters, destructuring its types, or computing derived values from its fields — the logic belongs in the data owner, not the consumer.

**Indicator:** A function that takes a struct from another module and accesses 3+ of its fields.

**Fix:** Move the computation to the module that owns the data. Expose a method instead of exposing fields.

### 1.8 Layered Architecture

**Impact: CRITICAL (foundational constraint for dependency management)**

Applications should be organized in layers where dependencies flow DOWN only. Higher layers (UI, application logic) depend on lower layers (core engine, infrastructure). Lower layers never import from higher layers.

```
┌─────────────────────────────────────────┐
│           UI Layer (Presentation)       │
├─────────────────────────────────────────┤
│          Core Engine (Business Logic)   │
│    ┌─────────┬─────────┬─────────┐      │
│    │ Domain  │ Domain  │ Domain  │      │
│    │  Area A │  Area B │  Area C │      │
│    └─────────┴─────────┴─────────┘      │
├─────────────────────────────────────────┤
│        Infrastructure (Storage, IPC)    │
└─────────────────────────────────────────┘
```

**Rule:** Modules communicate via defined interfaces, never internal types. Each module owns its data and exposes only its public contract.

---

## 2. Performance Anti-Patterns

**Impact: CRITICAL**

Patterns that violate hot-path performance constraints. Must be rejected in latency-sensitive execution paths.

### 2.1 Mutex in Hot Path

**Impact: CRITICAL (+10-100us per contention event)**

`Mutex<T>` or `RwLock<T>` in latency-sensitive paths (order processing, tick handling) adds contention-dependent latency that violates sub-microsecond budgets.

**Detection:** `Mutex<T>` or `RwLock<T>` used in data flow paths.

**Fix:** Lock-free alternatives — SPSC ring buffers, atomics, `ArcSwap` for read-heavy shared state.

### 2.2 Heap Allocation in Hot Path

**Impact: CRITICAL (+50-500ns per allocation)**

`Vec::new()`, `Box::new()`, `String::from()` in hot paths cause allocator contention and unpredictable latency spikes from system allocator calls.

**Detection:** `Vec::new()`, `Box::new()`, `String::new()`, `format!()` in latency-sensitive code paths.

**Fix:** Pre-allocate at startup. Use object pools, bounded ring buffers, or stack-allocated arrays. All collections should be created with known capacity during initialization.

### 2.3 Dynamic Dispatch in Hot Path

**Impact: CRITICAL (+5-20ns per call from vtable indirection)**

`Box<dyn Trait>` and `&dyn Trait` in hot paths prevent inlining and add vtable lookup overhead on every call. In tight loops processing millions of events, this compounds.

**Detection:** `Box<dyn ...>` or `&dyn ...` in data processing paths.

**Fix:** Use generics with static dispatch. Monomorphization eliminates vtable overhead and enables inlining.

### 2.4 String Formatting in Hot Path

**Impact: CRITICAL (+100-500ns per format call)**

`format!()`, `to_string()`, and string interpolation allocate heap memory and invoke the formatting machinery on every call.

**Detection:** `format!()`, `.to_string()`, string concatenation in latency-sensitive paths.

**Fix:** Pre-allocated buffers written at startup, or `write!()` into a reusable buffer. For logging, use compile-time disabled macros or sampling in hot paths.

### 2.5 HashMap Lookup in Hot Path

**Impact: CRITICAL (+20-50ns per lookup)**

`HashMap::get()` involves hashing, bucket traversal, and potential cache misses. In tight loops this adds measurable latency.

**Detection:** `HashMap` access patterns in data processing paths.

**Fix:** Array index with known-range keys, perfect hashing for static key sets, or pre-computed lookup tables populated at startup.

### 2.6 System Calls in Hot Path

**Impact: CRITICAL (+100ns-10us per call)**

File I/O, time syscalls (`SystemTime::now()`), and other kernel transitions add unpredictable latency from context switches and kernel scheduling.

**Detection:** File operations, system time calls, network I/O in latency-sensitive paths.

**Fix:** Batch operations, cache results (e.g., read time once per batch), and move I/O to background threads.

---

## 3. Lock-Free Anti-Patterns

**Impact: HIGH**

Concurrency mistakes in lock-free code — wrong ordering, missing fences, lifetime escapes. Cause data races and corruption.

### 3.1 Wrong Atomic Ordering

**Impact: HIGH (data races that only manifest under load)**

Using `Relaxed` ordering everywhere is a common shortcut that causes data races on non-x86 architectures and can produce stale reads even on x86 under contention.

**Detection:** All atomics using `Ordering::Relaxed` without analysis of happens-before requirements.

**Fix:** Use proper Acquire/Release pairs. Producer stores with `Release`, consumer loads with `Acquire`. Use `SeqCst` only when total ordering across multiple atomics is required.

### 3.2 Missing Fence

**Impact: HIGH (stale reads causing data corruption)**

When a memory fence is needed (e.g., between non-atomic writes and an atomic flag), omitting it allows the CPU to reorder operations, causing consumers to read partially-updated data.

**Detection:** Atomic flag patterns without corresponding `fence()` calls where non-atomic data must be visible.

**Fix:** Add appropriate fence. Verify correctness with loom (not TSAN — TSAN does not understand fence-based synchronization).

### 3.3 TSAN for Fence-Based Code

**Impact: HIGH (false confidence in correctness)**

Thread Sanitizer (TSAN) does not understand `std::sync::atomic::fence()` synchronization patterns. It will report false negatives (no warnings) for code that has real data races, and false positives for correct fence usage.

**Detection:** Using TSAN to validate code that relies on explicit fences for synchronization.

**Fix:** Use loom for verification of fence-based synchronization. Loom models the memory ordering rules correctly and explores possible interleavings.

### 3.4 Escaped Guard Lifetime

**Impact: HIGH (use-after-free in concurrent code)**

Crossbeam epoch-based reclamation guards protect memory from deallocation while the guard is alive. If a reference obtained under a guard escapes the guard's scope, the memory can be reclaimed while still referenced.

**Detection:** References derived from crossbeam `Guard`-protected loads that outlive the guard scope.

**Fix:** Process all data within the guard scope. Clone or copy needed values before dropping the guard.

### 3.5 ABA Problem

**Impact: HIGH (silent corruption in CAS loops)**

Compare-and-swap (CAS) succeeds if the current value matches the expected value. If a value changes from A to B and back to A between the read and CAS, the CAS succeeds despite the intermediate mutation — potentially corrupting data structures.

**Detection:** CAS loops without generation counters or hazard pointers.

**Fix:** Use tagged pointers (pack a generation counter into the pointer) or hazard pointer schemes that detect intermediate modifications.

---

## 4. Error Handling & Data Integrity

**Impact: HIGH**

Error handling strategy and data immutability rules. Violations cause silent data corruption or missed failures.

### 4.1 Fail Fast Over Silent Degradation

**Impact: HIGH (silent failures hide bugs until production)**

Critical execution paths must fail loudly rather than silently degrade. Skipping invalid data, queuing indefinitely on missing connections, or approximating on validation failure all hide problems until they compound into production incidents.

- Invalid data: panic or return error immediately; never skip or substitute
- Missing connection: error immediately; never queue indefinitely
- Validation failure: halt processing; never approximate

**Exception:** Observability tools (profilers, metrics, logging) should degrade gracefully with warnings rather than crash the application.

### 4.2 Data Immutability After Reception

**Impact: HIGH (mutation causes data corruption and replay inconsistency)**

Market data and other time-series inputs must be frozen after normalization. Mutating received data breaks audit trails, makes replays non-deterministic, and risks one consumer's transformation affecting another's view.

- Use `Copy` types for small value types (ticks, quotes, bars)
- Transformations create new instances; never mutate originals
- Freeze data after the normalization/parsing step

### 4.3 Investigate Errors Before Dismissal

**Impact: HIGH (silently broken functionality goes unnoticed)**

Never dismiss errors, warnings, or unexpected behavior without investigation. Errors that "seem harmless" often indicate silently broken functionality discovered too late.

Investigation checklist:
1. **Trace to source** — where is this coming from?
2. **Understand intent** — what was supposed to happen?
3. **Verify impact** — is functionality silently broken?
4. Only dismiss after confirming harmless (document why in a comment)

---

## 5. Review Process

**Impact: MEDIUM**

Architecture review scoring, quality gates, and technical debt classification. Provides consistent evaluation framework.

### 5.1 Review Scoring Rubric

**Impact: MEDIUM (inconsistent review standards without a framework)**

Architecture reviews score across five dimensions:

| Dimension | Weight | Pass | Focus |
|-----------|--------|------|-------|
| **Design Efficiency** | 2x | >=90 | No anti-patterns in hot path |
| Modularity | 1x | >=80 | Clean boundaries, single responsibility |
| Maintainability | 1x | >=80 | Easy to modify, understand |
| Testability | 1x | >=80 | Easy to unit test, mock |
| Scalability | 1x | >=80 | Can handle growth |

**Formula:** (Design Efficiency x 2 + Modularity + Maintainability + Testability + Scalability) / 6

**Pass criteria:** Overall >=80 AND Design Efficiency >=90.

#### Scoring Guide

**Design Efficiency (0-100):**
- 100: Zero anti-patterns, optimal data flow
- 90: Minor inefficiencies, no hot-path issues
- 70: Some anti-patterns, not in critical path
- 50: Anti-patterns affect performance
- <50: Critical anti-patterns in hot path

**Modularity (0-100):**
- 100: Perfect separation, clear interfaces
- 80: Good boundaries, minor coupling
- 60: Some tight coupling
- <60: Circular dependencies or god objects

### 5.2 Quality Gates

**Impact: MEDIUM (architectural violations slip through without gates)**

Every architecture review must pass these gates before approval:

- [ ] Follows layered architecture (dependencies flow down)
- [ ] No circular dependencies
- [ ] Abstractions at module boundaries
- [ ] No hot-path anti-patterns
- [ ] Platform differences handled explicitly
- [ ] Pre-allocation strategy documented
- [ ] Error handling strategy clear

**Reject if any gate fails.**

### 5.3 Technical Debt Classification

**Impact: MEDIUM (unclassified debt gets lost or misprioritized)**

Classify discovered technical debt by impact and urgency:

| Priority | Impact | Timeline | Example |
|----------|--------|----------|---------|
| P1 Urgent | Blocks performance budget | Fix immediately | Mutex in tick processing |
| P2 High | Architectural violation | Fix this cycle | Circular dependency |
| P3 Normal | Code smell, tech debt | Plan for backlog | Missing abstraction |
| P4 Low | Minor improvement | Track only | Naming, docs |

#### Tracking Format

```
TD-XXX: [Short description]
- Location: module/file description
- Impact: [Performance|Maintainability|Safety]
- Priority: P1|P2|P3|P4
- Estimate: 1-5 points
- Notes: [Additional context]
```

---

## 6. UI Anti-Patterns

**Impact: MEDIUM**

UI-layer patterns that cause frame drops, frozen interfaces, or memory growth.

### 6.1 Per-Item UI Update

**Impact: MEDIUM (frame drops from excessive redraws)**

Updating UI elements individually in a loop triggers a redraw per item, causing frame drops when processing collections.

**Detection:** Loop with individual widget updates or state invalidations.

**Fix:** Batch updates and trigger a single invalidation after the batch completes.

### 6.2 UI Thread Blocking

**Impact: MEDIUM (frozen interface during I/O)**

Synchronous I/O (file reads, network calls, database queries) on the main/UI thread blocks the event loop, freezing the interface for the duration of the operation.

**Detection:** Sync I/O calls in message handlers or view functions.

**Fix:** Move I/O to async tasks or background threads. Communicate results back via messages.

### 6.3 Unbounded Collection

**Impact: MEDIUM (memory growth without limit)**

Collections that grow without bounds (log buffers, event histories, undo stacks) eventually consume all available memory, causing OOM or degraded performance from allocation pressure.

**Detection:** `Vec`, `VecDeque`, or other collections with `push` but no eviction or capacity limit.

**Fix:** Use bounded buffers with a maximum capacity. Evict oldest entries when full (ring buffer pattern).

---

## 7. Architecture Review Workflow

A structured process for evaluating code architecture against the anti-patterns and principles defined above.

### Step 1: Identify Files to Review

Categorize changed files by architectural layer:
- **Hot path** — latency-sensitive data processing (zero-alloc, lock-free rules apply)
- **UI layer** — presentation and event handling (UI anti-patterns apply)
- **Infrastructure** — tools, build, persistence (architectural anti-patterns apply)
- **Cross-cutting** — changes spanning multiple layers (all rules apply)

### Step 2: Run Anti-Pattern Detection

**Hot path files:**
- Check for `Mutex`, `RwLock` usage (reject per 2.1)
- Check for `Vec::new()`, `Box::new()`, `String::from()` (reject per 2.2)
- Check for `Box<dyn ...>`, `&dyn ...` (reject per 2.3)
- Check for `HashMap` usage (flag per 2.5)

**All files:**
- Check for god objects (structs with 6+ responsibilities)
- Check for circular dependencies (module A imports B, B imports A)
- Check for tight coupling (concrete types instead of traits at boundaries)
- Check for leaky abstractions (internal details crossing module boundaries)

### Step 3: Score Architecture Dimensions

Use the scoring rubric (5.1) to evaluate each dimension. Design Efficiency is weighted 2x.

### Step 4: Run Quality Gates

Use the quality gates checklist (5.2). Reject if any gate fails.

### Step 5: Classify Technical Debt

For issues not immediately fixable, classify using the tech debt format (5.3).

### Output

Return findings as structured review with:
- Anti-pattern violations (with locations and fix recommendations)
- Dimension scores
- Quality gate pass/fail
- Technical debt items (if any)
