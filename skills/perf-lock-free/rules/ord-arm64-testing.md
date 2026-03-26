---
title: Test on ARM64 to Catch Weak-Memory Bugs
impact: HIGH
impactDescription: Ordering bugs hidden by x86 strong memory model
tags: arm64, weak_memory, testing, apple_silicon
---

## Test on ARM64 to Catch Weak-Memory Bugs

**Impact: HIGH (ordering bugs hidden by x86 strong memory model)**

x86 has a strong memory model (TSO) that masks many ordering bugs. ARM64 (Apple Silicon, AWS Graviton) has a weakly-ordered memory model where missing Acquire/Release fences cause real failures. Always test lock-free code on ARM64 in addition to x86.

Loom's model checker does explore weak-memory orderings, but running real tests on ARM64 hardware catches issues in library code and compiler code generation that loom's model doesn't cover.

### Platform Barrier Cost

Memory ordering has different hardware costs per architecture:

| Architecture | Relaxed | Acquire | Release | AcqRel | SeqCst |
|-------------|---------|---------|---------|--------|--------|
| x86/x86_64 (TSO) | free | free | free | free | `mfence` or `lock` prefix |
| ARM64 | free | `dmb ishld` | `dmb ish` | `dmb ish` | `dmb ish` + `dmb ish` |
| POWER | free | `lwsync` + `isync` | `lwsync` | `lwsync` | `sync` |
| RISC-V (RVWMO) | free | `fence r,rw` | `fence rw,w` | `fence.tso` | `fence rw,rw` |

x86 TSO gives Acquire/Release for free — the hardware enforces store-to-load ordering by default. Only SeqCst requires an explicit `mfence`. This means ordering bugs on x86 are **silent** until deployed on ARM64 or tested under loom.

This is why ARM64 testing is not optional — it is the only way to surface ordering bugs that x86 TSO masks.
