---
title: Allocation Profiling
impact: HIGH
impactDescription: Without profiling, allocation sources remain unknown even after detection
tags: dhat, heaptrack, profiling, tracking
---

## Allocation Profiling

**Impact: HIGH (without profiling, allocation sources remain unknown even after detection)**

Use profiling tools to identify where allocations occur before optimizing.

### dhat (Rust-native, source-level attribution)

```toml
[dev-dependencies]
dhat = "0.3"

[features]
dhat-heap = []

[profile.test]
opt-level = 1  # Required for accurate dhat results
```

```rust
#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[test]
fn profile_allocations() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::builder().testing().build();

    let mut processor = DataProcessor::new(4096, 20);

    #[cfg(feature = "dhat-heap")]
    let stats_before = dhat::HeapStats::get();

    processor.process(&create_test_data());

    #[cfg(feature = "dhat-heap")]
    {
        let stats_after = dhat::HeapStats::get();
        assert_eq!(
            stats_after.total_blocks - stats_before.total_blocks,
            0,
            "Hot path must not allocate"
        );
    }
}
// Run with: cargo test --features dhat-heap
```

### External Tools

```bash
# heaptrack (recommended for full-program profiling)
heaptrack ./target/release/my_binary
heaptrack_gui heaptrack.my_binary.*.gz

# Valgrind massif
valgrind --tool=massif --massif-out-file=massif.out ./target/release/my_binary
ms_print massif.out > heap_report.txt
```

### Verification Checklist

1. **Profile first**: heaptrack, dhat, or Divan's AllocProfiler
2. **Add allocation tests**: assert zero allocations in hot path
3. **Check hidden allocations**: `format!`, `collect()`, `to_string()`
4. **Verify capacity**: ensure Vecs do not grow in hot path
