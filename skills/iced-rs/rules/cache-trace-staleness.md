---
title: Trace Staleness Before Coding
impact: HIGH
impactDescription: Stale caches cause visible bugs that are hard to reproduce
tags: cache, state, invalidation, multi_window
---

## Trace Staleness Before Coding

**Impact: HIGH (stale caches cause visible bugs that are hard to reproduce)**

When adding cached or mirrored UI state (snapshots, summaries, registries), enumerate every mutation path that can stale it before writing code: direct handlers, drag/drop helpers, transfer/split, open/close, reset, and foreign-window events.
