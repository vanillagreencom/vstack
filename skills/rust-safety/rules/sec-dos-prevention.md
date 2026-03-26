---
title: DoS Prevention
impact: HIGH
impactDescription: Unbounded resources enable denial of service
tags: security, dos, rate-limit, resources
---

## DoS Prevention

**Impact: HIGH (unbounded resources enable denial of service)**

Enforce resource limits on all external-facing interfaces: rate limiting on request handlers, bounded queue sizes, maximum allocation sizes, and timeout values. An attacker must not be able to exhaust memory, CPU, or file descriptors through crafted input.
