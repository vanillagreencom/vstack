---
name: rust-safety
description: Safety audit checklists, SAFETY comment standards, and violation reporting for unsafe Rust code. Use when reviewing unsafe blocks, auditing unsafe code, documenting safety invariants, or verifying lock-free structures.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Safety Audit Patterns

Checklists and rules for auditing unsafe Rust code, prioritized by impact.

## When to Apply

Reference these guidelines when:
- Reviewing or writing `unsafe` blocks
- Auditing lock-free data structures or atomic operations
- Documenting SAFETY comments for pointer operations
- Verifying memory safety invariants (aliasing, lifetime, initialization)
- Reviewing code that handles external input through unsafe interfaces
- Classifying audit findings by severity

## Rule Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | SAFETY Comments | CRITICAL | `safety-` |
| 2 | Unsafe Block Audit | CRITICAL | `unsafe-` |
| 3 | Memory Safety | CRITICAL | `mem-` |
| 4 | Raw Pointer Audit | CRITICAL | `ptr-` |
| 5 | Lock-Free Structures | HIGH | `lockfree-` |
| 6 | Crossbeam Epoch | HIGH | `epoch-` |
| 7 | Security | HIGH | `sec-` |
| 8 | Severity Classification | MEDIUM | `sev-` |
| 9 | Sanitizers | HIGH | `san-` |
| 10 | Fuzzing | HIGH | `fuzz-` |
| 11 | Supply Chain | MEDIUM | `supply-` |

## Quick Reference

### 1. SAFETY Comments (CRITICAL)

- `safety-comment-standard` - Every unsafe block needs SAFETY comment covering validity, alignment, aliasing, initialization, lifetime
- `safety-audit-questions` - Verify each SAFETY claim is provable from local context

### 2. Unsafe Block Audit (CRITICAL)

- `unsafe-block-checklist` - Per-block checklist: SAFETY comment, pointer ops, UB analysis, MIRI coverage, panic paths

### 3. Memory Safety (CRITICAL)

- `mem-use-after-free` - Verify no pointer/reference used after allocation is freed
- `mem-double-free` - Verify ownership transferred exactly once
- `mem-uninitialized-reads` - Use MaybeUninit; verify initialization before read
- `mem-out-of-bounds` - Verify pointer arithmetic and slice indexing within allocated bounds
- `mem-data-races` - Verify shared mutable state has proper synchronization

### 4. Raw Pointer Audit (CRITICAL)

- `ptr-provenance` - Track pointer origin; document provenance chain in SAFETY comment
- `ptr-validity` - Validate non-null, within allocation, not dangling before dereference
- `ptr-alignment` - Verify pointer alignment matches pointee type requirement
- `ptr-lifetime` - Verify pointed-to data outlives every use of the pointer
- `ptr-aliasing` - Enforce single-writer-or-multiple-readers; use UnsafeCell for interior mutability

### 5. Lock-Free Structures (HIGH)

- `lockfree-loom-testing` - Every lock-free structure must have passing loom tests
- `lockfree-atomic-ordering` - Document and justify memory ordering on every atomic operation
- `lockfree-fence-coverage` - No atomic::fence without loom test proving necessity
- `lockfree-memory-reclamation` - Use epoch, hazard pointers, or owned transfer for safe reclamation
- `lockfree-aba-prevention` - Address ABA in CAS-based structures via tagged pointers or epoch

### 6. Crossbeam Epoch (HIGH)

- `epoch-pin-before-load` - epoch::pin() before every atomic load of shared data
- `epoch-guard-lifetime` - Guard lifetime must contain all access to protected data
- `epoch-defer-destroy` - Use defer_destroy() for epoch-protected data removal
- `epoch-no-manual-drop` - Never manually drop epoch-protected data
- `epoch-no-scope-escape` - Clone/copy data into owned storage before dropping guard

### 7. Security (HIGH)

- `sec-input-validation` - Bounds-check all external inputs before unsafe use
- `sec-no-panic-on-malformed` - Return Result on external data; never unwrap
- `sec-dos-prevention` - Rate limiting, bounded queues, resource limits on external interfaces
- `sec-error-propagation` - Propagate errors; never silently ignore
- `sec-checked-arithmetic` - Checked/saturating arithmetic where overflow is possible

### 8. Severity Classification (MEDIUM)

- `sev-classification` - CRITICAL/HIGH block merge; MEDIUM allows follow-up; LOW is advisory

### 9. Sanitizers (HIGH)

- `san-tsan-msan` - ThreadSanitizer for data races; MemorySanitizer for uninitialized reads
- `san-miri-ci` - Miri for compile-time UB: dangling pointers, Stacked Borrows, invalid discriminants
- `san-ci-integration` - CI pipeline: Miri + ASan blocking, TSan advisory, matrix strategy

### 10. Fuzzing (HIGH)

- `fuzz-target-selection` - Fuzz parsers, deserializers, FFI, unsafe; skip trivial safe code

### 11. Supply Chain (MEDIUM)

- `supply-dependency-hygiene` - Minimal deps, cargo-machete, duplicate detection, lockfile pinning

## How to Use

Read individual rule files for detailed explanations and code examples:

```
rules/safety-comment-standard.md
rules/lockfree-atomic-ordering.md
```

Each rule file contains:
- Brief explanation of why it matters
- Incorrect code example with explanation (where applicable)
- Correct code example with explanation (where applicable)

## Resources

Documentation lookup order: local skill files → ctx7 CLI → web fallback.

### ctx7 CLI

| Library | ctx7 ID | Use For |
|---------|---------|---------|
| Rust std | `/websites/doc_rust-lang_stable_std` | unsafe semantics, ptr, mem, MaybeUninit, UnsafeCell |
| crossbeam | `/crossbeam-rs/crossbeam` | Epoch-based reclamation, atomic utilities |

## Full Compiled Document

For the complete guide with all rules expanded: `AGENTS.md`
