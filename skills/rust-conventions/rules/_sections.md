# Sections

This file defines all sections, their ordering, impact levels, and descriptions.
The section ID (in parentheses) is the filename prefix used to group rules.

---

## 1. Style & Formatting (style)

**Impact:** MEDIUM
**Description:** Consistent Rust code style — clippy pedantic, imports, doc comments, safety comments, formatting. Reduces review friction and prevents clippy failures.

## 2. Code Structure (struct)

**Impact:** HIGH
**Description:** How to organize and split Rust code — file limits, modularity, extraction patterns. Violations cause bloated files, duplicated logic, and architectural drift.

## 3. Testing (test)

**Impact:** HIGH
**Description:** Test structure, flaky test avoidance, and test quality. Violations cause CI flakiness, false confidence, and hard-to-debug failures.

## 4. Completeness (complete)

**Impact:** HIGH
**Description:** Definition of done — when tests, benchmarks, and docs are required. Prevents gaps in coverage and undocumented public APIs.

## 5. Navigation (nav)

**Impact:** MEDIUM
**Description:** How to explore Rust codebases efficiently using LSP and grep. Prevents wasted time reading entire files when a semantic query would suffice.

## 6. Gotchas (gotcha)

**Impact:** MEDIUM
**Description:** Specific Rust language footguns that cause subtle bugs.
