---
title: No Shared Reference Scope Escape
impact: HIGH
impactDescription: Escaped references become dangling after guard drop
tags: crossbeam, epoch, scope, reference
---

## No Shared Reference Scope Escape

**Impact: HIGH (escaped references become dangling after guard drop)**

Shared references obtained through an epoch guard must not escape the guard's scope. Collect owned copies of the data you need before dropping the guard. If you need to return data from an epoch-protected section, clone or copy it into owned storage within the guard's lifetime.
