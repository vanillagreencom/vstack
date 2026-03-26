---
title: UI Thread Blocking
impact: MEDIUM
impactDescription: frozen interface during I/O
tags: ui, performance, blocking, async
---

## UI Thread Blocking

**Impact: MEDIUM (frozen interface during I/O)**

Synchronous I/O (file reads, network calls, database queries) on the main/UI thread blocks the event loop, freezing the interface for the duration of the operation.

**Detection:** Sync I/O calls in message handlers or view functions.

**Fix:** Move I/O to async tasks or background threads. Communicate results back via messages.
