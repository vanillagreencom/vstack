---
title: Split at 5+ Match Arms
impact: HIGH
tags: match, dispatch, extraction
---

## Split at 5+ Match Arms

**Impact: HIGH (unreadable dispatch handlers)**

When a handler dispatches 5+ message types with non-trivial logic, split into focused helpers. The dispatcher becomes a thin match → method call.
