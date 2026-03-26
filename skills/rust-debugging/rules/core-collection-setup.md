---
title: Core Dump Collection Setup
impact: HIGH
tags: core-dump, crash, ulimit, systemd, coredumpctl
---

## Core Dump Collection Setup

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
