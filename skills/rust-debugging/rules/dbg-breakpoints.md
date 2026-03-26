---
title: Breakpoint Strategies
impact: HIGH
tags: debugger, breakpoints, gdb, lldb, panic
---

## Breakpoint Strategies

**Impact: HIGH (inefficient debugging without targeted breakpoints)**

Set breakpoints strategically to catch panics, specific functions, and conditional states. Break on `rust_panic` to halt at the panic site before unwinding destroys the stack.

**Incorrect (running to crash without breakpoints):**

```rust
// Running the program and only seeing the panic message:
// thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 5'
// No live state to inspect — stack already unwound
```

**Correct (setting targeted breakpoints):**

```rust
// GDB breakpoints:
// break rust_panic                         — halt on any panic
// break module::function                   — break on specific function
// break file.rs:42 if x > 100             — conditional breakpoint
// break <Type as Trait>::method            — break on trait method impl
// break test_module::test_name             — break in specific test

// LLDB equivalents:
// br s -n rust_panic                       — halt on any panic
// br s -n module::function                 — break on specific function
// br s -f file.rs -l 42 -c 'x > 100'      — conditional breakpoint
```
