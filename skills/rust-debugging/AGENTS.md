# Rust Debugging

**Version 1.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when debugging,
> profiling, or diagnosing Rust programs. Humans may also find it useful,
> but guidance here is optimized for automation and consistency by
> AI-assisted workflows.

---

## Abstract

Debugger setup, panic analysis, runtime introspection, and debug symbol management for Rust programs, prioritized by impact from high (debugger setup, panic analysis, runtime introspection) to medium (debug symbols and tools).

---

## Table of Contents

1. [Debugger Setup](#1-debugger-setup) — **HIGH**
   - 1.1 [Breakpoint Strategies](#11-breakpoint-strategies)
   - 1.2 [VS Code CodeLLDB Configuration](#12-vs-code-codelldb-configuration)
   - 1.3 [Debugging Optimized Builds](#13-debugging-optimized-builds)
   - 1.4 [Concurrency Debugging](#14-concurrency-debugging)
2. [Panic Analysis](#2-panic-analysis) — **HIGH**
   - 2.1 [Custom Panic Hooks](#21-custom-panic-hooks)
3. [Runtime Introspection](#3-runtime-introspection) — **HIGH**
   - 3.1 [Tracing #\[instrument\] Spans](#31-tracing-instrument-spans)
   - 3.2 [tokio-console Async Debugging](#32-tokio-console-async-debugging)
   - 3.3 [TSan Report Reading](#33-tsan-report-reading)
4. [Core Dump Pipeline](#4-core-dump-pipeline) — **HIGH**
   - 4.1 [Core Dump Collection Setup](#41-core-dump-collection-setup)
   - 4.2 [Non-Interactive Core Dump Triage](#42-non-interactive-core-dump-triage)
5. [Debug Symbols & Tools](#5-debug-symbols--tools) — **MEDIUM**
   - 5.1 [Separate Debug Symbols](#51-separate-debug-symbols)
   - 5.2 [Debug Info in Release Builds](#52-debug-info-in-release-builds)

---

## 1. Debugger Setup

**Impact: HIGH**

GDB and LLDB configuration for Rust — pretty-printers, breakpoints, VS Code integration. Correct setup prevents wasted time reading raw struct internals and enables efficient crash diagnosis.

### 1.1 Breakpoint Strategies

**Impact: HIGH (inefficient debugging without targeted breakpoints)**

Set breakpoints strategically to catch panics, specific functions, and conditional states. Break on `rust_panic` to halt at the panic site before unwinding destroys the stack.

**GDB breakpoints:**

| Command | Purpose |
|---------|---------|
| `break rust_panic` | Halt on any panic |
| `break module::function` | Break on specific function |
| `break file.rs:42 if x > 100` | Conditional breakpoint |
| `break <Type as Trait>::method` | Break on trait method impl |
| `break test_module::test_name` | Break in specific test |

**LLDB equivalents:**

| Command | Purpose |
|---------|---------|
| `br s -n rust_panic` | Halt on any panic |
| `br s -n module::function` | Break on specific function |
| `br s -f file.rs -l 42 -c 'x > 100'` | Conditional breakpoint |

### 1.2 VS Code CodeLLDB Configuration

**Impact: HIGH (no IDE debugging without correct launch.json)**

The CodeLLDB extension provides Rust debugging in VS Code. Configure `launch.json` with cargo integration for automatic builds, source mapping for standard library stepping, and test binary support.

**Incorrect (missing cargo integration and source maps):**

```json
{
    "type": "lldb",
    "request": "launch",
    "program": "target/debug/myapp"
    // No automatic build — may debug stale binary
    // No source map — can't step into std library
}
```

**Correct (full CodeLLDB configuration):**

```json
{
    "type": "lldb",
    "request": "launch",
    "name": "Debug",
    "cargo": {
        "args": ["build"]
    },
    "sourceMap": {
        "/rustc/<hash>": "${env:HOME}/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust"
    }
}
```

Debug tests — build but don't run, then set program to the test binary path:

```json
{
    "type": "lldb",
    "request": "launch",
    "name": "Debug Tests",
    "cargo": {
        "args": ["test", "--no-run"]
    }
}
```

### 1.3 Debugging Optimized Builds

**Impact: HIGH (optimized builds hide variables and reorder code, misleading debugger output)**

Debugging optimized Rust code requires specific techniques because the compiler eliminates variables, inlines functions, and reorders instructions. "Value optimized out" is the most common obstacle — work around it by checking registers, reducing optimization for suspect crates, and using disassembly views.

**Incorrect (trusting debugger variable display in optimized builds):**

```rust
// With opt-level = 2 or 3, GDB often shows:
// (gdb) print result
// $1 = <optimized out>
//
// Developer assumes the value is gone — but it's in a register.
// Developer rebuilds entire project in debug mode, losing the
// optimized-only bug they were chasing.
```

**Correct (using registers, selective optimization, and disassembly):**

```rust
// Step 1: Check registers — optimized-out values often live there
// (gdb) info registers
// (gdb) info registers rax rdx    — check specific registers

// Step 2: Mark suspect functions to prevent inlining
#[inline(never)]
fn suspect_calculation(input: &[u8]) -> u64 {
    // This function will now appear as a real stack frame,
    // not an (inlined by) virtual frame
    input.iter().map(|&b| b as u64).sum()
}

// Step 3: Reduce optimization for a specific crate only
// In Cargo.toml:
// [profile.dev.package.suspect-crate]
// opt-level = 1
// [profile.release.package.suspect-crate]
// opt-level = 1

// Step 4: Use interleaved source+assembly (most reliable in optimized code)
// (gdb) disassemble /s function_name

// Step 5: Freeze other threads while single-stepping
// (gdb) set scheduler-locking on
// Prevents other threads from running, which avoids races
// corrupting the debug state during single-step

// Step 6: Continuous disassembly view
// (gdb) set disassemble-next-line on
// Shows disassembly for the next line at every stop
```

**Key GDB commands for optimized builds:**

| Command | Purpose |
|---------|---------|
| `info registers` | Check register values for optimized-out variables |
| `disassemble /s function_name` | Interleaved source + assembly |
| `set scheduler-locking on` | Freeze other threads while stepping |
| `set disassemble-next-line on` | Continuous disassembly view |
| `#[inline(never)]` | Prevent inlining of suspect functions |

**Note:** Inlined frames show as `(inlined by)` in backtraces — they are virtual, not real stack frames. You cannot set per-frame breakpoints on them directly.

### 1.4 Concurrency Debugging

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

---

## 2. Panic Analysis

**Impact: HIGH**

Panic triage, custom hooks, and backtrace configuration. Violations cause undiagnosed crashes, missing context in production logs, and slow incident response.

### 2.1 Custom Panic Hooks

**Impact: HIGH (panics in production vanish without structured logging)**

Use `std::panic::set_hook` for structured panic handling. Log thread name, backtrace, and panic location to tracing or error reporting before aborting. In production, always abort after logging to prevent undefined state.

**Incorrect (default panic handler with no structured logging):**

```rust
fn main() {
    // Default handler prints to stderr — lost in container logs,
    // no structured fields, no integration with error tracking
    run_server();
}
```

**Correct (custom panic hook with full context):**

```rust
fn main() {
    std::panic::set_hook(Box::new(|info| {
        let thread = std::thread::current();
        let thread_name = thread.name().unwrap_or("<unnamed>");
        let backtrace = std::backtrace::Backtrace::force_capture();

        eprintln!("PANIC in thread '{thread_name}': {info}");
        eprintln!("{backtrace}");

        // In production: log to tracing/sentry before abort
        // tracing::error!(%thread_name, %info, %backtrace, "panic");
    }));

    // Always abort in production — unwinding after panic leaves
    // undefined state
    // std::process::abort();

    run_server();
}
```

---

## 3. Runtime Introspection

**Impact: HIGH**

dbg!() macro, tracing instrumentation, and tokio-console for async debugging. Enables visibility into runtime behavior without resorting to println-driven debugging.

### 3.1 Tracing #[instrument] Spans

**Impact: HIGH (invisible async execution flow without structured spans)**

`#[instrument]` creates a span automatically on function entry, recording arguments as fields. Skip large or sensitive arguments, set appropriate levels, and attach spans to futures for async visibility.

**Incorrect (manual span creation with missing context):**

```rust
async fn process_order(order: Order, db: &Database) {
    // Manual span — verbose, easy to forget fields
    let span = tracing::info_span!("process_order");
    let _guard = span.enter();
    // Also wrong: .enter() doesn't work correctly in async —
    // the guard is held across await points
    let result = db.save(&order).await;
}
```

**Correct (using #[instrument] with appropriate options):**

```rust
// Auto-creates span with function name and args as fields
#[tracing::instrument(
    level = "info",
    skip(self, db),                        // Skip large/sensitive args
    fields(order_id = %order.id)           // Add custom fields
)]
async fn process_order(&self, order: Order, db: &Database) -> Result<()> {
    // Span automatically tracks the entire async execution
    let result = db.save(&order).await?;
    tracing::debug!("order saved successfully");
    Ok(result)
}

// For futures not tied to a function:
use tracing::Instrument;
let future = async_operation()
    .instrument(tracing::info_span!("background_task", task_id = %id));
tokio::spawn(future);
```

### 3.2 tokio-console Async Debugging

**Impact: HIGH (async bugs invisible without runtime introspection)**

tokio-console provides a top-like view of async tasks — poll durations, waker counts, and resource contention. Essential for diagnosing slow polls, task starvation, and waker storms that are invisible to traditional debuggers.

**Incorrect (diagnosing async issues with println):**

```rust
// Scattering println! to find slow tasks — no aggregate view,
// floods logs, can't see waker/poll relationships
async fn handle_request(req: Request) -> Response {
    println!("start handling request");  // Which task? When?
    let data = fetch_data().await;
    println!("data fetched");            // How long was the poll?
    process(data)
}
```

**Correct (using tokio-console for runtime visibility):**

```toml
# Cargo.toml
[dependencies]
console-subscriber = "0.4"
tokio = { version = "1", features = ["full", "tracing"] }
```

```rust
// main.rs — initialize console subscriber
fn main() {
    console_subscriber::init();  // Replaces default tracing subscriber

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async { run_server().await });
}

// Build with tokio_unstable cfg flag:
// RUSTFLAGS="--cfg tokio_unstable" cargo build

// Run tokio-console in another terminal:
// $ tokio-console
```

**What tokio-console shows:**

| Metric | Indicates |
|--------|-----------|
| Slow polls (>1ms) | Blocking work on async thread |
| Task starvation | Tasks waiting too long for poll |
| Waker storms | Excessive wake notifications |
| Resource contention | Mutex/semaphore bottlenecks |

### 3.3 TSan Report Reading

**Impact: HIGH (misreading TSan reports leads to ignoring real races or chasing false positives)**

ThreadSanitizer (TSan) reports have a specific structure: first section shows the racing write, second shows the conflicting access, third shows thread creation points. Understanding this structure is essential for diagnosing data races in `unsafe` code and FFI boundaries that Rust's type system cannot prevent.

**Incorrect (ignoring or misinterpreting TSan output):**

```rust
// Running with TSan and seeing output like:
// WARNING: ThreadSanitizer: data race
// Developer ignores it because "Rust prevents data races"
// — but TSan catches races in unsafe blocks and FFI that
// the borrow checker can't see

// Or: developer sees Arc<T> reference counting flagged
// and wastes time investigating a false positive
```

**Correct (systematic TSan report analysis):**

```rust
// TSan report structure:
//
// Section 1 — The racing access:
//   Write of size 8 at 0x7f1234 by thread T2:
//     #0 myapp::engine::update src/engine.rs:45
//     #1 myapp::engine::run    src/engine.rs:120
//
// Section 2 — The conflicting previous access:
//   Previous read of size 8 at 0x7f1234 by thread T1:
//     #0 myapp::engine::read_state src/engine.rs:30
//     #1 myapp::engine::poll       src/engine.rs:88
//
// Section 3 — Thread creation points:
//   Thread T1 (tid=12345, running) created by main thread at:
//     #0 std::thread::spawn ...
//   Thread T2 (tid=12346, running) created by main thread at:
//     #0 std::thread::spawn ...

// Map hex addresses to source lines:
// $ addr2line -e target/debug/myapp 0x55a1234

// Common Rust false positives (safe to suppress):
// - Arc reference counting: TSan doesn't understand atomic guarantees
// - lazy_static initialization: one-time race, harmless
// - std::sync::Once internals

// Suppress known false positives:
// Create tsan.supp:
//   race:std::sync::Arc
//   race:lazy_static
//
// Run with: TSAN_OPTIONS="suppressions=tsan.supp" ./target/debug/myapp

// Build with TSan enabled:
// RUSTFLAGS="-Z sanitizer=thread" cargo +nightly build
// RUSTFLAGS="-Z sanitizer=thread" cargo +nightly test
```

**TSan report cheat sheet:**

| Report Section | What It Shows |
|----------------|---------------|
| First access block | The racing write/read (file:line + thread) |
| Second access block | The conflicting previous access |
| Thread creation | Where each involved thread was spawned |
| `addr2line -e binary 0xaddr` | Map address to source file:line |

**Key principle:** TSan catches races in `unsafe` code and FFI boundaries that Rust's type system cannot prevent. Safe Rust code should never produce real TSan warnings — if it does, there is a compiler or standard library bug.

---

## 4. Core Dump Pipeline

**Impact: HIGH**

Core dump collection, configuration, and non-interactive triage. Ensures every crash produces a usable dump and that analysis can be automated in CI without interactive debugger sessions.

### 4.1 Core Dump Collection Setup

**Impact: HIGH (crashes without core dumps leave no evidence for post-mortem analysis)**

Enable and configure core dump collection so that every crash produces a usable dump file. Without this setup, crashes vanish — the default on many Linux systems is to discard cores.

**Incorrect (relying on system defaults for core dumps):**

```bash
# Default ulimit is often 0 — no core dumps generated
$ ulimit -c
0

# Crash happens, no core file written
$ ./target/release/myapp
Segmentation fault
# No core file — post-mortem analysis impossible
```

**Correct (enabling and configuring core dump collection):**

```bash
# Step 1: Enable core dumps for current session
ulimit -c unlimited

# Step 2: Configure core file naming pattern
# Tokens: %e=executable name, %p=pid, %t=timestamp
echo '/tmp/core-%e-%p-%t' > /proc/sys/kernel/core_pattern

# Step 3: For persistent configuration, add to /etc/security/limits.conf
# * soft core unlimited

# For systemd-managed systems (most modern Linux):
# List recent core dumps
coredumpctl list

# Show details for a specific dump
coredumpctl info

# Launch GDB directly on a dump
coredumpctl gdb

# Filter by executable
coredumpctl gdb myapp
```

**Core dump configuration summary:**

| Method | Scope | Command |
|--------|-------|---------|
| `ulimit -c unlimited` | Current shell session | Immediate, temporary |
| `/proc/sys/kernel/core_pattern` | System-wide pattern | Requires root, resets on reboot |
| `/etc/security/limits.conf` | Persistent, all users | Survives reboot |
| `coredumpctl` | systemd journal | Automatic storage and retrieval |

### 4.2 Non-Interactive Core Dump Triage

**Impact: HIGH (manual interactive debugging does not scale to CI or fleet-wide crash analysis)**

Use GDB batch mode for automated core dump analysis without interactive sessions. Extract backtraces, register state, and signal info in a single command. Ship stripped binaries with separate `.debug` files and use `debuginfod` for automatic remote symbol resolution.

**Incorrect (requiring interactive GDB for every crash):**

```bash
# Manual process — doesn't scale
gdb ./myapp core.12345
(gdb) bt
(gdb) info registers
(gdb) quit

# No automation, no CI integration, each crash requires
# a developer to sit at a terminal
```

**Correct (batch analysis and automated triage):**

```bash
# Step 1: One-shot batch analysis — all info in a single command
gdb -batch \
    -ex 'thread apply all bt full' \
    -ex 'info registers' \
    -ex 'print $_siginfo' \
    ./prog core.12345

# Step 2: CI script — capture output, extract key info
gdb -batch \
    -ex 'thread apply all bt full' \
    -ex 'info registers' \
    -ex 'print $_siginfo' \
    ./prog core.12345 2>&1 | tee crash-report.txt

# Grep for panic locations and signal info
grep -E '(rust_panic|SIGSEGV|SIGABRT|panicked)' crash-report.txt

# Step 3: Ship stripped binaries with separate debug files
objcopy --only-keep-debug target/release/app app.debug
strip --strip-debug target/release/app
objcopy --add-gnu-debuglink=app.debug target/release/app

# Step 4: Enable debuginfod for automatic remote symbol resolution
export DEBUGINFOD_URLS="https://debuginfod.ubuntu.com"
# GDB will automatically download debug symbols when analyzing
# core dumps from system libraries
```

**Batch GDB commands for CI:**

| Command | Output |
|---------|--------|
| `thread apply all bt full` | All thread backtraces with locals |
| `info registers` | CPU register state at crash |
| `print $_siginfo` | Signal that caused the crash |
| `info sharedlibrary` | Loaded shared libraries and addresses |

**Key principle:** Stripped binaries + separate `.debug` files + `debuginfod` = production binaries stay small while crash analysis remains possible from any machine with network access.

---

## 5. Debug Symbols & Tools

**Impact: MEDIUM**

Debug symbol management, release profile configuration, and core dump analysis. Ensures post-mortem debugging is possible and release binaries remain debuggable when needed.

### 5.1 Separate Debug Symbols

**Impact: MEDIUM (large binaries in production or no symbols for post-mortem)**

Extract debug symbols into a separate file for crash analysis while shipping stripped binaries. Use `rustfilt` for symbol demangling when analyzing addresses.

**Incorrect (shipping full debug binary or stripping everything):**

```bash
# Option A: Ship 500MB debug binary to production
cp target/release/app /deploy/app

# Option B: Strip everything — no post-mortem possible
strip target/release/app
# Core dump is now useless — no symbols to resolve
```

**Correct (extract symbols, strip binary, link for analysis):**

```bash
# 1. Extract debug symbols to separate file
objcopy --only-keep-debug target/release/app app.debug

# 2. Strip the binary for deployment
strip --strip-debug target/release/app

# 3. Add debug link so GDB auto-finds symbols
objcopy --add-gnu-debuglink=app.debug target/release/app

# 4. Ship stripped binary, archive .debug file

# 5. Demangle addresses from crash logs
addr2line -e app.debug 0x12345 | rustfilt
# Output: myapp::server::handle_request at src/server.rs:42
```

### 5.2 Debug Info in Release Builds

**Impact: MEDIUM (release-only bugs undiagnosable without symbols)**

Always build release with at least `debug = 1` (line tables) for profiling. Use `debug = 2` for full debugging of release-only issues. Debug info does not affect runtime performance — only binary size and build time.

**Incorrect (release profile with no debug info):**

```toml
[profile.release]
# debug = false (default) — no symbols at all
# Profilers show only hex addresses
# Release-only bugs can't be diagnosed
```

**Correct (release profile with appropriate debug info):**

```toml
# For profiling (recommended default):
[profile.release]
debug = 1                      # Line tables only — small overhead
split-debuginfo = "packed"     # Linux default — single .dwp file

# For debugging release-only issues:
[profile.release]
debug = 2                      # Full debug info — variable inspection
split-debuginfo = "unpacked"   # macOS — faster incremental linking
```

| Level | Binary Size Impact | Capability |
|-------|-------------------|------------|
| `debug = 1` | ~10-20% larger | Line-level profiling |
| `debug = 2` | ~2-5x larger | Full variable inspection |
| Runtime performance | Identical | Debug info not loaded at runtime |

