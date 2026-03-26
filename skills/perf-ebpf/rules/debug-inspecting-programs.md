---
title: Inspecting Loaded Programs
impact: MEDIUM
impactDescription: cannot verify programs loaded or maps populated without inspection
tags: bpftool, inspection, maps, debugging
---

## Inspecting Loaded Programs

**Impact: MEDIUM (cannot verify programs loaded or maps populated without inspection)**

`bpftool prog list` for loaded programs, `bpftool prog dump xlated name my_prog` for translated bytecode, `bpftool map show` for active maps, `bpftool map dump name MY_MAP` for map contents. Use for verifying programs loaded correctly and maps are being populated.

**Incorrect (guessing whether programs are loaded):**

```bash
# No visibility into BPF subsystem state
# Assuming program loaded because no error was printed
cargo xtask run &
# ... hope it works
```

**Correct (bpftool for runtime inspection):**

```bash
# List all loaded BPF programs
bpftool prog list
# Output: 42: tracepoint name my_prog tag abc123 ...

# Dump translated bytecode for a specific program
bpftool prog dump xlated name my_prog

# List all active BPF maps
bpftool map show
# Output: 7: ringbuf name EVENTS ...

# Dump map contents to verify data flow
bpftool map dump name CONN_STATE

# Combined: verify program is attached and map is populated
bpftool prog list | grep my_prog && bpftool map dump name EVENTS
```
