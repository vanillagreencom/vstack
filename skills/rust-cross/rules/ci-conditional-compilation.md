---
title: Conditional Compilation
impact: MEDIUM
tags: cfg, conditional, platform, target_os, target_arch
---

## Conditional Compilation

**Impact: MEDIUM (dead code behind untested cfg is maintenance debt that rots silently)**

Use `#[cfg(...)]` attributes for platform-specific code. Test ALL cfg branches in CI by building against each target. Use `cfg_if` for complex conditions. Code behind `#[cfg]` that is never compiled in CI will accumulate errors silently.

**Incorrect (untested platform-specific code):**

```rust
// Platform-specific code that is never compiled in CI:
#[cfg(target_os = "linux")]
fn get_cpu_count() -> usize {
    // This compiles and is tested
    num_cpus::get()
}

#[cfg(target_os = "macos")]
fn get_cpu_count() -> usize {
    // This is never compiled in CI — may have syntax errors,
    // missing imports, or wrong API usage that goes undetected
    sysctl_get_cpu_count()  // Does this function even exist?
}

// Per-item cfg scattered everywhere:
#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
#[cfg(target_arch = "aarch64")]
fn fast_path() { /* ... */ }
#[cfg(target_arch = "x86_64")]
fn fast_path() { /* ... */ }
```

**Correct (organized cfg with CI verification):**

```rust
// Use cfg_if for complex platform branching
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux_impl;
        pub use linux_impl::get_cpu_count;
    } else if #[cfg(target_os = "macos")] {
        mod macos_impl;
        pub use macos_impl::get_cpu_count;
    } else {
        // Fallback — compile error on unsupported platforms
        compile_error!("Unsupported OS: add a get_cpu_count implementation");
    }
}

// Group platform-specific code into modules, not scattered per-item
#[cfg(target_arch = "aarch64")]
mod simd_aarch64;
#[cfg(target_arch = "x86_64")]
mod simd_x86;

// Verify all cfg branches compile in CI:
// cargo check --target x86_64-unknown-linux-gnu
// cargo check --target aarch64-unknown-linux-gnu
// cargo check --target x86_64-apple-darwin
```
