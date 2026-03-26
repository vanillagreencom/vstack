# Iced 0.14 Patterns

**Version 1.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when building,
> maintaining, or refactoring Iced 0.14 applications. Humans may also
> find it useful, but guidance here is optimized for automation and
> consistency by AI-assisted workflows.

---

## Nomenclature

App > Window > Shell > Zone > TitleBar > Panel > Canvas > Overlay

## Dev Tools

| Tool | Purpose | Install |
|------|---------|---------|
| `cargo-hot` | Live UI patching without restart | `cargo install cargo-hot` |
| `comet` | Iced debugger: frame metrics, widget tree, message inspector | `cargo install --locked --git https://github.com/iced-rs/comet.git` |

## Abstract

Patterns and rules for building high-performance UIs with Iced 0.14's reactive Elm Architecture, prioritized by impact from critical (framework hard rules) to incremental (interaction gotchas). Each rule includes detailed explanations and, where applicable, incorrect vs. correct code examples.

---

## Table of Contents

1. [Hard Rules](#1-hard-rules) — **CRITICAL**
   - 1.1 [Widget Tree Consistency](#11-widget-tree-consistency)
   - 1.2 [Pick Area Geometry](#12-pick-area-geometry)
   - 1.3 [Scroll State Initialization](#13-scroll-state-initialization)
   - 1.4 [Overlay State Isolation](#14-overlay-state-isolation)
   - 1.5 [Single Message Per Interaction](#15-single-message-per-interaction)
   - 1.6 [View Is Pure](#16-view-is-pure)
   - 1.7 [Minimum Pane Size](#17-minimum-pane-size)
   - 1.8 [Title Bar Event Ordering](#18-title-bar-event-ordering)
2. [Development Practices](#2-development-practices) — **HIGH**
   - 2.1 [Validate API Before Use](#21-validate-api-before-use)
   - 2.2 [Reactive Discipline](#22-reactive-discipline)
   - 2.3 [Instrument Budgeted Paths](#23-instrument-budgeted-paths)
   - 2.4 [No Redundant Event Subscriptions](#24-no-redundant-event-subscriptions)
   - 2.5 [Press-and-Hold Input](#25-press-and-hold-input)
   - 2.6 [Smoke Test After UI Changes](#26-smoke-test-after-ui-changes)
3. [Cache & Multi-Window](#3-cache--multi-window) — **HIGH**
   - 3.1 [Trace Staleness Before Coding](#31-trace-staleness-before-coding)
   - 3.2 [Extend Existing Event Paths](#32-extend-existing-event-paths)
   - 3.3 [Regression Tests for Invalidation](#33-regression-tests-for-invalidation)
4. [Elm Architecture](#4-elm-architecture) — **MEDIUM**
   - 4.1 [Message and State Stay in Root](#41-message-and-state-stay-in-root)
   - 4.2 [Module Extraction Pattern](#42-module-extraction-pattern)
5. [Interaction](#5-interaction) — **MEDIUM**
   - 5.1 [Overlay Starvation](#51-overlay-starvation)
   - 5.2 [Keep PaneGrid Drag Feedback Internal](#52-keep-panegrid-drag-feedback-internal)
6. [Chart Rendering](#6-chart-rendering)
7. [Subscription-Based Data Streams](#7-subscription-based-data-streams)
8. [Theming](#8-theming)
9. [Iced 0.14 API Reference](#9-iced-014-api-reference)

---

## 1. Hard Rules

**Impact: CRITICAL**

Non-negotiable constraints from Iced 0.14 framework behavior. Violations cause silent breakage — widgets stop responding, events misroute, or state corrupts without errors.

### 1.1 Widget Tree Consistency

**Impact: CRITICAL (widgets silently stop responding to events)**

Iced tracks widgets by tree position. Conditionally wrapping widgets based on interaction state changes the tree structure, breaking event tracking.

**Incorrect (conditional wrapping changes tree shape):**

```rust
if dragging {
    mouse_area(label).into()
} else {
    label.into()
}
```

**Correct (always wrap, conditionally enable):**

```rust
mouse_area(label)
    .on_press_maybe(if enable { Some(msg) } else { None })
```

### 1.2 Pick Area Geometry

**Impact: CRITICAL (pane drag completely disabled)**

TitleBar content must use `Shrink` width so empty space remains for the pick area. `Fill` width on tab row content eliminates the pick area, disabling pane drag entirely.

**Incorrect (Fill width consumes pick area):**

```rust
pane_grid::TitleBar::new(
    row![tabs].width(Length::Fill)  // No pick area left
)
```

**Correct (Shrink width preserves pick area):**

```rust
pane_grid::TitleBar::new(
    row![tabs].width(Length::Shrink)  // Pick area in remaining space
)
```

### 1.3 Scroll State Initialization

**Impact: CRITICAL (initial dimensions never captured)**

`scrollable.on_scroll` fires only on user-initiated scroll events, never on initial layout. Use `sensor.on_show` to capture initial dimensions, then combine with `on_scroll` for ongoing tracking.

### 1.4 Overlay State Isolation

**Impact: CRITICAL (base layer widgets break when overlays change)**

Overlay layers (stack children beyond the base) must not affect the base layer's widget structure. Add/remove overlay layers freely, but never change how base-layer widgets are constructed based on overlay presence.

### 1.5 Single Message Per Interaction

**Impact: CRITICAL (race conditions and unpredictable state)**

Each widget interaction produces exactly one message. For composite actions (e.g., tab press that might become a drag), use state machines in `update()` rather than emitting multiple messages from `view()`.

### 1.6 View Is Pure

**Impact: CRITICAL (hidden state corruption and non-deterministic rendering)**

`view()` must be a pure function of `State`. No side effects, no memoization that depends on call frequency, no hidden state. All mutable state lives in `State` and changes only in `update()`.

### 1.7 Minimum Pane Size

**Impact: CRITICAL (panes collapse or ignore per-pane minimums)**

`PaneGrid::min_size` sets a uniform minimum for ALL panes. For per-pane minimums, wrap panel content in `container` with `min_width`/`min_height` and clamp resize ratios (0.15-0.85).

### 1.8 Title Bar Event Ordering

**Impact: CRITICAL (state cleared by body handler overwrites title bar state)**

In `pane_grid::Content::update`, the title bar is processed before the body. When the cursor crosses from body to title bar in a single frame, title bar messages (e.g., `TabBarEntered`) fire before body messages (e.g., `PaneBodyExited`). Do not unconditionally clear state in body-exit handlers that the title bar just established.

---

## 2. Development Practices

**Impact: HIGH**

Practices that prevent common Iced development pitfalls — API drift, runtime panics, redundant subscriptions, and missed performance regressions.

### 2.1 Validate API Before Use

**Impact: HIGH (silent compilation failures or runtime panics from 0.13 API assumptions)**

Iced 0.14 has significant breaking changes from 0.13. Always verify widget APIs, entry points, and trait signatures against current docs before assuming API shape.

### 2.2 Reactive Discipline

**Impact: HIGH (unnecessary redraws, wasted GPU cycles, janky frame rates)**

Never trigger redraws from `view()`. Invalidate caches explicitly in `update()`. Batch high-frequency data updates into ~16ms windows so `update()` sees bounded work and idle windows cause no redraw.

### 2.3 Instrument Budgeted Paths

**Impact: HIGH (performance regressions go undetected)**

Every function with a performance budget must have an `iced::debug::time` wrapper. This feeds the comet debugger's timing panel for runtime validation.

Wrap update() message handling:

```rust
fn update(&mut self, message: Message) -> Task<Message> {
    iced::debug::time(format!("{message:?}"), || {
        match message { /* ... */ }
    })
}
```

Wrap specific expensive operations:

```rust
let geometries = iced::debug::time("chart::draw", || {
    self.data_cache.draw(renderer, bounds, |frame| { /* ... */ })
});
```

`time_with` returns T only (duration goes to beacon internally):

```rust
let elem = iced::debug::time_with("subscription::drain", || {
    self.drain_market_data()
});
```

### 2.4 No Redundant Event Subscriptions

**Impact: HIGH (duplicate event handling, wasted computation, subtle bugs)**

Before adding a new `window::*` or event subscription, check whether the same event family already flows through an existing listener. Extend the existing path unless a separate subscription is required and benchmarked.

### 2.5 Press-and-Hold Input

**Impact: HIGH (hold actions fire on release instead of press)**

`button(...).on_press(...)` fires on mouse-up (release). For true mouse-down behavior (repeat scroll, press-and-hold actions), use `mouse_area(...).on_press(...)`.

### 2.6 Smoke Test After UI Changes

**Impact: HIGH (runtime panics not caught by clippy)**

Clippy catches compile errors but not runtime panics (missing Tokio runtime, wgpu init failures, font loading). Always run the app briefly after UI changes to catch these.

---

## 3. Cache & Multi-Window

**Impact: HIGH**

Rules for managing cached/mirrored UI state across panes and windows. Stale caches cause visible bugs that are hard to reproduce.

### 3.1 Trace Staleness Before Coding

**Impact: HIGH (stale caches cause visible bugs that are hard to reproduce)**

When adding cached or mirrored UI state (snapshots, summaries, registries), enumerate every mutation path that can stale it before writing code: direct handlers, drag/drop helpers, transfer/split, open/close, reset, and foreign-window events.

### 3.2 Extend Existing Event Paths

**Impact: HIGH (parallel subscriptions cause duplicate handling and ordering bugs)**

When changing window lifecycle handling, prefer extending the existing global event path over adding parallel subscriptions for the same event family.

### 3.3 Regression Tests for Invalidation

**Impact: HIGH (invalidation bugs reintroduced silently)**

Add at least one regression test for each non-obvious cache invalidation or source-window gate you introduce.

---

## 4. Elm Architecture

**Impact: MEDIUM**

Structural patterns for organizing Iced Elm Architecture applications. Violations cause coupling, bloated root modules, and difficult-to-test code.

### 4.1 Message and State Stay in Root

**Impact: MEDIUM (coupling and import cycles across modules)**

Message enum and State struct stay in the root module. Extracted modules receive `&State` or `&mut State` references. Never split these across files. Root keeps: State, Message, boot/new/update/subscription/view dispatch, thin multi-subsystem accessors.

### 4.2 Module Extraction Pattern

**Impact: MEDIUM (bloated root module, hard-to-test code)**

Extract when: feature-gated and self-contained, OR cohesive responsibility group, OR >30 lines on a well-defined State subset. Module pattern: `impl State` block with doc comment, `crate::` imports, `pub(crate)` methods. Feature gates move with the function — if all functions share a gate, apply it to the `mod` declaration.

---

## 5. Interaction

**Impact: MEDIUM**

Gotchas with mouse areas, overlays, and drag/drop that cause events to silently stop working.

### 5.1 Overlay Starvation

**Impact: MEDIUM (drag targets silently stop receiving events)**

Stacked `mouse_area(...).interaction(...)` layers can stop underlying hover/move handlers from receiving events, even without `opaque(...)`. Prefer setting `Interaction::Grabbing` on the real drag target widgets instead of adding a global cursor layer. Use `opaque(...)` only for true capture zones (app-edge drop zones).

### 5.2 Keep PaneGrid Drag Feedback Internal

**Impact: MEDIUM (native Dropped events never arrive)**

If pane dragging uses `pane_grid.on_drag(...)`, keep feedback inside the picked pane subtree or `pane_grid::Style`. `mouse_area`/`opaque` pane-drag overlays are rebuild-sensitive and can prevent native `Dropped` events from arriving.

---

## 6. Chart Rendering

Hybrid Canvas + Shader architecture for high-performance chart rendering.

Two widget types per chart, layered via `stack![]`:

- **Shader widget**: Candlestick bodies, wicks, volume bars — instanced wgpu rendering
- **Canvas widget**: Axes, gridlines, indicators, crosshair — cached geometry

### Triple-Cache Pattern

Three independent `canvas::Cache` instances per chart:

| Cache | Content | Invalidation |
|-------|---------|-------------|
| Frame cache | Axes, gridlines, price scale | Resize or axis range change |
| Data cache | Indicator lines (SMA, EMA, Bollinger) | New data or viewport scroll/zoom |
| Overlay | Crosshair, cursor price, tooltip | Fresh every frame (no cache) |

```rust
struct ChartCanvas {
    frame_cache: canvas::Cache,
    data_cache: canvas::Cache,
    // overlay is drawn fresh each frame
}
```

### Shader Instanced Rendering

```rust
struct CandleGpuData {  // 32 bytes, repr(C) for wgpu buffer layout
    open: f32, high: f32, low: f32, close: f32,
    x: f32, width: f32,
    color: u32, _pad: u32,
}
```

Single instanced draw call: `draw(0..6, 0..N)` for N candles. Handles 5,000+ candles at sub-millisecond GPU cost.

### LOD Pre-computation

Four tiers pre-computed on data arrival (not on-demand):
- 1x (raw), 10x, 100x, 1000x (MinMaxFirstLast downsampled)

Viewport changes synchronously select tier + slice visible range in <100us.

### Data Access

`Arc<ArcSwap<Vec<Bar>>>` for lock-free read access from render thread. Producer atomically swaps; renderer always sees consistent snapshot.

---

## 7. Subscription-Based Data Streams

### One Subscription Per Data Source

```rust
fn data_stream(id: &SourceId) -> impl Stream<Item = (SourceId, DataBatch)> {
    // Build with iced::stream::channel and batch inside the stream.
}

fn subscription(&self) -> Subscription<Message> {
    Subscription::batch(
        self.sources.iter().map(|source| {
            Subscription::run_with(source.id, data_stream)
                .map(Message::DataReceived)
        })
    )
}
```

Each source's subscription runs independently on Iced's executor. Use `run_with` or `.with(id)` for stable identity when a source has its own receiver.

### Batch Processing

Iced batches all pending messages before calling `view()`, but high-frequency data should still pre-aggregate in the subscription worker. Emit one batch per non-empty ~16ms window so `update()` sees bounded work and idle windows cause no redraw.

### Channel Backpressure

- Use bounded channels with `try_send()` only on the producer side
- Empty batch windows must emit no message
- Full channels drop and count on the producer side; the UI must not push back on data sources

### Performance Targets

| Metric | Target |
|--------|--------|
| Channel push | <200ns P50 |
| Subscription drain (batch) | <500us P50 |
| update() per message | <200us P50 |

---

## 8. Theming

### Custom Theme via Extended Palette

```rust
Theme::custom_with_fn("My Dark Theme", palette, |palette| {
    theme::palette::Extended::generate(palette)
})
```

### Palette Mapping

Iced's built-in palette provides `primary`, `success`, `danger`, `warning`. Map your domain tokens to these slots:

| Iced Palette Slot | Example Domain Token |
|-------------------|---------------------|
| `palette.primary` | Accent / brand color |
| `palette.success` | Positive state |
| `palette.danger` | Negative state |
| `palette.warning` | Warning state |

Colors without a palette slot (elevation, spacing, muted, border, highlight) go in a sidecar.

### Custom Tokens via LazyLock Sidecar

Iced's `Theme::Custom` has no mechanism to attach custom data. A `LazyLock` global static avoids the 15-20 Catalog trait implementations required by a custom Theme type.

```rust
use std::sync::LazyLock;

pub struct AppTokens {
    pub surface: [Color; 5],
    pub text_primary: Color,
    pub text_secondary: Color,
    pub border: Color,
    pub border_focused: Color,
    pub space_xs: f32,
    pub space_sm: f32,
    pub space_md: f32,
    pub space_lg: f32,
}

pub static TOKENS: LazyLock<AppTokens> = LazyLock::new(|| { /* ... */ });
```

Style closures access `TOKENS` directly — no parameter threading needed:

```rust
pub fn panel_container(_theme: &iced::Theme) -> container::Style {
    let t = &*TOKENS;
    container::Style {
        background: Some(iced::Background::Color(t.surface[1])),
        border: iced::Border { color: t.border, width: 1.0, radius: 4.0.into() },
        ..Default::default()
    }
}
```

**Migration path**: If the product later requires user-selectable themes, migrate to a custom Theme type. Style functions remain identical — only the token source changes.

### Font Loading

Bundle the font via the application builder, then reference it with `Font::MONOSPACE`:

```rust
iced::daemon(boot, State::update, State::view)
    .font(include_bytes!("../fonts/JetBrainsMono-Regular.ttf"))
```

`Font::MONOSPACE` resolves to the first loaded monospace font. For system-installed fonts, use `Font::with_name("JetBrains Mono")`. cosmic-text supports mixed font families via `Shaping::Advanced`.

---

## 9. Iced 0.14 API Reference

### Breaking Changes from 0.13

- `Widget::update` takes `Event` by reference
- `Shrink` prioritized over `Fill` in layout; entry point is `iced::daemon(boot, update, view)` for multi-window, or `iced::application(new, update, view)` for single-window
- Theme palette uses Oklch; keyboard subscriptions unified into `keyboard::listen`

### Key Widgets

| Widget | Use | Hard Rule |
|--------|-----|-----------|
| `sensor` | Layout dimensions via `on_show` (initial) + `on_resize` (changes). Use instead of `responsive`. | HR-3 |
| `float` | Content rendered via overlay system (above siblings). Use for tooltips. | — |
| `pin` | Absolute positioning within parent bounds. Use for drag ghosts, context menus. | — |
| `stack` | Z-order layering within widget tree. Use for overlay composition. | HR-4 |
| `table` | Data tables | — |
| `scrollable` | `on_scroll` only fires on user scroll, NOT initial render/resize. | HR-3 |
| `mouse_area` | `on_press` fires on mouse-down. Use for drag initiation, hold actions. | — |
| `button` | `on_press` fires on mouse-up (release). Standard activate-on-release. | — |

### PaneGrid

- **Event capture**: Both `button` and `mouse_area` call `capture_event()` on press. Tab elements capture → custom tab drag. Empty title bar → pane_grid native drag. Coexist via pick area geometry.
- **Tab drag** (custom, resilient to rebuilds): `mouse_area.on_press` per tab + `listen_with` subscription for `CursorMoved`/`ButtonReleased`. State machine: Idle → Pressed(origin) → Dragging (8px threshold). App-level state.

### Debug

`features = ["debug"]` + F12 at runtime. `iced::debug::time_with("label", || { ... })`. Stress: `ICED_PRESENT_MODE=Immediate` + `unconditional-rendering`.

### Multi-Window

`window::open(settings) -> Task<window::Id>`, `window::close(id)`. `view()` receives `window::Id`.

### Testing

`iced_test`: `Simulator` (headless widget), `Emulator` (full runtime), snapshot support.
