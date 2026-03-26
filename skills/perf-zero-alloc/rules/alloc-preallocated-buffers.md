---
title: Preallocated Buffers
impact: CRITICAL
impactDescription: Vec growth in hot path causes unpredictable latency from reallocation
tags: buffer, vec, clear, capacity
---

## Preallocated Buffers

**Impact: CRITICAL (Vec growth in hot path causes unpredictable latency from reallocation)**

Allocate buffers with known maximum capacity at startup. Reuse via `.clear()` which retains capacity.

```rust
pub struct DataProcessor {
    parse_buffer: Vec<u8>,
    level_buffer: Vec<PriceLevel>,
    output_buffer: Vec<Update>,
}

impl DataProcessor {
    pub fn new(max_message_size: usize, max_levels: usize) -> Self {
        Self {
            parse_buffer: Vec::with_capacity(max_message_size),
            level_buffer: Vec::with_capacity(max_levels),
            output_buffer: Vec::with_capacity(100),
        }
    }

    pub fn process(&mut self, raw_data: &[u8]) -> &[Update] {
        // Reuse buffers — zero allocations
        self.parse_buffer.clear();
        self.level_buffer.clear();
        self.output_buffer.clear();

        // Work with preallocated buffers...
        &self.output_buffer
    }
}
```

`.clear()` keeps capacity; `.truncate(0)` has the same effect. Never rely on `push()` without verifying remaining capacity.
