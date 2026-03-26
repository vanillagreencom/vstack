---
title: Instrument Budgeted Paths
impact: HIGH
impactDescription: Performance regressions go undetected
tags: debug, timing, performance, budget, comet
---

## Instrument Budgeted Paths

**Impact: HIGH (performance regressions go undetected)**

Every function with a performance budget must have an `iced::debug::time` wrapper. This feeds the comet debugger's timing panel for runtime validation.

**Wrap update() message handling:**

```rust
fn update(&mut self, message: Message) -> Task<Message> {
    iced::debug::time(format!("{message:?}"), || {
        match message { /* ... */ }
    })
}
```

**Wrap specific expensive operations:**

```rust
let geometries = iced::debug::time("chart::draw", || {
    self.data_cache.draw(renderer, bounds, |frame| { /* ... */ })
});
```

**time_with returns T only (duration goes to beacon internally):**

```rust
let elem = iced::debug::time_with("subscription::drain", || {
    self.drain_market_data()
});
```
