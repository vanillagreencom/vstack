---
title: Single Message Per Interaction
impact: CRITICAL
impactDescription: Race conditions and unpredictable state
tags: message, update, view, state_machine
---

## Single Message Per Interaction

**Impact: CRITICAL (race conditions and unpredictable state)**

Each widget interaction produces exactly one message. For composite actions (e.g., tab press that might become a drag), use state machines in `update()` rather than emitting multiple messages from `view()`.
