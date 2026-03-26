---
title: Production-Safe One-Liners
impact: HIGH
impactDescription: ad-hoc diagnostics without recompilation or restart
tags: bpftrace, one-liner, syscall, latency, scheduling
---

## Production-Safe One-Liners

**Impact: HIGH (ad-hoc diagnostics without recompilation or restart)**

Production-safe diagnostic one-liners that require no recompilation and add negligible overhead.

- **Syscall count by process**: `bpftrace -e 'tracepoint:raw_syscalls:sys_enter { @[comm] = count(); }'`
- **Read latency histogram for specific process**: `bpftrace -e 'tracepoint:syscalls:sys_enter_read /comm == "myapp"/ { @start[tid] = nsecs; } tracepoint:syscalls:sys_exit_read /comm == "myapp"/ { @us = hist((nsecs - @start[tid]) / 1000); delete(@start[tid]); }'`
- **Why threads are getting descheduled**: `bpftrace -e 'tracepoint:sched:sched_switch { @[kstack] = count(); }'`

**Incorrect (strace for production diagnostics):**

```bash
# strace attaches via ptrace — stops process on every syscall
# 10-100x overhead, unsafe for production
strace -p $(pidof myapp) -c
```

**Correct (bpftrace with negligible overhead):**

```bash
# Syscall count by process — runs in-kernel, no ptrace
bpftrace -e 'tracepoint:raw_syscalls:sys_enter { @[comm] = count(); }'

# Read latency histogram filtered to specific process
bpftrace -e '
tracepoint:syscalls:sys_enter_read /comm == "myapp"/ {
    @start[tid] = nsecs;
}
tracepoint:syscalls:sys_exit_read /comm == "myapp"/ {
    @us = hist((nsecs - @start[tid]) / 1000);
    delete(@start[tid]);
}'

# Scheduling analysis — why threads get descheduled
bpftrace -e 'tracepoint:sched:sched_switch { @[kstack] = count(); }'
```
