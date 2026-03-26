# Sections

This file defines all sections, their ordering, impact levels, and descriptions.
The section ID (in parentheses) is the filename prefix used to group rules.

---

## 1. Soundness (sound)

**Impact:** CRITICAL
**Description:** Violations of these rules are undefined behavior. The compiler and sanitizers may not catch them, and they can cause silent data corruption or security vulnerabilities.

## 2. Verification (verify)

**Impact:** CRITICAL
**Description:** Rules for choosing and applying the correct verification tool. Using the wrong tool gives false confidence — e.g., TSAN cannot verify atomic fences.

## 3. Ordering (ord)

**Impact:** HIGH
**Description:** Atomic memory ordering rules. Incorrect ordering causes data races on weakly-ordered architectures (ARM64) that may not manifest on x86.

## 4. Epoch Reclamation (epoch)

**Impact:** HIGH
**Description:** Rules for safe use of epoch-based memory reclamation (crossbeam-epoch). Violations cause use-after-free in lock-free data structures.

## 5. Testing (test)

**Impact:** MEDIUM
**Description:** Best practices for loom model testing. Poor test design leads to state space explosion or insufficient coverage.
