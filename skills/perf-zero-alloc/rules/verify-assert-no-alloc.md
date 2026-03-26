---
title: Allocation Assertions
impact: HIGH
impactDescription: Without runtime assertions, hidden allocations go undetected until production
tags: assert, test, ci, assert_no_alloc
---

## Allocation Assertions

**Impact: HIGH (without runtime assertions, hidden allocations go undetected until production)**

Use `assert_no_alloc` as a global allocator in test binaries to enforce zero allocations in hot paths. This is the CI gate -- profiling tools like dhat are for local attribution only.

```toml
[dev-dependencies]
assert_no_alloc = "1.1"
```

```rust
use assert_no_alloc::*;

#[test]
fn hot_path_must_not_allocate() {
    let mut processor = DataProcessor::new(4096, 20);
    let data = create_test_data();

    assert_no_alloc(|| {
        processor.process(&data); // Aborts if any allocation occurs
    });
}
```

No feature flags required -- `assert_no_alloc` uses a global allocator override in each test binary.

Structure allocation tests as dedicated test binaries (one per hot path) for clear CI output.
