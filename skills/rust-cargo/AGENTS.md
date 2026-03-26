# Cargo Workflows & Build Optimization

**Version 1.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when configuring
> Cargo workspaces, optimizing builds, or setting up CI pipelines. Humans
> may also find it useful, but guidance here is optimized for automation
> and consistency by AI-assisted workflows.

---

## Abstract

Workspace management, build tooling, compilation performance, and release/CI configuration for Rust projects, prioritized by impact from high (workspace management, build tooling, build performance) to medium (release and CI configuration).

---

## Table of Contents

1. [Workspace Management](#1-workspace-management) — **HIGH**
   - 1.1 [Centralized Workspace Dependencies](#11-centralized-workspace-dependencies)
   - 1.2 [Crate Splitting Strategy](#12-crate-splitting-strategy)
   - 1.3 [Feature Flag Discipline](#13-feature-flag-discipline)
2. [Build Tooling](#2-build-tooling) — **HIGH**
   - 2.1 [cargo-deny Policy Enforcement](#21-cargo-deny-policy-enforcement)
   - 2.2 [cargo-nextest Parallel Testing](#22-cargo-nextest-parallel-testing)
   - 2.3 [cargo-machete Unused Dependency Detection](#23-cargo-machete-unused-dependency-detection)
3. [Build Performance](#3-build-performance) — **HIGH**
   - 3.1 [Build Timing Analysis](#31-build-timing-analysis)
   - 3.2 [Fast Linker Configuration](#32-fast-linker-configuration)
   - 3.3 [Cranelift Dev Backend](#33-cranelift-dev-backend)
   - 3.4 [Incremental Build Tuning](#34-incremental-build-tuning)
   - 3.5 [Shared Compilation Cache](#35-shared-compilation-cache)
4. [Release & CI](#4-release--ci) — **MEDIUM**
   - 4.1 [Release Profile Configuration](#41-release-profile-configuration)
   - 4.2 [Monomorphization Bloat Detection](#42-monomorphization-bloat-detection)
   - 4.3 [Binary Size Analysis and Reduction](#43-binary-size-analysis-and-reduction)

---

## 1. Workspace Management

**Impact: HIGH**

Cargo workspace layout, dependency centralization, crate splitting strategy, and feature flag discipline. Violations cause version conflicts, slow builds, and broken feature combinations.

### 1.1 Centralized Workspace Dependencies

**Impact: HIGH (version conflicts and duplicate dependency trees)**

Use `[workspace.dependencies]` in the root `Cargo.toml` to centralize version management. Member crates reference shared dependencies with `dep.workspace = true` instead of specifying versions directly. `resolver = "2"` is mandatory for edition 2021+ (feature unification per-platform, not global). Use the virtual manifest pattern (no `[package]` in root) for multi-crate projects.

**Incorrect (versions scattered across member crates):**

```toml
# crates/core/Cargo.toml
[dependencies]
serde = "1.0.197"
tokio = { version = "1.36", features = ["full"] }

# crates/api/Cargo.toml
[dependencies]
serde = "1.0.193"  # Different version — causes duplicate in tree
tokio = { version = "1.35", features = ["rt"] }
```

**Correct (centralized in workspace root):**

```toml
# Cargo.toml (virtual manifest)
[workspace]
members = ["crates/*"]
resolver = "2"

[workspace.dependencies]
serde = { version = "1.0.197", features = ["derive"] }
tokio = { version = "1.36", features = ["full"] }

# crates/core/Cargo.toml
[dependencies]
serde.workspace = true
tokio.workspace = true

# crates/api/Cargo.toml
[dependencies]
serde.workspace = true
tokio = { workspace = true, features = ["rt"] }  # Can add features
```

### 1.2 Crate Splitting Strategy

**Impact: HIGH (serial compilation bottlenecks and slow incremental builds)**

Split workspace crates strategically to maximize build parallelism and minimize incremental rebuild scope. Key splitting triggers: separate proc-macros (they compile serially and block all dependents), break circular dependencies, and isolate frequently-changed code so edits don't invalidate the entire dependency graph. Group by coupling — types and functions that change together stay in the same crate.

**Incorrect (monolith crate with embedded proc-macro):**

```toml
# Cargo.toml — single crate does everything
[package]
name = "myapp"

[dependencies]
syn = "2"       # proc-macro deps compile even for non-macro code
quote = "1"
proc-macro2 = "1"

[lib]
proc-macro = true  # Entire crate is proc-macro — nothing compiles in parallel
```

**Correct (split by compilation characteristics):**

```toml
# Cargo.toml (workspace root)
[workspace]
members = [
    "crates/myapp",         # Binary — depends on core + macros
    "crates/myapp-core",    # Types, traits — no heavy deps, changes rarely
    "crates/myapp-macros",  # Proc-macro — compiles in parallel with core
    "crates/myapp-engine",  # Business logic — changes frequently, fast incremental
]
resolver = "2"
```

### 1.3 Feature Flag Discipline

**Impact: HIGH (broken builds from non-additive features or implicit feature creation)**

Features must be additive — enabling a feature must never remove functionality or change behavior in a way that breaks code compiled without it. Use `dep:optional_dep` syntax (Rust 1.60+) to avoid implicit feature creation from optional dependency names. Default features should cover the common use case. Never feature-gate safety-critical code (safety checks must always be compiled in).

**Incorrect (non-additive feature and implicit feature creation):**

```toml
[features]
default = ["std"]
std = []
no_std = []  # Non-additive: enabling both std and no_std is contradictory

[dependencies]
openssl = { version = "0.10", optional = true }
# Creates implicit feature "openssl" — confusing, leaks dep name as public API
```

```rust
#[cfg(feature = "no_std")]
fn validate_input(data: &[u8]) -> bool {
    // Safety check only compiled in no_std — BUG: std builds skip validation
    data.len() <= MAX_SIZE
}
```

**Correct (additive features with dep: syntax):**

```toml
[features]
default = ["std"]
std = []
# no "no_std" feature — std absence is the no_std path

tls = ["dep:openssl"]  # Explicit dep: syntax, no implicit feature leak

[dependencies]
openssl = { version = "0.10", optional = true }
```

```rust
// Safety validation always compiled — never behind a feature gate
fn validate_input(data: &[u8]) -> bool {
    data.len() <= MAX_SIZE
}

#[cfg(feature = "tls")]
fn connect_tls(addr: &str) -> Result<TlsStream, Error> {
    // Feature-gated functionality that adds capability
    // ...
}
```

---

## 2. Build Tooling

**Impact: HIGH**

Essential cargo plugins for policy enforcement, fast testing, and dependency hygiene. Missing tooling lets vulnerabilities, unused deps, and slow test suites slip through CI.

### 2.1 cargo-deny Policy Enforcement

**Impact: HIGH (vulnerable, banned, or unlicensed dependencies slip into production)**

`cargo-deny` enforces license, advisory, ban, and source policies. Configure `deny.toml` with four sections: `[advisories]` for RUSTSEC database checks, `[licenses]` with an explicit allowlist, `[bans]` for duplicate or forbidden crates, and `[sources]` for registry restrictions. Run as a blocking CI check. Example: ban `openssl` globally, allow via a wrapper exception.

**Incorrect (no policy enforcement — anything goes):**

```yaml
# CI pipeline with no dependency auditing
steps:
  - run: cargo build
  - run: cargo test
  # No license check, no advisory scan, no ban enforcement
```

**Correct (deny.toml with full policy):**

```toml
# deny.toml
[advisories]
vulnerability = "deny"
unmaintained = "warn"
yanked = "warn"
notice = "warn"

[licenses]
unlicensed = "deny"
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Unicode-DFS-2016",
]
confidence-threshold = 0.8

[bans]
multiple-versions = "warn"
wildcards = "allow"
deny = [
    { name = "openssl-sys", wrappers = ["openssl"] },
    { name = "cmake" },
]

[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["https://github.com/rust-lang/crates.io-index"]
```

```yaml
# CI step — blocking check
- run: cargo install cargo-deny --locked
- run: cargo deny check
```

### 2.2 cargo-nextest Parallel Testing

**Impact: HIGH (slow test suites blocking CI feedback loops)**

`cargo nextest run` provides parallel test execution, 2-3x faster than `cargo test` for large test suites. Configure `.config/nextest.toml` for test-threads, retries for flaky tests, JUnit output for CI reporting, per-test timeouts, and test grouping. Nextest runs each test as a separate process, providing better isolation and failure output.

**Incorrect (default cargo test with no configuration):**

```yaml
# CI — serial test execution, no retries, no timeout
steps:
  - run: cargo test --workspace
  # Single-threaded by default for integration tests
  # No JUnit output for CI dashboards
  # Flaky test fails the whole run
```

**Correct (nextest with full configuration):**

```toml
# .config/nextest.toml
[store]
dir = "target/nextest"

[profile.default]
test-threads = "num-cpus"
status-level = "pass"
failure-output = "immediate-final"
slow-timeout = { period = "60s", terminate-after = 2 }

[profile.default.junit]
path = "target/nextest/default/junit.xml"

[profile.ci]
test-threads = 4
retries = 2
fail-fast = false

[profile.ci.junit]
path = "target/nextest/ci/junit.xml"
```

```yaml
# CI step
steps:
  - run: cargo install cargo-nextest --locked
  - run: cargo nextest run --workspace --profile ci
```

### 2.3 cargo-machete Unused Dependency Detection

**Impact: HIGH (bloated dependency tree increases compile time and attack surface)**

Detect unused dependencies with `cargo-machete`. Run before releases to trim the dependency tree. Known false positives: proc-macro dependencies (used at compile time only), build-only dependencies (`build-dependencies`), and crates used only via `#[cfg]`-gated code. Combine with `cargo-udeps` (requires nightly) for compile-time verification that catches cases machete's heuristic misses.

**Incorrect (unused dependencies accumulate silently):**

```toml
# Cargo.toml — accumulated deps never cleaned up
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"          # Was used, removed from code, still in Cargo.toml
rand = "0.8"              # Only used in a deleted test
regex = "1"               # Replaced by manual parsing months ago
tokio = { version = "1", features = ["full"] }
```

**Correct (regular auditing with machete):**

```bash
# Quick heuristic scan (works on stable)
cargo install cargo-machete
cargo machete

# Compile-time verification (requires nightly)
cargo install cargo-udeps --locked
cargo +nightly udeps --workspace

# Suppress false positives in Cargo.toml
[package.metadata.cargo-machete]
ignored = ["proc-macro-crate"]  # Used by proc-macro at compile time
```

---

## 3. Build Performance

**Impact: HIGH**

Compilation speed optimizations — linker choice, caching, codegen backend, incremental build tuning. Slow builds kill iteration speed and developer productivity.

### 3.1 Build Timing Analysis

**Impact: HIGH (invisible serial bottlenecks waste minutes per build)**

`cargo build --timings` generates an HTML timeline showing crate parallelism and bottlenecks. Identify: serial bottleneck crates (long bars with nothing compiling in parallel), proc-macro compilation (blocks all dependents), and underutilized CPU cores. Run periodically to catch regressions in build graph structure.

**Incorrect (blind to build bottlenecks):**

```bash
# Just build and hope it's fast
cargo build --release
# No idea which crate takes 40% of total build time
# No idea proc-macro blocks 12 downstream crates
```

**Correct (regular timing analysis):**

```bash
# Generate build timing report
cargo build --timings --release

# Opens cargo-timing.html showing:
# - Per-crate compile time as horizontal bars
# - Parallelism chart (how many crates compile simultaneously)
# - Critical path through the dependency graph
# - Codegen vs non-codegen time breakdown

# Look for:
# 1. Long bars with low parallelism = serial bottleneck
# 2. Proc-macro crates blocking many dependents
# 3. Single crate dominating total time = split candidate
```

### 3.2 Fast Linker Configuration

**Impact: HIGH (linking dominates build time for large binaries)**

Linker speed comparison: mold (5-10x over GNU ld) > lld (2x) > gold (1.5x) > GNU ld. Configure in `.cargo/config.toml`. Biggest win for large binaries where linking can take 10-30 seconds with GNU ld. mold is the recommended default for Linux development.

**Incorrect (default GNU ld linker):**

```toml
# .cargo/config.toml — no linker configured
# Uses system default (GNU ld) — slowest option
# 15-30 second link times for large binaries
```

**Correct (mold linker on Linux):**

```toml
# .cargo/config.toml

# Linux — mold (fastest)
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]

# macOS — use lld via Xcode (mold not available)
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# Windows — lld (bundled with Rust)
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

```bash
# Install mold on Linux
# Ubuntu/Debian
sudo apt install mold
# Arch
sudo pacman -S mold
# Fedora
sudo dnf install mold
```

### 3.3 Cranelift Dev Backend

**Impact: HIGH (unnecessary codegen time in debug builds slows iteration)**

Cranelift codegen backend provides 20-40% faster development compilation at the cost of slower runtime performance. Use for `cargo check`-equivalent workflow where you need to run tests but don't care about execution speed. Configure via environment variable or cargo config. Nightly-only. Never use for release builds or benchmarks.

**Incorrect (using cranelift for release or benchmarks):**

```toml
# .cargo/config.toml
[profile.release]
codegen-backend = "cranelift"  # WRONG: cranelift generates slower code
# Benchmarks will be meaningless, release binary will underperform
```

**Correct (cranelift only for dev profile):**

```bash
# Install cranelift component (nightly required)
rustup component add rustc-codegen-cranelift-preview --toolchain nightly

# Option 1: Environment variable (per-session)
CARGO_PROFILE_DEV_CODEGEN_BACKEND=cranelift cargo +nightly test

# Option 2: Cargo config (persistent)
```

```toml
# .cargo/config.toml — dev only
[unstable]
codegen-backend = true

[profile.dev]
codegen-backend = "cranelift"

# Release always uses default LLVM backend
[profile.release]
# codegen-backend not set — uses LLVM
```

### 3.4 Incremental Build Tuning

**Impact: HIGH (suboptimal debug profile wastes 20-40% of every rebuild)**

Tune `[profile.dev]` for fast incremental builds. `debug = 1` (line tables only) saves 20-40% build time over `debug = 2` (full debuginfo). `split-debuginfo = "unpacked"` on macOS avoids dsymutil bottleneck. `[profile.dev.package."*"] opt-level = 1` optimizes dependencies in debug mode without hurting own-code incremental compilation. `codegen-units = 256` maximizes dev parallelism.

**Incorrect (default dev profile with full debuginfo):**

```toml
# Cargo.toml — using defaults
[profile.dev]
# debug = 2 (default) — full DWARF, slow to generate and link
# codegen-units = 16 (default) — could be higher for dev
# Dependencies compiled at opt-level = 0 — slow runtime in tests
```

**Correct (tuned dev profile):**

```toml
# Cargo.toml
[profile.dev]
debug = 1               # Line tables only — 20-40% faster builds
codegen-units = 256      # Maximum parallelism for dev builds
incremental = true       # Explicit (default for dev, but document intent)
split-debuginfo = "unpacked"  # macOS: skip dsymutil bottleneck

# Optimize dependencies even in dev (faster test execution)
[profile.dev.package."*"]
opt-level = 1            # Light optimization for deps
                         # Own code stays at opt-level 0 for fast incremental

# CI-specific: disable incremental (no cache between runs)
# Set via environment: CARGO_INCREMENTAL=0
```

### 3.5 Shared Compilation Cache

**Impact: HIGH (redundant recompilation across builds and CI runs)**

Shared compilation cache via `RUSTC_WRAPPER=sccache`. Works locally or distributed (S3, GCS, Redis). For GitHub Actions CI, use `Swatinem/rust-cache@v2` to cache `target/` between runs. Disable caching for final release builds where cache may interfere with LTO.

**Incorrect (no caching — full rebuild every time):**

```yaml
# GitHub Actions — no caching
steps:
  - uses: actions/checkout@v4
  - run: cargo build --workspace
  # Every CI run recompiles all dependencies from scratch
  # 5-15 minutes wasted per run
```

**Correct (sccache locally + CI caching):**

```bash
# Local development
cargo install sccache --locked
export RUSTC_WRAPPER=sccache

# Verify it's working
sccache --show-stats

# Optional: distributed cache
export SCCACHE_BUCKET=my-sccache-bucket
export SCCACHE_REGION=us-east-1
```

```yaml
# GitHub Actions — with rust-cache
steps:
  - uses: actions/checkout@v4

  - uses: dtolnay/rust-toolchain@stable

  - uses: Swatinem/rust-cache@v2
    with:
      cache-on-failure: true
      shared-key: "ci-${{ hashFiles('**/Cargo.lock') }}"

  - name: Install sccache
    uses: mozilla-actions/sccache-action@v0.0.4

  - run: cargo build --workspace
    env:
      RUSTC_WRAPPER: sccache

  # Disable cache for final release (LTO needs clean build)
  - run: cargo build --release
    if: github.ref == 'refs/heads/main'
    env:
      RUSTC_WRAPPER: ""
      CARGO_INCREMENTAL: "0"
```

---

## 4. Release & CI

**Impact: MEDIUM**

Release profile configuration, monomorphization bloat detection, and binary size tracking. Misconfigurations waste runtime performance or produce unnecessarily large binaries.

### 4.1 Release Profile Configuration

**Impact: MEDIUM (suboptimal release builds waste runtime performance or link time)**

Configure release profile for the right tradeoff: `opt-level = 3` for maximum runtime performance, `lto = "thin"` for +5-15% performance with moderate link time, `codegen-units = 1` for better optimization at the cost of slower builds, and `strip = "symbols"` for smaller binaries. `panic = "abort"` saves binary size if no `catch_unwind` is used.

| LTO Mode | Perf Gain | Link Time | Use Case |
|----------|-----------|-----------|----------|
| `false` | Baseline | Fast | Dev iteration |
| `"thin"` | +5-15% | Moderate | Default release |
| `true` / `"fat"` | +10-20% | Slow | Maximum performance |

**Incorrect (default release profile — no tuning):**

```toml
# Cargo.toml — using release defaults
[profile.release]
# opt-level = 3 (ok, default)
# lto = false (default) — missing 5-15% perf
# codegen-units = 16 (default) — prevents cross-crate optimization
# strip = false (default) — debug symbols inflate binary
```

**Correct (tuned release profile):**

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = "thin"           # +5-15% perf, moderate link time
codegen-units = 1      # Better optimization, slower build
strip = "symbols"      # Remove debug symbols from binary
panic = "abort"        # Smaller binary, no catch_unwind support

# Separate profile for profiling (needs debug symbols)
[profile.profiling]
inherits = "release"
debug = 1              # Line tables for perf/flamegraph
strip = false          # Keep symbols for profiler
```

### 4.2 Monomorphization Bloat Detection

**Impact: MEDIUM (generic-heavy code inflates compile time and binary size)**

Measure monomorphization bloat with `cargo llvm-lines | head -20`. Each generic function instantiation creates a separate copy of LLVM IR. Fix with the thin generic wrapper pattern: public generic function calls a concrete inner function. Target: no single function should exceed 5% of total LLVM lines.

**Incorrect (fully generic function body):**

```rust
// Every call with a different T duplicates the entire function body
pub fn process_data<T: AsRef<[u8]>>(data: T, config: &Config) -> Result<Output> {
    let bytes = data.as_ref();
    // 50+ lines of processing logic
    // All duplicated for every T: &[u8], Vec<u8>, String, &str, Bytes...
    let header = parse_header(bytes)?;
    let payload = decrypt(bytes, config)?;
    let result = transform(payload, &header)?;
    validate(&result)?;
    Ok(result)
}
```

**Correct (thin generic wrapper + concrete inner):**

```rust
// Generic wrapper — only the conversion is duplicated (1 line)
pub fn process_data<T: AsRef<[u8]>>(data: T, config: &Config) -> Result<Output> {
    process_data_inner(data.as_ref(), config)
}

// Concrete inner — compiled once regardless of how many T types exist
fn process_data_inner(bytes: &[u8], config: &Config) -> Result<Output> {
    let header = parse_header(bytes)?;
    let payload = decrypt(bytes, config)?;
    let result = transform(payload, &header)?;
    validate(&result)?;
    Ok(result)
}
```

```bash
# Measure bloat
cargo install cargo-llvm-lines
cargo llvm-lines --release | head -20

# Output shows: Lines  Copies  Function
# Target: top functions < 5% of total lines
```

### 4.3 Binary Size Analysis and Reduction

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
