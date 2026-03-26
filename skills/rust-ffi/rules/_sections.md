# Sections

This file defines all sections, their ordering, impact levels, and descriptions.
The section ID (in parentheses) is the filename prefix used to group rules.

---

## 1. String & Data Handling (data)

**Impact:** CRITICAL
**Description:** Safe conversion of strings, slices, and owned data across the Rust/C boundary. Violations cause buffer overflows (missing null terminators), use-after-free (wrong ownership), and memory leaks (mismatched allocators).

## 2. Bindgen & Cbindgen (gen)

**Impact:** HIGH
**Description:** Automated binding generation and sys-crate organization. Violations cause stale bindings, manual transcription errors, and tangled safe/unsafe code in one crate.

## 3. Safe Wrappers (wrap)

**Impact:** HIGH
**Description:** Wrapping raw C handles and callbacks in safe Rust abstractions. Violations cause resource leaks (missing Drop), unsound Send/Sync, use-after-free (wrong lifetimes), and UB from panics unwinding through C frames.
