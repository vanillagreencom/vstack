---
title: Debugging Optimized Builds
impact: HIGH
tags: debugger, optimization, gdb, registers, disassembly, inline
---

## Debugging Optimized Builds

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
