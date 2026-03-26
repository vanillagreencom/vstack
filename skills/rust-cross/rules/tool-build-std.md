---
title: Build-std (Nightly)
impact: HIGH
tags: build-std, nightly, no_std, embedded, custom-target
---

## Build-std (Nightly)

**Impact: HIGH (bare-metal targets have no pre-built std — build fails without this)**

`-Zbuild-std` rebuilds the standard library from source for custom targets. Required for bare-metal/embedded targets not available via `rustup`. Nightly-only. Use a custom target specification JSON for exotic platforms with custom data layouts, linker flavors, or panic behavior.

**Incorrect (trying to use a bare-metal target without build-std):**

```rust
// cargo build --target thumbv7em-none-eabihf
// Error: can't find crate for `core`
//
// Or trying to use a custom target JSON:
// cargo build --target my-custom-target.json
// Error: no pre-built std available for this target
```

**Correct (using -Zbuild-std on nightly):**

```bash
# Install nightly and rust-src component
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly

# Build with core + alloc (no_std with heap allocation)
cargo +nightly build -Zbuild-std=core,alloc \
  --target thumbv7em-none-eabihf --release

# For custom targets, create a target spec JSON:
# my-target.json
```

```json
{
  "llvm-target": "thumbv7em-none-eabihf",
  "data-layout": "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64",
  "arch": "arm",
  "os": "none",
  "env": "eabihf",
  "linker-flavor": "gnu-lld-cc",
  "linker": "arm-none-eabi-gcc",
  "panic-strategy": "abort",
  "features": "+thumb2,+v7,+vfp4,-d32"
}
```

```bash
# Build with custom target spec
cargo +nightly build -Zbuild-std=core,alloc \
  --target my-target.json --release
```
