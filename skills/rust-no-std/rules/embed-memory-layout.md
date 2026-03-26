---
title: Memory Layout
impact: HIGH
tags: memory-x, linker-script, flash, ram, binary-size, release-profile
---

## Memory Layout

**Impact: HIGH (incorrect memory layout causes hard faults or flash overflow)**

Embedded targets require explicit memory layout via linker scripts. The `memory.x` file defines FLASH and RAM regions for the target MCU. Configure `.cargo/config.toml` for the target and linker script. Optimize release profile for binary size: every byte counts on microcontrollers.

**memory.x (example for STM32F411):**

```
MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 512K
    RAM   : ORIGIN = 0x20000000, LENGTH = 128K
}
```

**.cargo/config.toml:**

```toml
[target.thumbv7em-none-eabihf]
runner = "probe-run --chip STM32F411CEUx"
rustflags = [
    "-C", "link-arg=-Tlink.x",
]

[build]
target = "thumbv7em-none-eabihf"
```

**Incorrect (debug-friendly settings in release for embedded):**

```toml
[profile.release]
opt-level = 2        # Not size-optimized
# lto = false        # Missing LTO
# panic = "unwind"   # Unwinding pulls in massive code
```

**Correct (release profile minimizing binary size):**

```toml
[profile.release]
opt-level = "z"      # Optimize for size
lto = true           # Link-time optimization
panic = "abort"      # No unwinding — saves significant space
codegen-units = 1    # Better optimization, slower compile
strip = true         # Strip symbols
```
