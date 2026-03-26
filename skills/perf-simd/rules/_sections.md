# Sections

This file defines all sections, their ordering, impact levels, and descriptions.
The section ID (in parentheses) is the filename prefix used to group rules.

---

## 1. Auto-Vectorization (auto)

**Impact:** HIGH
**Description:** Verifying and unblocking LLVM auto-vectorization in Rust hot paths. Violations leave performance on the table — scalar code where SIMD should be free.

## 2. Manual SIMD (simd)

**Impact:** HIGH
**Description:** Explicit SIMD intrinsics via core::arch, runtime CPU detection, and dispatch patterns. Violations cause crashes on unsupported hardware, frequency throttling, or incorrect results from misaligned access.

## 3. Portable SIMD (portable)

**Impact:** MEDIUM
**Description:** Platform-independent SIMD via std::simd and drop-in SIMD-accelerated crates. Reduces manual intrinsic complexity when nightly is acceptable or when crate alternatives exist.
