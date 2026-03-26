---
title: Aya Project Setup
impact: HIGH
impactDescription: wrong workspace layout prevents compilation or deployment
tags: aya, setup, workspace, no_std, xtask
---

## Aya Project Setup

**Impact: HIGH (wrong workspace layout prevents compilation or deployment)**

Workspace layout: `my-ebpf-ebpf/` (kernel-side `#![no_std]`/`#![no_main]`, target: bpf), `my-ebpf/` (userspace with tokio), `xtask/` (build helper). Generate with `cargo generate https://github.com/aya-rs/aya-template`. Build: `cargo xtask build-ebpf && cargo xtask run`. Kernel-side requires `#[panic_handler]` that calls `core::hint::unreachable_unchecked()`. Maps declared with `#[map]` attribute.

**Incorrect (flat layout, no workspace separation):**

```rust
// Single crate trying to do both kernel and userspace
#![no_std]
use std::net::TcpStream; // ERROR: no std in BPF programs
```

**Correct (proper workspace layout):**

```rust
// my-ebpf-ebpf/src/main.rs (kernel-side)
#![no_std]
#![no_main]

use aya_ebpf::{macros::tracepoint, programs::TracePointContext};

#[tracepoint]
pub fn my_prog(ctx: TracePointContext) -> u32 {
    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

// my-ebpf/src/main.rs (userspace)
use aya::programs::TracePoint;
use tokio::signal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut ebpf = aya::Ebpf::load(aya::include_bytes_aligned!(
        concat!(env!("OUT_DIR"), "/my-ebpf")
    ))?;
    let program: &mut TracePoint = ebpf.program_mut("my_prog").unwrap().try_into()?;
    program.load()?;
    program.attach("syscalls", "sys_enter_read")?;
    signal::ctrl_c().await?;
    Ok(())
}
```
