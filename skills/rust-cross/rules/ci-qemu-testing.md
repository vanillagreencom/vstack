---
title: QEMU Testing for Cross Targets
impact: MEDIUM
tags: qemu, testing, arm64, runner, memory-ordering
---

## QEMU Testing for Cross Targets

**Impact: MEDIUM (architecture-specific bugs ship undetected without cross-target testing)**

Use QEMU to run cross-compiled test binaries on the host machine. The `cross` tool includes QEMU automatically. For manual setup, configure the QEMU runner in `.cargo/config.toml`. Testing on ARM64 catches memory ordering bugs hidden by x86's strong TSO memory model.

**Incorrect (only testing on x86_64 and assuming other architectures work):**

```rust
// Code with relaxed atomics that works on x86 (TSO) but breaks on ARM:
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

static FLAG: AtomicBool = AtomicBool::new(false);
static DATA: AtomicU64 = AtomicU64::new(0);

// Writer
DATA.store(42, Ordering::Relaxed);
FLAG.store(true, Ordering::Relaxed);  // No release barrier

// Reader — on ARM, may see FLAG=true but DATA=0
if FLAG.load(Ordering::Relaxed) {
    assert_eq!(DATA.load(Ordering::Relaxed), 42); // Can fail on ARM!
}
```

**Correct (testing on ARM64 via QEMU to catch ordering bugs):**

```toml
# .cargo/config.toml — configure QEMU runner
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
runner = "qemu-aarch64 -L /usr/aarch64-linux-gnu"
```

```bash
# Option 1: Using cross (includes QEMU automatically)
cross test --target aarch64-unknown-linux-gnu

# Option 2: Manual QEMU setup
sudo apt install qemu-user qemu-user-static
rustup target add aarch64-unknown-linux-gnu
sudo apt install gcc-aarch64-linux-gnu
cargo test --target aarch64-unknown-linux-gnu
# cargo uses the runner from .cargo/config.toml automatically
```
