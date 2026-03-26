---
title: Non-Interactive Core Dump Triage
impact: HIGH
tags: core-dump, gdb, batch, ci, debuginfod, triage
---

## Non-Interactive Core Dump Triage

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
