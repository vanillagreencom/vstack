# Sections

This file defines all sections, their ordering, impact levels, and descriptions.
The section ID (in parentheses) is the filename prefix used to group rules.

---

## 1. Architectural Anti-Patterns (arch)

**Impact:** CRITICAL
**Description:** Structural violations that degrade maintainability, testability, and scalability. Detect during design reviews and reject in PRs.

## 2. Performance Anti-Patterns (perf)

**Impact:** CRITICAL
**Description:** Patterns that violate hot-path performance constraints. Must be rejected in latency-sensitive execution paths.

## 3. Lock-Free Anti-Patterns (lock)

**Impact:** HIGH
**Description:** Concurrency mistakes in lock-free code — wrong ordering, missing fences, lifetime escapes. Cause data races and corruption.

## 4. Error Handling & Data Integrity (err)

**Impact:** HIGH
**Description:** Error handling strategy and data immutability rules. Violations cause silent data corruption or missed failures.

## 5. Review Process (review)

**Impact:** MEDIUM
**Description:** Architecture review scoring, quality gates, and technical debt classification. Provides consistent evaluation framework.

## 6. UI Anti-Patterns (ui)

**Impact:** MEDIUM
**Description:** UI-layer patterns that cause frame drops, frozen interfaces, or memory growth.
