---
title: SAFETY Comment Audit Questions
impact: CRITICAL
impactDescription: Unverified safety claims mask unsoundness
tags: unsafe, safety, review, audit
---

## SAFETY Comment Audit Questions

**Impact: CRITICAL (unverified safety claims mask unsoundness)**

When reviewing SAFETY comments, verify each claim against the code:

- Is each claim verifiable from the surrounding code?
- Are line number references accurate and up to date?
- Do invariants hold across ALL call sites, not just the obvious one?
- What happens if preconditions are violated — panic, UB, or graceful error?

Every SAFETY comment must be provably correct from local context. If a claim requires understanding distant code, the invariant should be enforced closer to the unsafe block (e.g., a wrapper type with a validity invariant).
