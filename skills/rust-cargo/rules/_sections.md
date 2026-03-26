# Sections

This file defines all sections, their ordering, impact levels, and descriptions.
The section ID (in parentheses) is the filename prefix used to group rules.

---

## 1. Workspace Management (ws)

**Impact:** HIGH
**Description:** Cargo workspace layout, dependency centralization, crate splitting strategy, and feature flag discipline. Violations cause version conflicts, slow builds, and broken feature combinations.

## 2. Build Tooling (tool)

**Impact:** HIGH
**Description:** Essential cargo plugins for policy enforcement, fast testing, and dependency hygiene. Missing tooling lets vulnerabilities, unused deps, and slow test suites slip through CI.

## 3. Build Performance (perf)

**Impact:** HIGH
**Description:** Compilation speed optimizations — linker choice, caching, codegen backend, incremental build tuning. Slow builds kill iteration speed and developer productivity.

## 4. Release & CI (ci)

**Impact:** MEDIUM
**Description:** Release profile configuration, monomorphization bloat detection, and binary size tracking. Misconfigurations waste runtime performance or produce unnecessarily large binaries.
