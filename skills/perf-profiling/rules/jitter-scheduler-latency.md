---
title: Scheduler Latency Analysis
impact: HIGH
impactDescription: High tail latency with low P50 indicates OS scheduler interference
tags: scheduler, jitter, perf-sched, latency, RT
---

## Scheduler Latency Analysis

**Impact: HIGH (high tail latency with low P50 indicates OS scheduler interference)**

When P99.9 is much higher than P50, the problem is often OS scheduler interference rather than application code. Use `perf sched` to measure scheduling latency directly.

```bash
# Measure scheduler hiccups
perf sched latency -p $(pidof my_app)

# Record scheduling events for detailed analysis
perf sched record -p $(pidof my_app) -- sleep 10
perf sched latency

# Visual CPU usage timeline
perf sched map

# Set real-time scheduling (requires root or CAP_SYS_NICE)
sudo chrt -f 99 ./my_app

# Verify scheduling policy
chrt -p $(pidof my_app)
# Should show: SCHED_FIFO or SCHED_RR with appropriate priority
```

Combine with core isolation and IRQ affinity for lowest achievable jitter.
