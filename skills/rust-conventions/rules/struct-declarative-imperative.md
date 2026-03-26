---
title: Declarative vs Imperative
impact: MEDIUM
tags: organization, style, hot_path
---

## Declarative vs Imperative

**Impact: MEDIUM**

- Configuration/setup → declarative (structs, builders, config files)
- Hot path execution → imperative (explicit control, zero-cost)
- Cold path queries → declarative acceptable (SQL, iterators)
- UI bindings → declarative (Elm architecture, reactive patterns)
