# Sections

This file defines all sections, their ordering, impact levels, and descriptions.
The section ID (in parentheses) is the filename prefix used to group rules.

---

## 1. Environment Tiers (env)

**Impact:** CRITICAL
**Description:** Core vs alloc vs std tier selection and crate-level declarations. Understanding which types and traits live in which tier is fundamental to no_std development. Always start from core and escalate only when required.


## 2. Panic & Allocator (rt)

**Impact:** CRITICAL
**Description:** Panic handlers, global allocators, and OOM handling. In no_std environments, you must provide your own panic handler and, if using alloc, a global allocator. These are hard requirements — the binary will not link without them.


## 3. Portable Library Design (lib)

**Impact:** HIGH
**Description:** Feature-gated std support, core-only API patterns, and error handling without std. Libraries that support both std and no_std consumers reach the widest audience. Careful API design with feature gates makes this achievable.


## 4. Embedded Patterns (embed)

**Impact:** HIGH
**Description:** Entry points, memory layout, and HAL abstraction for bare-metal targets. These patterns apply when targeting microcontrollers and other embedded platforms where there is no OS.


## 5. Testing (test)

**Impact:** MEDIUM
**Description:** Host-based testing strategies and defmt logging for on-device test execution. Testing no_std code requires strategies for both fast host iteration and hardware validation.
