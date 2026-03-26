---
title: mlockall for Page Fault Prevention
impact: HIGH
impactDescription: Minor page faults cost 1-5us each during steady-state
tags: mlockall, page_fault, memory, latency, mlock
---

## mlockall for Page Fault Prevention

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
