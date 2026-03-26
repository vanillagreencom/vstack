---
title: Thread Pinning
impact: CRITICAL
impactDescription: Unpinned threads migrate between cores corrupting cache state
tags: affinity, pinning, pthread, core_affinity, sched_getaffinity
---

## Thread Pinning

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
