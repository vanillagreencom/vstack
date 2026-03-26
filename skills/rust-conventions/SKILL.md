---
name: rust-conventions
description: Rust code style, testing patterns, modularity, and completeness rules. Use when writing, reviewing, or refactoring Rust code — covers clippy pedantic, imports, test structure, file organization, flaky test avoidance, and definition of done.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Rust Conventions

Style, testing, structure, and completeness rules for Rust codebases, prioritized by impact.

## When to Apply

Reference these guidelines when:
- Writing or reviewing Rust code
- Organizing modules, splitting files, or extracting logic
- Writing or debugging tests (especially flaky ones)
- Adding new public APIs or hot-path code
- Running clippy pedantic or resolving lint warnings

## Rule Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Style & Formatting | MEDIUM | `style-` |
| 2 | Code Structure | HIGH | `struct-` |
| 3 | Testing | HIGH | `test-` |
| 4 | Completeness | HIGH | `complete-` |
| 5 | Navigation | MEDIUM | `nav-` |
| 6 | Gotchas | MEDIUM | `gotcha-` |

## Quick Reference

### 1. Style & Formatting (MEDIUM)

- `style-doc-comments` - Backticks for code refs, full paths for external items, # Panics/# Errors sections
- `style-clippy-pedantic` - inline not inline(always), &T over T, casting lint allowances
- `style-imports` - std → external → crate → super ordering, no globs except preludes
- `style-safety-comments` - SAFETY: comment on every unsafe block

### 2. Code Structure (HIGH)

- `struct-no-magic-numbers` - Named constants for non-obvious numeric literals
- `struct-single-init` - One constructor calls the other, or both call shared helper
- `struct-extract-shared` - Extract duplicated computation to shared struct/helper
- `struct-early-return` - Early-return on no-op in frame-frequency functions
- `struct-split-match-arms` - Split 5+ match arms into focused helpers
- `struct-per-instance-state` - Per-instance state, not global fields
- `struct-file-limits` - ≤1,500 lines production, ≤1,000 test; split by contract
- `struct-declarative-imperative` - Declarative for config, imperative for hot paths

### 3. Testing (HIGH)

- `test-path-pattern` - #[path] sibling pattern for pub(crate) test access
- `test-feature-gates` - Module-level cfg, not per-item; MIRI gating for unsafe only
- `test-mirror-modules` - Split tests when production modules split
- `test-flaky-avoidance` - Signals not iterations, barriers, no sleep, no static mutable
- `test-quality` - Verify setup, question existing patterns, names match behavior

### 4. Completeness (HIGH)

- `complete-tests-required` - New public functions need tests; don't reduce coverage
- `complete-benchmarks-for-hot-paths` - Criterion benchmark for perf-sensitive code
- `complete-definition-of-done` - Compiles clean, tested, benchmarked if hot, docs if public

### 5. Navigation (MEDIUM)

- `nav-lsp-vs-grep` - LSP for semantics, grep for text patterns, cargo check for stale diagnostics

### 6. Gotchas (MEDIUM)

- `gotcha-zero-is-multiple` - 0.is_multiple_of(n) returns true; guard with > 0

## How to Use

Read individual rule files for detailed explanations and code examples:

```
rules/style-clippy-pedantic.md
rules/test-flaky-avoidance.md
```

Each rule file contains:
- Brief explanation of why it matters
- Incorrect code example with explanation
- Correct code example with explanation

## Resources

### ctx7 CLI

| Library | ctx7 ID | Use For |
|---------|---------|---------|
| Rust std | `/websites/doc_rust-lang_stable_std` | Standard library API |
| tokio | `/websites/rs_tokio` | Async runtime |
| serde | `/websites/rs_serde` | Serialization |

## Full Compiled Document

For the complete guide with all rules expanded: `AGENTS.md`
