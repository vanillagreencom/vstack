---
title: CPU Topology Discovery
impact: CRITICAL
impactDescription: Cross-CCD thread placement adds 2-3x latency
tags: topology, ccd, numa, l3, sysfs, hyperthreading
---

## CPU Topology Discovery

**Impact: CRITICAL (cross-CCD thread placement adds 2-3x latency)**

Read CPU topology from sysfs before pinning any threads:

- `/sys/devices/system/cpu/cpu*/cache/index3/shared_cpu_list` — L3 sharing groups (CCD on AMD)
- `/sys/devices/system/cpu/cpu*/topology/core_id` — physical core ID
- `/sys/devices/system/cpu/cpu*/topology/thread_siblings_list` — hyperthreading pairs

Intra-CCD latency is 2-3x lower than cross-CCD on AMD Zen. Pin communicating threads (e.g., producer/consumer SPSC) to the same CCD. Avoid hyperthreading siblings for latency-critical threads — they share execution resources.

**Incorrect (ignoring topology — communicating threads land on different CCDs):**

```rust
// Pins producer to core 0, consumer to core 8 — may be different CCDs
// Cross-CCD adds ~40ns per cache line transfer
let producer_core = 0;
let consumer_core = 8;
```

**Correct (read topology, pin communicating threads to same CCD):**

```rust
// Read L3 sharing groups from sysfs
// /sys/devices/system/cpu/cpu0/cache/index3/shared_cpu_list => "0-3,8-11"
// This means cores 0-3 and 8-11 share an L3 (same CCD)
let ccd0_cores = parse_shared_cpu_list("/sys/devices/system/cpu/cpu0/cache/index3/shared_cpu_list");

// Pick two physical cores (not HT siblings) from the same CCD
// thread_siblings_list for core 0 => "0,8" — avoid both, use 0 and 1
let producer_core = ccd0_cores[0]; // e.g., 0
let consumer_core = ccd0_cores[1]; // e.g., 1 — same CCD, different physical core
```
