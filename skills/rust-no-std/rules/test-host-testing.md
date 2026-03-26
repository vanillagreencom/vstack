---
title: Host Testing
impact: MEDIUM
tags: testing, no_std, host, cfg
---

## Host Testing

**Impact: MEDIUM (no test coverage without host testing strategy)**

Use conditional compilation to enable std during test builds. This lets you run logic tests on the host with `cargo test` while keeping the library no_std in production.

**Incorrect (no test support in no_std crate):**

```rust
#![no_std]

// Can't run `cargo test` — no test harness without std
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
```

**Correct (conditional no_std for host testing):**

```rust
#![cfg_attr(not(test), no_std)]

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

Mock hardware dependencies with traits to enable host testing:

```rust
pub trait Clock {
    fn now_ms(&self) -> u64;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockClock(u64);
    impl Clock for MockClock {
        fn now_ms(&self) -> u64 { self.0 }
    }

    #[test]
    fn test_with_mock_clock() {
        let clock = MockClock(1000);
        // Test logic without real hardware
    }
}
```
