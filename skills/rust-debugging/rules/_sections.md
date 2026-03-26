# Sections

This file defines all sections, their ordering, impact levels, and descriptions.
The section ID (in parentheses) is the filename prefix used to group rules.

---

## 1. Debugger Setup (dbg)

**Impact:** HIGH
**Description:** GDB and LLDB configuration for Rust — pretty-printers, breakpoints, VS Code integration. Correct setup prevents wasted time reading raw struct internals and enables efficient crash diagnosis.

## 2. Panic Analysis (panic)

**Impact:** HIGH
**Description:** Panic triage, custom hooks, and backtrace configuration. Violations cause undiagnosed crashes, missing context in production logs, and slow incident response.

## 3. Runtime Introspection (rt)

**Impact:** HIGH
**Description:** dbg!() macro, tracing instrumentation, and tokio-console for async debugging. Enables visibility into runtime behavior without resorting to println-driven debugging.

## 4. Core Dump Pipeline (core)

**Impact:** HIGH
**Description:** Core dump collection, configuration, and non-interactive triage. Ensures every crash produces a usable dump and that analysis can be automated in CI without interactive debugger sessions.

## 5. Debug Symbols & Tools (sym)

**Impact:** MEDIUM
**Description:** Debug symbol management, release profile configuration, and core dump analysis. Ensures post-mortem debugging is possible and release binaries remain debuggable when needed.
