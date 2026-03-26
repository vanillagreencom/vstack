---
name: perf-ebpf
description: eBPF/Aya kernel-level observability in Rust. Covers Aya framework project setup, program types, map patterns, bpftrace diagnostics, verifier debugging, and program inspection. Use when writing eBPF programs, diagnosing kernel-level performance, or tracing application behavior.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# eBPF/Aya Observability

Kernel-level observability with eBPF and the Aya framework in Rust, prioritized by impact.

## When to Apply

Reference these guidelines when:
- Creating new eBPF programs with the Aya framework
- Choosing between program types (tracepoint, kprobe, uprobe, XDP)
- Selecting map types for kernel-to-userspace communication
- Running bpftrace one-liners for production diagnostics
- Debugging BPF verifier errors or inspecting loaded programs
- Profiling network paths or syscall latency

## Rule Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Aya Framework | HIGH | `aya-` |
| 2 | bpftrace Diagnostics | HIGH | `trace-` |
| 3 | Verifier & Debugging | MEDIUM | `debug-` |

## Quick Reference

### 1. Aya Framework (HIGH)

- `aya-project-setup` - Workspace layout: ebpf crate (no_std/no_main, bpf target), userspace crate (tokio), xtask build helper
- `aya-program-types` - Program type selection: tracepoint, kprobe, uprobe, XDP, perf_event — match to use case
- `aya-map-patterns` - RingBuf over PerfEventArray, HashMap for per-entity state, PerCpuArray for counters

### 2. bpftrace Diagnostics (HIGH)

- `trace-one-liners` - Production-safe one-liners: syscall counts, latency histograms, scheduling analysis
- `trace-network-diagnosis` - TCP retransmits, receive buffer sizing, XDP for multicast filtering

### 3. Verifier & Debugging (MEDIUM)

- `debug-verifier-errors` - Common verifier error to fix table: null checks, bounded loops, vmlinux bindings, capabilities
- `debug-inspecting-programs` - bpftool commands: list programs, dump bytecode, show maps, dump map contents

## How to Use

Read individual rule files for detailed explanations and code examples:

```
rules/aya-project-setup.md
rules/aya-map-patterns.md
rules/trace-one-liners.md
```

Each rule file contains:
- Brief explanation of why it matters
- Incorrect code example with explanation
- Correct code example with explanation

## Resources

### ctx7 CLI

| Library | ctx7 ID | Use For |
|---------|---------|---------|
| aya | `/aya-rs/book` | Aya eBPF framework |
| aya-bpf | `/aya-rs/aya` | Kernel-side eBPF helpers |
| bpftrace | `/bpftrace/bpftrace` | bpftrace reference |

## Full Compiled Document

For the complete guide with all rules expanded: `AGENTS.md`
