---
title: Layered Architecture
impact: CRITICAL
impactDescription: foundational constraint for dependency management
tags: architecture, layers, dependencies
---

## Layered Architecture

**Impact: CRITICAL (foundational constraint for dependency management)**

Applications should be organized in layers where dependencies flow DOWN only. Higher layers (UI, application logic) depend on lower layers (core engine, infrastructure). Lower layers never import from higher layers.

```
┌─────────────────────────────────────────┐
│           UI Layer (Presentation)       │
├─────────────────────────────────────────┤
│          Core Engine (Business Logic)   │
│    ┌─────────┬─────────┬─────────┐      │
│    │ Domain  │ Domain  │ Domain  │      │
│    │  Area A │  Area B │  Area C │      │
│    └─────────┴─────────┴─────────┘      │
├─────────────────────────────────────────┤
│        Infrastructure (Storage, IPC)    │
└─────────────────────────────────────────┘
```

**Rule:** Modules communicate via defined interfaces, never internal types. Each module owns its data and exposes only its public contract.
