---
title: Error Propagation
impact: HIGH
impactDescription: Silently ignored errors mask security-critical failures
tags: security, error, propagation, handling
---

## Error Propagation

**Impact: HIGH (silently ignored errors mask security-critical failures)**

Errors must be propagated, never silently ignored. A silently swallowed error in validation, authentication, or authorization code can make an entire security check a no-op. Use `?` for propagation or explicitly handle and log every error branch.
