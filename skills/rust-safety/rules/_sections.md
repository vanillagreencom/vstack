# Sections

This file defines all sections, their ordering, impact levels, and descriptions.
The section ID (in parentheses) is the filename prefix used to group rules.

---

## 1. SAFETY Comments (safety)

**Impact:** CRITICAL
**Description:** Standards for documenting unsafe code. Every unsafe block requires a SAFETY comment proving soundness — covering validity, alignment, aliasing, initialization, and lifetime.

## 2. Unsafe Block Audit (unsafe)

**Impact:** CRITICAL
**Description:** Per-block checklist for auditing unsafe Rust code. Covers SAFETY comments, pointer validation, UB analysis, test coverage, and panic-path safety.

## 3. Memory Safety (mem)

**Impact:** CRITICAL
**Description:** Memory safety invariants that must hold for all unsafe code — no use-after-free, double-free, uninitialized reads, out-of-bounds access, or data races.

## 4. Raw Pointer Audit (ptr)

**Impact:** CRITICAL
**Description:** Per-pointer checklist for raw pointer usage — provenance tracking, validity, alignment, lifetime, and aliasing rules.

## 5. Lock-Free Structures (lockfree)

**Impact:** HIGH
**Description:** Audit rules for lock-free data structures — loom testing, atomic ordering justification, fence coverage, memory reclamation, and ABA prevention.

## 6. Crossbeam Epoch (epoch)

**Impact:** HIGH
**Description:** Audit rules specific to crossbeam epoch-based memory reclamation — pin before load, guard lifetime, deferred destruction, and scope escapes.

## 7. Security (sec)

**Impact:** HIGH
**Description:** Security audit rules for code handling external input or exposed via unsafe interfaces — input validation, panic prevention, DoS mitigation, and checked arithmetic.

## 8. Severity Classification (sev)

**Impact:** MEDIUM
**Description:** Definitions for classifying audit findings by severity and the corresponding merge-blocking actions.

## 9. Sanitizers (san)

**Impact:** HIGH
**Description:** Runtime and compile-time sanitizers for detecting memory errors, data races, and undefined behavior — AddressSanitizer, ThreadSanitizer, MemorySanitizer, and Miri with CI integration.

## 10. Fuzzing (fuzz)

**Impact:** HIGH
**Description:** Coverage-guided fuzz testing with cargo-fuzz and libFuzzer — target selection, structured fuzzing, corpus management, and ASan integration.

## 11. Supply Chain (supply)

**Impact:** MEDIUM
**Description:** Supply chain security for Rust dependencies — advisory database checks, peer review of third-party code, and dependency hygiene practices.
