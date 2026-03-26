---
title: Doc Comment Conventions
impact: MEDIUM
tags: docs, comments, style
---

## Doc Comment Conventions

**Impact: MEDIUM**

- Backticks for all code refs: `Box::into_raw`, `repr(C)`, `UnsafeCell`
- Full paths for external items: `std::mem::MaybeUninit`
- Add `# Panics` doc section if function can panic
- Add `# Errors` doc section if function returns `Result`
- Add `#[must_use]` on pure functions returning values
