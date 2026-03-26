---
title: Pointer Provenance Tracking
impact: CRITICAL
impactDescription: Lost provenance causes undefined behavior under strict provenance rules
tags: pointer, provenance, raw, unsafe
---

## Pointer Provenance Tracking

**Impact: CRITICAL (lost provenance causes undefined behavior under strict provenance rules)**

For each raw pointer, track where it came from (its provenance). A pointer must be derived from a valid allocation and must not be fabricated from an integer without `with_addr()` or `from_exposed_addr()`. Document the provenance chain in the SAFETY comment.
