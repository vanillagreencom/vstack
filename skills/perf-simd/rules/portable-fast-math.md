---
title: Drop-in SIMD-Accelerated Crates
impact: MEDIUM
impactDescription: reimplementing what battle-tested crates already provide
tags: portable, fast-float, simd-json, parsing, drop-in
---

## Drop-in SIMD-Accelerated Crates

**Impact: MEDIUM (reimplementing what battle-tested crates already provide)**

For batch floating-point and data parsing, use crates that leverage SIMD internally without requiring manual intrinsics:

| Crate | Replaces | Speedup |
|-------|----------|---------|
| `fast-float2` | `str::parse::<f64>()` | 2-10x faster decimal string to f64 |
| `simd-json` | `serde_json::from_str` | 2-4x faster JSON parsing |

These are drop-in replacements requiring no unsafe code in your codebase.

**Incorrect (standard library parsing in hot path):**

```rust
fn parse_prices(lines: &[&str]) -> Vec<f64> {
    lines.iter()
        .map(|s| s.parse::<f64>().unwrap()) // ~100ns per parse
        .collect()
}
```

**Correct (fast-float2 for SIMD-accelerated parsing):**

```rust
use fast_float2::parse;

fn parse_prices(lines: &[&str]) -> Vec<f64> {
    lines.iter()
        .map(|s| {
            let (val, _) = parse::<f64, _>(s.as_bytes()).unwrap(); // ~15ns per parse
            val
        })
        .collect()
}
```
