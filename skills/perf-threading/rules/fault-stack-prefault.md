---
title: Stack Pre-faulting
impact: HIGH
impactDescription: Lazy stack allocation causes page faults on first deep call
tags: stack, prefault, page_fault, thread, latency
---

## Stack Pre-faulting

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
