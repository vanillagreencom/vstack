---
title: Map Patterns
impact: HIGH
impactDescription: wrong map type causes data loss or ordering bugs
tags: aya, ringbuf, perf_event_array, hashmap, per_cpu_array, maps
---

## Map Patterns

**Impact: HIGH (wrong map type causes data loss or ordering bugs)**

Use `RingBuf` (kernel >=5.8) over `PerfEventArray` for events — lower overhead, preserved cross-CPU ordering, variable-size records. Pattern: kernel reserves with `EVENTS.reserve::<MyEvent>(0)`, writes, submits. Userspace reads via `AsyncFd` for async polling. Use `HashMap` for per-PID/per-connection state. Use `PerCpuArray` for lock-free per-CPU counters (aggregate in userspace).

**Incorrect (PerfEventArray when RingBuf available):**

```rust
// Kernel-side: PerfEventArray has per-CPU buffers, no cross-CPU ordering
#[map]
static EVENTS: PerfEventArray<MyEvent> = PerfEventArray::with_max_entries(1024, 0);

// Loses ordering when events fire on different CPUs
EVENTS.output(&ctx, &event, 0);
```

**Correct (RingBuf for events, HashMap for state, PerCpuArray for counters):**

```rust
// Kernel-side: RingBuf preserves cross-CPU ordering
#[map]
static EVENTS: RingBuf = RingBuf::with_byte_size(256 * 1024, 0);

#[map]
static CONN_STATE: HashMap<u32, ConnInfo> = HashMap::with_max_entries(10240, 0);

#[map]
static COUNTERS: PerCpuArray<u64> = PerCpuArray::with_max_entries(8, 0);

// Submit event via RingBuf — reserve, write, submit
if let Some(mut entry) = EVENTS.reserve::<MyEvent>(0) {
    entry.write(MyEvent { pid, latency_ns });
    entry.submit(0);
}

// Per-PID state lookup
if let Some(info) = unsafe { CONN_STATE.get(&pid) } {
    // use connection state
}

// Lock-free per-CPU counter increment
if let Some(counter) = unsafe { COUNTERS.get_ptr_mut(0) } {
    unsafe { *counter += 1 };
}
```
