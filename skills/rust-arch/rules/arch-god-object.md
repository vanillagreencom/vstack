---
title: God Object
impact: CRITICAL
impactDescription: hard to test, modify, or reason about
tags: architecture, modularity, responsibility
---

## God Object

**Impact: CRITICAL (hard to test, modify, or reason about)**

A struct with 6+ distinct responsibilities violates single-responsibility principle. It becomes a change magnet — every feature touches it, making parallel development and testing difficult.

**Indicator:** Struct with methods spanning unrelated domains (parsing, routing, rendering, persistence).

**Fix:** Split into focused components, each owning one responsibility. Connect via trait interfaces.
