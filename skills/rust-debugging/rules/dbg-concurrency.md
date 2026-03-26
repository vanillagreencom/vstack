---
title: Concurrency Debugging
impact: HIGH
tags: debugger, concurrency, deadlock, threads, gdb, mutex
---

## Concurrency Debugging

**Impact: HIGH (deadlocks and races are invisible without thread-aware debugging)**

Diagnosing deadlocks and thread contention requires GDB's thread inspection commands. Threads stuck in `__lll_lock_wait` indicate futex contention. Dump all backtraces at once to find lock ordering issues, and use per-thread conditional breakpoints to isolate specific threads.

**Incorrect (debugging concurrency issues with println):**

```rust
// Scattering println! across threads — output interleaves randomly,
// timing changes mask the deadlock, and you can't inspect lock state
fn worker(mutex_a: &Mutex<Data>, mutex_b: &Mutex<Data>) {
    println!("thread {:?} acquiring A", std::thread::current().id());
    let _a = mutex_a.lock().unwrap();
    println!("thread {:?} acquired A, acquiring B", std::thread::current().id());
    let _b = mutex_b.lock().unwrap(); // Deadlocks here — but println never fires
}
```

**Correct (using GDB thread inspection for deadlock diagnosis):**

```rust
// Step 1: List all threads and their states
// (gdb) info threads
// Look for threads stuck in:
//   __lll_lock_wait       — futex contention (Linux)
//   __GI___pthread_mutex_lock — waiting to acquire mutex
//   pthread_cond_wait     — waiting on condition variable

// Step 2: Dump all thread backtraces at once
// (gdb) thread apply all bt
// Scan output for lock acquisition patterns — if thread 1 holds
// mutex A waiting for B, and thread 2 holds B waiting for A: deadlock

// Step 3: Inspect mutex owner to find who holds the lock
// (gdb) p ((pthread_mutex_t*)0x7fff1234)->__data.__owner
// Returns the TID of the thread holding the mutex

// Step 4: Set per-thread conditional breakpoints
// (gdb) break file.rs:42 thread 3
// Only thread 3 will stop at this breakpoint

// Step 5: Attach to a running process for live diagnosis
// $ gdb -p $(pgrep myapp)
// Useful when the deadlock only reproduces in production
```

**Key GDB commands for concurrency debugging:**

| Command | Purpose |
|---------|---------|
| `info threads` | List all threads with current location |
| `thread apply all bt` | Dump all thread backtraces |
| `thread apply all bt full` | Backtraces with local variables |
| `thread N` | Switch to thread N |
| `p ((pthread_mutex_t*)addr)->__data.__owner` | Find mutex owner TID |
| `break file.rs:42 thread 3` | Per-thread conditional breakpoint |
| `gdb -p $(pgrep myapp)` | Attach to running process |
| `set scheduler-locking on` | Freeze other threads while stepping |
