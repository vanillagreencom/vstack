---
title: mlockall for Page Fault Prevention
impact: HIGH
impactDescription: First-touch page faults add 1-5us per page on hot paths
tags: mlockall, page-fault, latency, memory, prefault
---

## mlockall for Page Fault Prevention

**Impact: HIGH (first-touch page faults add 1-5us per page on hot paths)**

`libc::mlockall(libc::MCL_CURRENT | libc::MCL_FUTURE)` at startup prevents page faults on hot paths. Every first access to a new page causes a minor fault (~1-5us). Pre-fault all buffers at startup by writing to every page, then mlockall to keep them resident. Verify: `perf stat -e page-faults ./prog` should show 0 faults during steady state. Downside: increases RSS — all mapped memory stays resident.

**Incorrect (lazy page allocation — first-touch faults during trading):**

```rust
fn main() {
    let ring_buffer = vec![0u8; 4 * 1024 * 1024]; // pages not yet mapped
    // ... start trading loop
    // First write to each page: minor fault, 1-5us stall
    ring_buffer[0] = 1;       // FAULT
    ring_buffer[4096] = 1;    // FAULT
    ring_buffer[8192] = 1;    // FAULT — 1,024 faults for 4MB
}
```

**Correct (pre-fault at startup, lock all pages):**

```rust
fn main() {
    // Lock all current and future pages
    unsafe {
        libc::mlockall(libc::MCL_CURRENT | libc::MCL_FUTURE);
    }

    // Allocate and pre-fault every page
    let mut ring_buffer = vec![0u8; 4 * 1024 * 1024];
    for page in ring_buffer.chunks_mut(4096) {
        page[0] = 0; // touch every page to force allocation
    }

    // Verify: perf stat -e page-faults ./prog
    // Should show 0 faults after startup phase

    // ... start trading loop — no page faults possible
}
```
