---
title: Fail Fast Over Silent Degradation
impact: HIGH
impactDescription: silent failures hide bugs until production
tags: error-handling, fail-fast, safety
---

## Fail Fast Over Silent Degradation

**Impact: HIGH (silent failures hide bugs until production)**

Critical execution paths must fail loudly rather than silently degrade. Skipping invalid data, queuing indefinitely on missing connections, or approximating on validation failure all hide problems until they compound into production incidents.

- Invalid data: panic or return error immediately; never skip or substitute
- Missing connection: error immediately; never queue indefinitely
- Validation failure: halt processing; never approximate

**Exception:** Observability tools (profilers, metrics, logging) should degrade gracefully with warnings rather than crash the application.
