---
title: Monomorphization Bloat Detection
impact: MEDIUM
impactDescription: generic-heavy code inflates compile time and binary size
tags: llvm-lines, monomorphization, generics, bloat
---

## Monomorphization Bloat Detection

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
