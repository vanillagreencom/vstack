---
title: Pointer Alignment Verification
impact: CRITICAL
impactDescription: Misaligned pointer dereference is undefined behavior
tags: pointer, alignment, repr, unsafe
---

## Pointer Alignment Verification

**Impact: CRITICAL (misaligned pointer dereference is undefined behavior)**

Verify that the pointer's alignment matches the pointee type's alignment requirement. Common sources of misalignment: casting between pointer types with different alignment (`*const u8` to `*const u64`), pointer arithmetic that breaks alignment, and packed struct field references.
