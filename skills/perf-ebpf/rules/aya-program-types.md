---
title: Program Type Selection
impact: HIGH
impactDescription: wrong program type = missed events or unnecessary overhead
tags: aya, tracepoint, kprobe, uprobe, xdp, perf_event
---

## Program Type Selection

**Impact: HIGH (wrong program type = missed events or unnecessary overhead)**

Program type selection: `#[tracepoint]` for syscall entry/exit (stable, low overhead), `#[kprobe]`/`#[kretprobe]` for any kernel function (flexible, slightly higher overhead), `#[uprobe]` for userspace function tracing without recompilation, `#[xdp]` for packet processing before kernel stack (lowest latency), `#[perf_event]` for hardware counter sampling.

Decision: tracepoints for syscall monitoring, uprobes for application tracing, XDP for network path.

**Incorrect (kprobe for stable syscall tracing):**

```rust
// kprobe on __x64_sys_read — breaks across kernel versions
#[kprobe]
pub fn trace_read(ctx: ProbeContext) -> u32 {
    // Kernel function names change between versions
    0
}
```

**Correct (tracepoint for syscall tracing, kprobe for internal functions):**

```rust
// Tracepoint: stable ABI, survives kernel upgrades
#[tracepoint]
pub fn trace_read(ctx: TracePointContext) -> u32 {
    // tracepoint:syscalls:sys_enter_read — stable interface
    0
}

// Kprobe: only when tracing internal kernel functions not exposed as tracepoints
#[kprobe]
pub fn trace_tcp_internal(ctx: ProbeContext) -> u32 {
    // kprobe:tcp_rcv_established — no tracepoint equivalent
    0
}

// XDP: packet processing before kernel stack (lowest latency path)
#[xdp]
pub fn filter_packets(ctx: XdpContext) -> u32 {
    // Runs before sk_buff allocation — minimal overhead
    xdp_action::XDP_PASS
}
```
