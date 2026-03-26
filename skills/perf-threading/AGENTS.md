# Perf Threading

**Version 1.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when designing
> threading architecture, pinning threads, selecting channels, or
> eliminating jitter in Rust low-latency systems. Humans may also find
> it useful, but guidance here is optimized for automation and
> consistency by AI-assisted workflows.

---

## Abstract

Topology-aware thread pinning, core isolation, SPSC patterns, and page fault prevention for Rust low-latency systems. Prioritized from critical (CPU topology — wrong placement adds 2-3x latency) to high (core isolation, SPSC selection, page fault elimination).

---

## Table of Contents

1. [CPU Topology](#1-cpu-topology) — **CRITICAL**
   - 1.1 [CPU Topology Discovery](#11-cpu-topology-discovery)
   - 1.2 [Thread Pinning](#12-thread-pinning)
2. [Core Isolation](#2-core-isolation) — **HIGH**
   - 2.1 [Kernel Core Isolation Parameters](#21-kernel-core-isolation-parameters)
   - 2.2 [IRQ Affinity Steering](#22-irq-affinity-steering)
3. [SPSC Patterns](#3-spsc-patterns) — **HIGH**
   - 3.1 [SPSC Channel Selection](#31-spsc-channel-selection)
   - 3.2 [SPSC Latency Measurement](#32-spsc-latency-measurement)
4. [Page Fault Prevention](#4-page-fault-prevention) — **HIGH**
   - 4.1 [mlockall for Page Fault Prevention](#41-mlockall-for-page-fault-prevention)
   - 4.2 [Stack Pre-faulting](#42-stack-pre-faulting)

---

## 1. CPU Topology

**Impact: CRITICAL**

CPU topology discovery and thread pinning — CCD/L3 sharing groups, hyperthreading siblings, NUMA awareness. Incorrect placement adds 2-3x latency on cross-CCD communication.

### 1.1 CPU Topology Discovery

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

### 1.2 Thread Pinning

**Impact: CRITICAL (unpinned threads migrate between cores corrupting cache state)**

Pin with `libc::pthread_setaffinity_np`: get thread handle via `libc::pthread_self()`, create `cpu_set_t` with `CPU_ZERO`/`CPU_SET`, call `pthread_setaffinity_np(handle, size_of::<cpu_set_t>(), &set)`. Always verify: read back with `sched_getaffinity` and check `/proc/<pid>/status` `Cpus_allowed` field. Pin at thread start, not after work begins — migration during warmup corrupts cache state.

**Incorrect (pinning after work begins, no verification):**

```rust
fn worker(core: usize) {
    do_warmup(); // Thread may migrate during warmup — cache state lost
    core_affinity::set_for_current(CoreId { id: core });
    // No verification that pinning succeeded
    do_work();
}
```

**Correct (pin at thread start with verification):**

```rust
fn worker(core: usize) {
    // Pin immediately at thread start
    unsafe {
        let mut set: libc::cpu_set_t = std::mem::zeroed();
        libc::CPU_ZERO(&mut set);
        libc::CPU_SET(core, &mut set);
        let handle = libc::pthread_self();
        // SAFETY: handle is valid for current thread, set is properly initialized
        let ret = libc::pthread_setaffinity_np(
            handle,
            std::mem::size_of::<libc::cpu_set_t>(),
            &set,
        );
        assert_eq!(ret, 0, "failed to pin thread to core {core}");

        // Verify readback
        let mut readback: libc::cpu_set_t = std::mem::zeroed();
        libc::sched_getaffinity(0, std::mem::size_of::<libc::cpu_set_t>(), &mut readback);
        assert!(libc::CPU_ISSET(core, &readback), "core {core} not in affinity mask");
    }

    do_warmup(); // Now warmup runs on the correct core
    do_work();
}
```

For the `core_affinity` crate (simpler API, less control):

```rust
core_affinity::set_for_current(CoreId { id: 4 });
```

---

## 2. Core Isolation

**Impact: HIGH**

Kernel-level core isolation, timer tick suppression, IRQ affinity steering. Without isolation, kernel housekeeping and interrupts inject microsecond-scale jitter into latency-critical threads.

### 2.1 Kernel Core Isolation Parameters

**Impact: HIGH (scheduler noise injects microsecond jitter on shared cores)**

`isolcpus=4,5,6,7` removes cores from the general scheduler — only explicitly pinned threads run there. Combine with `nohz_full=4,5,6,7` (disable timer ticks) and `rcu_nocbs=4,5,6,7` (offload RCU callbacks). For maximum isolation: `taskset -c 4 ./my_app` + `chrt -f 99` for FIFO real-time priority.

Verify isolation:
- `cat /sys/devices/system/cpu/isolated` — confirms isolated core list
- `ps -eo pid,comm,psr | awk '$3 == 4'` — confirms no unwanted processes on core 4

**Incorrect (running latency-critical thread on non-isolated core):**

```bash
# No kernel isolation — scheduler places housekeeping threads on same cores
./my_app  # Thread may share core with kworker, rcu, ksoftirqd
```

**Correct (full isolation stack):**

```bash
# Kernel boot params (grub/systemd-boot):
# isolcpus=4,5,6,7 nohz_full=4,5,6,7 rcu_nocbs=4,5,6,7

# Verify isolation
cat /sys/devices/system/cpu/isolated
# Output: 4-7

# Launch with core pinning + real-time priority
taskset -c 4 chrt -f 99 ./my_app

# Verify no unwanted processes on isolated core
ps -eo pid,comm,psr | awk '$3 == 4'
# Should show only your my_app process
```

### 2.2 IRQ Affinity Steering

**Impact: HIGH (hardware interrupts on trading cores inject 5-50us jitter)**

Move all IRQs away from isolated cores. Exception: the NIC IRQ for the trading feed should be on the same NUMA node as the trading thread but NOT on the same core.

**Incorrect (IRQs still routed to isolated cores):**

```bash
# Isolated cores 4-7 but IRQs not redirected
# Hardware interrupts still fire on core 4, injecting jitter
cat /proc/interrupts | grep eth0
#            CPU0  CPU1  CPU2  CPU3  CPU4  ...
# eth0:       102    55    37    28   312   <- IRQs hitting core 4
```

**Correct (steer all IRQs away, place NIC IRQ on same NUMA node):**

```bash
# Move ALL IRQs to housekeeping cores (0-3)
for i in /proc/irq/*/smp_affinity_list; do echo "0-3" > "$i"; done

# Set NIC to single queue for deterministic IRQ placement
ethtool -L eth0 combined 1

# Place NIC IRQ on same NUMA node as trading thread, different core
# Trading thread on core 4 (NUMA 0), NIC IRQ on core 2 (NUMA 0)
echo "2" > /proc/irq/<nic_irq>/smp_affinity_list

# Verify
cat /proc/interrupts | awk '{print $NF}' | sort -u
```

---

## 3. SPSC Patterns

**Impact: HIGH**

Single-producer single-consumer channel selection, topology-aware placement, and latency measurement. Wrong channel or cross-CCD placement adds 100-200ns per message on the hot path.

### 3.1 SPSC Channel Selection

**Impact: HIGH (wrong channel type adds 100-200ns per message on hot path)**

Decision table for channel selection:

| Channel | Latency | Pattern | Use Case |
|---------|---------|---------|----------|
| `rtrb` | ~50ns | Wait-free SPSC | Intra-process hot path (trading) |
| `ringbuf` | ~100ns | SPSC, more features | SPSC with need for custom blocking |
| `crossbeam::channel` | ~200ns | MPSC/MPMC | Backpressure, multiple producers |
| `tokio::sync::mpsc` | Higher | Async-aware | Async context only |

For trading hot path: always `rtrb`. For async-to-sync bridge: SPSC ring forward to tokio mpsc. Always use power-of-2 capacity for bitmask modulo (no division).

**Incorrect (using crossbeam for single-producer single-consumer hot path):**

```rust
// crossbeam MPMC channel on SPSC hot path — 4x slower than necessary
let (tx, rx) = crossbeam::channel::bounded(1024);
// Also: 1024 is power-of-2 (good), but crossbeam doesn't optimize for it
```

**Correct (rtrb for SPSC hot path with power-of-2 capacity):**

```rust
// rtrb: wait-free SPSC, cache-padded indices, ~50ns
let (mut producer, mut consumer) = rtrb::RingBuffer::new(1024); // power-of-2

// Producer (pinned to core 0)
producer.push(message).expect("ring full");

// Consumer (pinned to core 1, same CCD as producer)
if let Ok(msg) = consumer.pop() {
    process(msg);
}
```

### 3.2 SPSC Latency Measurement

**Impact: HIGH (Criterion can't measure two-thread latency — wrong tool gives wrong numbers)**

Criterion is single-threaded and cannot measure SPSC channel latency. Use a custom harness: spawn producer + consumer, pin to specific cores, producer writes timestamps, consumer records latency via `HdrHistogram`, run for fixed duration, report P50/P99/P99.9. Measure both intra-CCD and cross-CCD to quantify topology impact. Rate-limit producer to realistic message rate (e.g., 1M msg/s) — flood mode hides queueing effects.

**Incorrect (using Criterion for SPSC latency):**

```rust
// Criterion runs single-threaded — this measures push() alone, not end-to-end
fn bench_spsc(c: &mut Criterion) {
    let (mut prod, mut cons) = rtrb::RingBuffer::new(1024);
    c.bench_function("spsc", |b| {
        b.iter(|| {
            prod.push(42).unwrap();
            cons.pop().unwrap(); // Same thread — no contention, no cache transfer
        });
    });
}
```

**Correct (custom two-thread harness with histogram):**

```rust
use hdrhistogram::Histogram;
use std::time::{Duration, Instant};

fn measure_spsc_latency(producer_core: usize, consumer_core: usize) {
    let (mut prod, mut cons) = rtrb::RingBuffer::<Instant>::new(1024);
    let duration = Duration::from_secs(5);
    let msg_interval = Duration::from_micros(1); // 1M msg/s

    let consumer = std::thread::spawn(move || {
        pin_to_core(consumer_core);
        let mut hist = Histogram::<u64>::new(3).unwrap();
        while let Ok(sent_at) = cons.pop() {
            let latency_ns = sent_at.elapsed().as_nanos() as u64;
            hist.record(latency_ns).ok();
        }
        hist
    });

    pin_to_core(producer_core);
    let start = Instant::now();
    while start.elapsed() < duration {
        let _ = prod.push(Instant::now());
        std::thread::sleep(msg_interval); // Rate-limit to realistic rate
    }
    drop(prod); // Signal consumer to exit

    let hist = consumer.join().unwrap();
    println!("P50={}, P99={}, P99.9={}",
        hist.value_at_quantile(0.50),
        hist.value_at_quantile(0.99),
        hist.value_at_quantile(0.999));
}
```

---

## 4. Page Fault Prevention

**Impact: HIGH**

Memory locking, stack pre-faulting, and steady-state page fault elimination. A single minor page fault costs 1-5us — unacceptable on latency-critical paths.

### 4.1 mlockall for Page Fault Prevention

**Impact: HIGH (minor page faults cost 1-5us each during steady-state)**

`libc::mlockall(libc::MCL_CURRENT | libc::MCL_FUTURE)` at process startup prevents minor page faults during steady-state operation. Pre-fault all buffers by touching every page after allocation. Verify zero faults in steady state with `perf stat`. Downside: RSS grows to full virtual size — only use for latency-critical processes.

**Incorrect (no memory locking — page faults during trading):**

```rust
fn main() {
    let mut buffer = Vec::with_capacity(1_000_000);
    // Pages not yet mapped — first write to each page triggers minor fault (~1-5us)
    // During trading, these faults appear as latency spikes
    run_my_app(&mut buffer);
}
```

**Correct (mlockall + pre-fault all buffers at startup):**

```rust
fn main() {
    // Lock all current and future pages in memory
    unsafe {
        // SAFETY: mlockall is safe to call, only affects memory residency
        let ret = libc::mlockall(libc::MCL_CURRENT | libc::MCL_FUTURE);
        assert_eq!(ret, 0, "mlockall failed — check RLIMIT_MEMLOCK");
    }

    // Pre-fault buffer by touching every page
    let mut buffer = vec![0u8; 1_000_000]; // vec! with value touches all pages
    // For Vec::with_capacity, explicitly fill:
    // buffer.resize(capacity, 0);

    // Verify zero faults in steady state:
    // perf stat -e page-faults -p $(pidof app) sleep 10
    run_my_app(&mut buffer);
}
```

### 4.2 Stack Pre-faulting

**Impact: HIGH (lazy stack allocation causes page faults on first deep call)**

Default thread stack is 8MB but lazily allocated. First deep call stack triggers page faults. Pre-fault by writing to each page at thread start, or set a smaller stack size if 8MB is not needed.

**Incorrect (relying on default lazy stack allocation):**

```rust
std::thread::spawn(|| {
    // First deep recursion or large stack frame triggers page faults
    // Each faulted page adds ~1-5us latency
    deep_processing_function(); // Worst-case latency spike on first call
});
```

**Correct (pre-fault stack at thread start):**

```rust
fn prefault_stack() {
    // Touch every page in an 8MB stack allocation
    let mut buf = [0u8; 8 * 1024 * 1024];
    // SAFETY: black_box prevents compiler from optimizing away the write
    std::hint::black_box(&mut buf);
}

std::thread::spawn(|| {
    prefault_stack(); // All stack pages now resident
    deep_processing_function(); // No page faults
});

// Alternative: reduce stack size if 8MB not needed
std::thread::Builder::new()
    .stack_size(1024 * 1024) // 1MB — fewer pages to fault
    .spawn(|| {
        prefault_stack_1mb();
        do_work();
    })
    .unwrap();
```
