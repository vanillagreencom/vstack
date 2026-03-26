---
title: Core Isolation Verification
impact: HIGH
impactDescription: Unverified isolation allows OS tasks to preempt latency-critical threads
tags: core-isolation, isolcpus, taskset, scheduling
---

## Core Isolation Verification

**Impact: HIGH (unverified isolation allows OS tasks to preempt latency-critical threads)**

After configuring core isolation, verify that isolated cores only run your application threads. OS housekeeping on isolated cores defeats the purpose of isolation.

```bash
# Verify isolated cores (kernel parameter: isolcpus=0-3)
cat /sys/devices/system/cpu/isolated

# Check process CPU affinity
taskset -cp $(pidof my_app)
# Should show pinned to isolated cores

# Verify no other processes on isolated cores
ps -eo pid,comm,psr | awk '$3 ~ /^[0-3]$/'
# Should only show your application processes
```
