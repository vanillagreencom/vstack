# Rust Conventions

**Version 1.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when writing,
> reviewing, or refactoring Rust code. Humans may also find it useful,
> but guidance here is optimized for automation and consistency by
> AI-assisted workflows.

---

## Abstract

Style, testing, structure, and completeness rules for Rust codebases, prioritized by impact from high (code structure, testing, completeness) to medium (style, navigation, gotchas).

---

## Table of Contents

1. [Style & Formatting](#1-style--formatting) — **MEDIUM**
   - 1.1 [Doc Comment Conventions](#11-doc-comment-conventions)
   - 1.2 [Clippy Pedantic](#12-clippy-pedantic)
   - 1.3 [Import Conventions](#13-import-conventions)
   - 1.4 [Safety Comments](#14-safety-comments)
2. [Code Structure](#2-code-structure) — **HIGH**
   - 2.1 [No Magic Numbers](#21-no-magic-numbers)
   - 2.2 [Single Source of Initialization](#22-single-source-of-initialization)
   - 2.3 [Extract Shared Computation](#23-extract-shared-computation)
   - 2.4 [Early Return on No-Op](#24-early-return-on-no-op)
   - 2.5 [Split at 5+ Match Arms](#25-split-at-5-match-arms)
   - 2.6 [Per-Instance State](#26-per-instance-state)
   - 2.7 [File Size Limits](#27-file-size-limits)
   - 2.8 [Declarative vs Imperative](#28-declarative-vs-imperative)
3. [Testing](#3-testing) — **HIGH**
   - 3.1 [Unit Test Path Pattern](#31-unit-test-path-pattern)
   - 3.2 [Test Feature Gates](#32-test-feature-gates)
   - 3.3 [Test Modules Mirror Production](#33-test-modules-mirror-production)
   - 3.4 [Flaky Test Avoidance](#34-flaky-test-avoidance)
   - 3.5 [Test Quality](#35-test-quality)
4. [Completeness](#4-completeness) — **HIGH**
   - 4.1 [New Public Functions Require Tests](#41-new-public-functions-require-tests)
   - 4.2 [Benchmarks for Hot Paths](#42-benchmarks-for-hot-paths)
   - 4.3 [Definition of Done](#43-definition-of-done)
5. [Navigation](#5-navigation) — **MEDIUM**
   - 5.1 [LSP vs Grep](#51-lsp-vs-grep)
6. [Gotchas](#6-gotchas) — **MEDIUM**
   - 6.1 [0.is_multiple_of(n) Returns True](#61-0is_multiple_ofn-returns-true)

---

## 1. Style & Formatting

**Impact: MEDIUM**

Consistent Rust code style — clippy pedantic, imports, doc comments, safety comments. Reduces review friction and prevents clippy failures.

### 1.1 Doc Comment Conventions

**Impact: MEDIUM**

- Backticks for all code refs: `Box::into_raw`, `repr(C)`, `UnsafeCell`
- Full paths for external items: `std::mem::MaybeUninit`
- Add `# Panics` doc section if function can panic
- Add `# Errors` doc section if function returns `Result`
- Add `#[must_use]` on pure functions returning values

### 1.2 Clippy Pedantic

**Impact: MEDIUM (CI failures from unaddressed lints)**

For projects running `-W clippy::pedantic -D warnings`:

- `#[inline]` not `#[inline(always)]` — let compiler decide
- Prefer `&T` over owned `T` for non-consumed params (`needless_pass_by_value`)
- `#[repr(...)]` before `#[derive(...)]`
- Run `cargo fmt` before commit — always
- Don't manually align inline comments — `cargo fmt` normalizes them

#### Casting Lints

Pedantic flags `as` casts. Add `#[allow(clippy::...)]` with a justification comment:

| Lint | When |
|------|------|
| `cast_possible_truncation` | `u128 as u64`, `usize as u16` — bounded values |
| `cast_sign_loss` | `i32 as u32` on known-positive/negated values |
| `cast_possible_wrap` | `usize as i32` for returns bounded by design |

### 1.3 Import Conventions

**Impact: MEDIUM (inconsistent imports cause merge conflicts and confusion)**

- **Module-level imports** — `use` statements at top of file. Feature-gated: `#[cfg(feature = "X")] use crate::module::Thing;`. Function-level only when: single use AND would cause name clash.
- **Grouping order**: std → external crates → `crate::` → `super::`/`self::`. Blank line between groups.
- **Prefer**: modules, types, macros. Use qualified paths for functions: `module::function()`.
- **Avoid glob imports** except: preludes, `use super::*` in test modules.
- **Avoid enum variant imports** except: `Some`, `None`, `Ok`, `Err`.

### 1.4 Safety Comments

**Impact: MEDIUM (unsafe blocks without justification are review blockers)**

Document every `unsafe` block with a `// SAFETY:` comment explaining why invariants hold. Items `pub` only for benchmark/integration test access get `#[doc(hidden)]` to suppress `missing_docs`.

---

## 2. Code Structure

**Impact: HIGH**

How to organize and split Rust code — file limits, modularity, extraction patterns. Violations cause bloated files, duplicated logic, and architectural drift.

### 2.1 No Magic Numbers

**Impact: HIGH (silent divergence when values change)**

Numeric literals used more than once or with non-obvious meaning must be named constants (`const` at module top). Includes: thresholds, percentages, pixel sizes, timing values. Exception: 0, 1, 2 in obvious contexts.

### 2.2 Single Source of Initialization

**Impact: HIGH (duplicate field lists drift silently)**

When two constructors initialize the same fields, one must call the other or both call a shared helper.

### 2.3 Extract Shared Computation

**Impact: HIGH (duplication = guaranteed divergence on next edit)**

If two functions compute the same derived values (geometry, dimensions, layout), extract to a shared struct or helper.

### 2.4 Early Return on No-Op

**Impact: HIGH (wasted computation every frame)**

Functions called every frame (view, overlay builders) must early-return when their output is unused (e.g., no active drag → skip overlay construction).

### 2.5 Split at 5+ Match Arms

**Impact: HIGH (unreadable dispatch handlers)**

When a handler dispatches 5+ message types with non-trivial logic, split into focused helpers. The dispatcher becomes a thin match → method call.

### 2.6 Per-Instance State

**Impact: HIGH (global field silently returns wrong data)**

When different instances (panes, tabs, widgets) have different dimensions/state, store per-instance. A single global field silently returns wrong data for the non-last-updated instance.

### 2.7 File Size Limits

**Impact: HIGH (files beyond tool limits can't be read in one pass)**

- Implementation: ≤1,500 lines per file. Approaching limit → split proactively by responsibility.
- Test files: ≤1,000 lines target, 1,500 hard limit.
- Split by contract, not by size: each module gets one clear responsibility and a minimal public API.
- Group by coupling (types + functions that change together stay together).
- Thin dispatchers stay in the parent; logic moves to focused modules.
- Never split types from the functions that exclusively operate on them.

### 2.8 Declarative vs Imperative

**Impact: MEDIUM**

- Configuration/setup → declarative (structs, builders, config files)
- Hot path execution → imperative (explicit control, zero-cost)
- Cold path queries → declarative acceptable (SQL, iterators)
- UI bindings → declarative (Elm architecture, reactive patterns)

---

## 3. Testing

**Impact: HIGH**

Test structure, flaky test avoidance, and test quality. Violations cause CI flakiness, false confidence, and hard-to-debug failures.

### 3.1 Unit Test Path Pattern

**Impact: HIGH (tests can't access pub(crate) items without this)**

Sibling file pattern for `pub(crate)` access:
- `module.rs` + `module_tests.rs` in same directory
- Source declares: `#[cfg(test)] #[path = "module_tests.rs"] mod tests;`
- Test imports: `use super::*;`

When a test file exceeds 1,000 lines, split into focused modules with descriptive names. Split modules may use explicit `use super::Type` or `use crate::` imports.

### 3.2 Test Feature Gates

**Impact: HIGH (tests run in wrong contexts or skip silently)**

- **Infrastructure features**: Module-level gating (`#[cfg(all(test, feature = "X"))]` on module declaration), not per-item `#[cfg]` on each test.
- **MIRI/sanitizer gating**: Test modules that don't exercise unsafe code → `#[cfg(all(test, not(miri)))]`. MIRI/ASAN only detect UB and memory errors in unsafe code.
- **Loom ordering models**: In-source `#[cfg(loom)] mod loom_tests` permitted in `#[path]` test files for simplified atomic ordering models co-located with the code they verify.

### 3.3 Test Modules Mirror Production

**Impact: HIGH (stale catch-all test files after extraction)**

When production responsibilities split into focused files, move the matching focused tests and shared fixtures into sibling `#[path]` test modules in the same change. Do not leave the old catch-all test file as the stale owner.

### 3.4 Flaky Test Avoidance

**Impact: HIGH (CI failures that can't be reproduced locally)**

- **Use signals, not iteration counts** — `while !done.load()` not `for _ in 0..10000`
- **Startup barriers before concurrent work** — ensure all threads ready before test begins
- **spin_loop() is not synchronization** — use `yield_now()`, channels, or condition variables
- **No static mutable state in tests** — use thread_local or per-test instances
- **Parallel tests must be isolated** — shared global state = flaky failures in CI
- **Drain loops bounded by known quantities** — track actual counts, not arbitrary iterations
- **Never rely on timing** — `sleep()` for synchronization is a bug waiting to happen
- **No probabilistic aggregate assertions** — each iteration must be self-contained

### 3.5 Test Quality

**Impact: HIGH (false confidence from misleading tests)**

- **Verify setup reaches target** — trace call chains before copying patterns. Early-throwing mocks can shadow later overrides.
- **Question existing patterns** — "parity" work propagates flaws silently. Verify originals are sound.
- **Names must match behavior** — `WhenXThrows` but X never runs = misleading test.

---

## 4. Completeness

**Impact: HIGH**

Definition of done — when tests, benchmarks, and docs are required. Prevents gaps in coverage and undocumented public APIs.

### 4.1 New Public Functions Require Tests

**Impact: HIGH (untested public API surface)**

Unit test for happy path + at least one error case. Exception: trivial getters/setters, generated code. Don't reduce coverage without reason — removing tests requires explanation in commit message.

### 4.2 Benchmarks for Hot Paths

**Impact: HIGH (performance regressions go undetected)**

When adding performance-sensitive code, add a Criterion benchmark. Benchmarked types must be `pub` (benches/ is an external crate). If modifying an existing hot path, verify the existing benchmark covers your change. Integration tests go in `tests/`; benchmarks go in `benches/`.

### 4.3 Definition of Done

**Impact: HIGH (incomplete features shipped)**

- Code compiles with no warnings
- Tests exist for new behavior
- Benchmarks exist if hot path
- Docs updated if public API
- No incomplete code — every commit must be production-ready
- Measure everything — no optimization without benchmarks
- Profile before claiming — performance claims need proof
- Fix at the source — change defaults, don't add options
- Concise over verbose when equivalent

---

## 5. Navigation

**Impact: MEDIUM**

How to explore Rust codebases efficiently using LSP and grep.

### 5.1 LSP vs Grep

**Impact: MEDIUM (wasted time reading entire files)**

- **Semantic queries → LSP** — findReferences, goToDefinition, incomingCalls for understanding code structure and impact
- **Text patterns → Grep** — string literals, log messages, config keys
- **Before refactoring** — Use LSP findReferences to understand full impact
- **Type uncertainty** — Use LSP hover instead of reading entire files
- **LSP returns 0 results?** — Don't trust it; fall back to Grep. Position mapping is unreliable.
- **Stale diagnostics after commits** — Verify with `cargo check` before acting on LSP warnings.

---

## 6. Gotchas

**Impact: MEDIUM**

Specific Rust language footguns that cause subtle bugs.

### 6.1 0.is_multiple_of(n) Returns True

**Impact: MEDIUM (sampling/rate-limiting fires on first iteration)**

RFC 2413: 0 is a multiple of every integer. Guard sampling/rate-limiting:

**Incorrect:**

```rust
if poll_count.is_multiple_of(INTERVAL) { log_metrics(); }
// Fires immediately at poll_count == 0
```

**Correct:**

```rust
if poll_count > 0 && poll_count.is_multiple_of(INTERVAL) { log_metrics(); }
```
