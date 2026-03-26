---
title: Off-CPU Analysis for Blocking
impact: HIGH
impactDescription: Latency spikes from blocking invisible to on-CPU profilers
tags: off-cpu, blocking, scheduling, io
---

## Off-CPU Analysis for Blocking

**Impact: HIGH (latency spikes from blocking invisible to on-CPU profilers)**

When latency is high but CPU utilization is low, the problem is off-CPU: threads blocked on I/O, locks, or scheduling. Standard flamegraphs only show on-CPU time and will miss these entirely.

```bash
# Find where threads are blocked/waiting
sudo offcputime -f -p $(pidof my_app) 30 | flamegraph.pl --color=io > offcpu.svg

# Kernel stack traces for blocking analysis
sudo offcputime -K -p $(pidof my_app) 30 > offcpu_kernel.txt
```

**Use when:** Latency spikes occur but CPU is not saturated (blocking I/O, lock contention, scheduling delays).
