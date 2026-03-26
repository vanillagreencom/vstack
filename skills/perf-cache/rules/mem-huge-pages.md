---
title: Huge Pages for TLB Miss Reduction
impact: HIGH
impactDescription: TLB misses on large allocations add 10-100ns per access
tags: huge-pages, thp, tlb, madvise, memory
---

## Huge Pages for TLB Miss Reduction

**Impact: HIGH (TLB misses on large allocations add 10-100ns per access)**

Transparent Huge Pages (THP) use 2MB pages instead of 4KB, reducing TLB misses by up to 512x for large allocations. Check status: `cat /sys/kernel/mm/transparent_hugepage/enabled`. Use `madvise(MADV_HUGEPAGE)` on large allocations (>2MB) for up to 4.5x improvement on random access. For deterministic latency, prefer explicit huge pages over THP (THP can cause compaction stalls).

**Incorrect (large allocation with 4KB pages — TLB thrashing on random access):**

```rust
// 64MB buffer with 4KB pages = 16,384 TLB entries needed
// Most CPUs have ~1,500 dTLB entries — constant TLB misses
let buffer: Vec<u8> = vec![0u8; 64 * 1024 * 1024];
```

**Correct (advise kernel to use huge pages):**

```rust
let buffer: Vec<u8> = vec![0u8; 64 * 1024 * 1024];
unsafe {
    libc::madvise(
        buffer.as_ptr() as *mut libc::c_void,
        buffer.len(),
        libc::MADV_HUGEPAGE,
    );
}
// 64MB / 2MB = 32 TLB entries instead of 16,384

// Verify huge pages are active:
// grep -i huge /proc/$(pidof app)/smaps
```
