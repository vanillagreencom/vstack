# Sections

This file defines all sections, their ordering, impact levels, and descriptions.
The section ID (in parentheses) is the filename prefix used to group rules.

---

## 1. Target Configuration (target)

**Impact:** HIGH
**Description:** Target triple format, rustup setup, and cargo configuration for cross-compilation. Incorrect target configuration causes linker failures, wrong ABIs, and binaries that crash on the target platform.

## 2. Cross-Compilation Tools (tool)

**Impact:** HIGH
**Description:** Docker-based cross, Zig-based zigbuild, and nightly build-std for cross-compiling Rust. Wrong tool choice wastes hours on toolchain setup and produces broken builds.

## 3. Static Binaries (static)

**Impact:** HIGH
**Description:** musl-based static linking and OpenSSL strategies for portable binaries. Dynamic linking to glibc causes "GLIBC not found" errors on older systems and containers.

## 4. Testing & CI (ci)

**Impact:** MEDIUM
**Description:** QEMU testing, GitHub Actions matrix builds, and conditional compilation patterns. Untested cross targets ship architecture-specific bugs — especially memory ordering on ARM.
