# Sections

This file defines all sections, their ordering, impact levels, and descriptions.
The section ID (in parentheses) is the filename prefix used to group rules.

---

## 1. Hard Rules (hr)

**Impact:** CRITICAL
**Description:** Non-negotiable constraints from Iced 0.14 framework behavior. Violations cause silent breakage — widgets stop responding, events misroute, or state corrupts without errors.

## 2. Development Practices (dev)

**Impact:** HIGH
**Description:** Practices that prevent common Iced development pitfalls — API drift, runtime panics, redundant subscriptions, and missed performance regressions.

## 3. Cache & Multi-Window (cache)

**Impact:** HIGH
**Description:** Rules for managing cached/mirrored UI state across panes and windows. Stale caches cause visible bugs that are hard to reproduce.

## 4. Elm Architecture (elm)

**Impact:** MEDIUM
**Description:** Structural patterns for organizing Iced Elm Architecture applications. Violations cause coupling, bloated root modules, and difficult-to-test code.

## 5. Interaction (interaction)

**Impact:** MEDIUM
**Description:** Gotchas with mouse areas, overlays, and drag/drop that cause events to silently stop working.
