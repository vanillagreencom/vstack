---
title: eBPF for Production Profiling
impact: HIGH
impactDescription: perf overhead distorts measurements in production systems
tags: ebpf, bcc, production, low-overhead
---

## eBPF for Production Profiling

**Impact: HIGH (perf overhead distorts measurements in production systems)**

Use eBPF-based profiling (bcc tools) for production or long-running profiles. eBPF does in-kernel aggregation, producing significantly lower overhead than `perf record`.

```bash
# bcc's profile tool - in-kernel stack aggregation
sudo profile -F 99 -p $(pidof my_app) 30 > profile.txt

# Direct to flamegraph
sudo profile -F 99 -f -p $(pidof my_app) 30 | flamegraph.pl > ebpf_flame.svg

# System-wide profiling
sudo profile -F 99 -a 30 | flamegraph.pl > system_flame.svg
```

**When to use eBPF over perf:**
- Production systems where overhead matters
- Long-running profiles (minutes to hours)
- When perf's overhead itself affects measurements
- Kernel 4.6+ with BPF stack trace support required
