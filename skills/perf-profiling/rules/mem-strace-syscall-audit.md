---
title: Strace Syscall Audit for Hot Paths
impact: MEDIUM
impactDescription: Hidden syscalls in hot paths cause latency spikes invisible to CPU profilers
tags: strace, syscall, mmap, futex, latency, audit
---

## Strace Syscall Audit for Hot Paths

**Impact: MEDIUM (hidden syscalls in hot paths cause latency spikes invisible to CPU profilers)**

Use `strace -c` to audit syscalls in hot paths. Hidden syscalls from allocator growth, lock contention, or time queries add unpredictable latency invisible to CPU-level profilers.

```bash
# Syscall summary — shows count, time, and errors per syscall
strace -c ./target/release/my_app

# Memory syscalls only — find allocator growth
strace -e trace=memory -c ./target/release/my_app

# Filter to specific suspicious syscalls
strace -e trace=mmap,munmap,futex,clock_gettime -c ./target/release/my_app

# Attach to running process
strace -c -p $(pidof my_app)
```

**Syscalls that should be zero in hot paths:**

| Syscall | Indicates | Target |
|---------|-----------|--------|
| `mmap`/`munmap` | Allocator growth/shrink | Zero after startup |
| `futex` | Lock contention | Zero in lock-free paths |
| `clock_gettime` | Time queries | Batch or use TSC |
| `brk` | Heap expansion | Zero after warmup |
| `madvise` | THP faults | Zero in steady state |

**Combine with perf stat for full picture:**

```bash
# Syscall audit + hardware counters together
strace -c ./target/release/my_app 2> syscalls.txt &
perf stat -p $! -e cycles,instructions,cache-misses

# Or sequential: first syscalls, then counters
strace -c -o syscall_report.txt ./target/release/my_app
perf stat -e instructions,cycles,L1-dcache-load-misses ./target/release/my_app
```

**Warning:** `strace` adds significant overhead (ptrace-based). Use only for diagnosis, not benchmarking. For production syscall auditing, use eBPF-based `syscount` from bcc-tools.
