---
name: rust-debugging
description: Debugging Rust programs with GDB/LLDB, panic triage, tokio-console, tracing, and IDE integration. Use when diagnosing crashes, analyzing panics, setting up debug tooling, or introspecting async runtime behavior.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Rust Debugging

Debugger setup, panic analysis, runtime introspection, and debug symbol management for Rust programs, prioritized by impact.

## When to Apply

Reference these guidelines when:
- Setting up GDB or LLDB for Rust debugging
- Triaging panics, crashes, or core dumps
- Adding tracing instrumentation or tokio-console
- Configuring VS Code / CodeLLDB for Rust projects
- Managing debug symbols for release binaries
- Diagnosing async task starvation or slow polls

## Rule Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Debugger Setup | HIGH | `dbg-` |
| 2 | Panic Analysis | HIGH | `panic-` |
| 3 | Runtime Introspection | HIGH | `rt-` |
| 4 | Core Dump Pipeline | HIGH | `core-` |
| 5 | Debug Symbols & Tools | MEDIUM | `sym-` |

## Quick Reference

### 1. Debugger Setup (HIGH)

- `dbg-breakpoints` - Break on panic, conditional breakpoints, trait method breakpoints
- `dbg-vscode-config` - CodeLLDB launch.json, source mapping, test binary debugging
- `dbg-optimized-builds` - "Value optimized out" workaround: registers, selective opt-level, disassembly
- `dbg-concurrency` - Deadlock diagnosis: thread inspection, mutex owner, per-thread breakpoints

### 2. Panic Analysis (HIGH)

- `panic-custom-hooks` - Structured panic hooks with tracing, backtraces, and abort

### 3. Runtime Introspection (HIGH)

- `rt-tracing-instrument` - #[instrument] spans, skip/level/fields, future instrumentation
- `rt-tokio-console` - Async runtime debugging with console-subscriber, diagnosing slow polls
- `rt-tsan-report-reading` - TSan report structure, addr2line mapping, false positive suppression

### 4. Core Dump Pipeline (HIGH)

- `core-collection-setup` - Enable core dumps: ulimit, core_pattern, systemd coredumpctl
- `core-non-interactive-triage` - Batch GDB analysis, CI scripts, debuginfod remote symbols

### 5. Debug Symbols & Tools (MEDIUM)

- `sym-separate-symbols` - Extract debug symbols, ship stripped binaries, rustfilt demangling
- `sym-debug-release` - debug = 1/2 in release profiles, split-debuginfo options

## How to Use

Read individual rule files for detailed explanations and code examples:

```
rules/dbg-breakpoints.md
rules/dbg-optimized-builds.md
rules/dbg-concurrency.md
rules/panic-custom-hooks.md
rules/rt-tokio-console.md
rules/rt-tsan-report-reading.md
rules/core-collection-setup.md
rules/core-non-interactive-triage.md
```

Each rule file contains:
- Brief explanation of why it matters
- Incorrect code example with explanation
- Correct code example with explanation

## Resources

### ctx7 CLI

| Library | ctx7 ID | Use For |
|---------|---------|---------|
| Rust std | `/websites/doc_rust-lang_stable_std` | Standard library API, std::backtrace |
| tokio | `/websites/rs_tokio` | Async runtime, tokio-console |
| tracing | `/websites/rs_tracing` | Instrumentation, spans, subscribers |

## Full Compiled Document

For the complete guide with all rules expanded: `AGENTS.md`
