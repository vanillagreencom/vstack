---
title: Recursive Box Allocations
impact: MEDIUM
impactDescription: Box in recursive structures allocates on every insert
tags: box, recursive, tree, arena
---

## Recursive Box Allocations

**Impact: MEDIUM (Box in recursive structures allocates on every insert)**

`Box<T>` in recursive data structures (trees, linked lists) allocates on every node creation. Use arena allocators instead.

**Incorrect (heap allocation per node):**

```rust
enum Tree<T> {
    Leaf,
    Node {
        value: T,
        left: Box<Tree<T>>,   // Allocates!
        right: Box<Tree<T>>,  // Allocates!
    }
}
```

**Correct (arena-allocated nodes):**

```rust
struct ArenaTree<'a, T> {
    nodes: &'a Bump,
}
```

Arena allocation amortizes the cost across all nodes and frees them in a single `reset()`.
