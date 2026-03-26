---
title: Arena Allocators
impact: HIGH
impactDescription: Per-object heap allocations for temporary data add allocator overhead per operation
tags: arena, bumpalo, bump, temporary
---

## Arena Allocators

**Impact: HIGH (per-object heap allocations for temporary data add allocator overhead per operation)**

Use arena allocators for groups of temporary allocations that share a lifetime. Allocate from the arena, then reset in bulk.

```rust
use bumpalo::Bump;

pub struct ScratchArena {
    arena: Bump,
}

impl ScratchArena {
    pub fn new() -> Self {
        Self {
            arena: Bump::with_capacity(64 * 1024), // 64KB arena
        }
    }

    pub fn alloc_slice<T: Default + Copy>(&self, len: usize) -> &mut [T] {
        self.arena.alloc_slice_fill_default(len)
    }

    /// Clear arena for reuse (cheap pointer reset, no per-object drop)
    pub fn reset(&mut self) {
        self.arena.reset();
    }
}
```

**Usage pattern:**
```rust
let mut scratch = ScratchArena::new(); // Startup

// Hot path: use arena for temporary data
let temp = scratch.alloc_slice::<f64>(1000);
// Use temp...
scratch.reset(); // Reuse for next operation
```

Arenas are ideal when many small allocations are created and freed together (e.g., per-tick scratch data).
