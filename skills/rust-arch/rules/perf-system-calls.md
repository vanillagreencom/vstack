---
title: System Calls in Hot Path
impact: CRITICAL
impactDescription: +100ns-10us per call
tags: performance, hot-path, syscall, io
---

## System Calls in Hot Path

**Impact: CRITICAL (+100ns-10us per call)**

File I/O, time syscalls (`SystemTime::now()`), and other kernel transitions add unpredictable latency from context switches and kernel scheduling.

**Detection:** File operations, system time calls, network I/O in latency-sensitive paths.

**Fix:** Batch operations, cache results (e.g., read time once per batch), and move I/O to background threads.
