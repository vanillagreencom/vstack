---
title: Binary Size Analysis and Reduction
impact: MEDIUM
impactDescription: oversized binaries waste bandwidth and deployment time
tags: binary-size, bloat, cargo-bloat, strip, upx
---

## Binary Size Analysis and Reduction

**Impact: MEDIUM (oversized binaries waste bandwidth and deployment time)**

Use `cargo-bloat --release --crates` for per-crate size contribution analysis. Apply the reduction combo: `opt-level = "z"` (size-optimized) + `lto = true` + `codegen-units = 1` + `panic = "abort"` + `strip = "symbols"`. UPX for further compression of the final binary. Track binary size in CI to prevent gradual bloat.

**Incorrect (no size tracking, default profile):**

```bash
# No idea what contributes to binary size
cargo build --release
ls -la target/release/myapp
# 45 MB binary — who knows why
```

**Correct (analysis + reduction + CI tracking):**

```bash
# Analyze per-crate contribution
cargo install cargo-bloat --locked
cargo bloat --release --crates
# File  .text     Size  Crate
# 30.0% 10.0MiB   std
# 15.0%  5.0MiB   regex
# ...

# Analyze per-function contribution
cargo bloat --release -n 20
```

```toml
# Cargo.toml — size-optimized release profile
[profile.release-small]
inherits = "release"
opt-level = "z"        # Optimize for size over speed
lto = true             # Fat LTO for maximum size reduction
codegen-units = 1      # Single codegen unit
panic = "abort"        # Remove unwinding tables
strip = "symbols"      # Remove symbol table
```

```yaml
# CI — track binary size
steps:
  - run: cargo build --release
  - name: Record binary size
    run: |
      SIZE=$(stat --format=%s target/release/myapp)
      echo "binary_size=$SIZE" >> "$GITHUB_OUTPUT"
  - name: Check size regression
    run: |
      # Fail if binary grew more than 5%
      if [ "$SIZE" -gt "$((PREVIOUS_SIZE * 105 / 100))" ]; then
        echo "Binary size regression: $SIZE > $PREVIOUS_SIZE"
        exit 1
      fi
```
