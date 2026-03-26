---
title: Review Agent Lifecycle Management
impact: HIGH
impactDescription: Premature shutdown loses review context; leaked agents waste resources
tags: lifecycle, review, shutdown
---

## Review Agent Lifecycle Management

**Impact: HIGH (premature shutdown loses review context; leaked agents waste resources)**

Review agents persist across fix → re-review cycles within the review workflow. The review workflow manages their lifecycle:
- Spawn if review agents state is empty, skip if already alive
- After fixes, selectively shut down non-reporting agents for low-risk changes; keep all alive if risk flags present
- Full shutdown when review passes, clear review agents state

QA agents spawn and shut down per-agent within the review workflow (sequential execution, less benefit from context preservation).
