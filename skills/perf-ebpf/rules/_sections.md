# Sections

This file defines all sections, their ordering, impact levels, and descriptions.
The section ID (in parentheses) is the filename prefix used to group rules.

---

## 1. Aya Framework (aya)

**Impact:** HIGH
**Description:** Aya project setup, program type selection, and map patterns. Covers workspace layout, kernel-side vs userspace crate separation, program type matching to use case, and efficient kernel-to-userspace data transfer via maps.

## 2. bpftrace Diagnostics (trace)

**Impact:** HIGH
**Description:** Production-safe bpftrace one-liners for syscall analysis, latency histograms, scheduling diagnostics, and network troubleshooting. No recompilation required, negligible overhead.

## 3. Verifier & Debugging (debug)

**Impact:** MEDIUM
**Description:** BPF verifier error resolution and program inspection with bpftool. Covers common verifier rejections, capability requirements, and runtime debugging of loaded programs and maps.
