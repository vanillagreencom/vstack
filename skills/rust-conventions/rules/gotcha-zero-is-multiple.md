---
title: 0.is_multiple_of(n) Returns True
impact: MEDIUM
tags: gotcha, numeric, sampling, rate_limiting
---

## 0.is_multiple_of(n) Returns True

**Impact: MEDIUM (sampling/rate-limiting fires on first iteration)**

RFC 2413: 0 is a multiple of every integer. Guard sampling/rate-limiting:

**Incorrect:**

```rust
if poll_count.is_multiple_of(INTERVAL) { log_metrics(); }
// Fires immediately at poll_count == 0
```

**Correct:**

```rust
if poll_count > 0 && poll_count.is_multiple_of(INTERVAL) { log_metrics(); }
```
