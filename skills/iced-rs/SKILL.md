---
name: iced-rs
description: Iced 0.14 UI patterns, hard rules, and development practices for high-performance reactive applications. Use when implementing Iced views, widgets, charts, pane_grid layouts, or working with the Elm Architecture.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Iced 0.14 Patterns

Patterns and rules for building high-performance UIs with Iced 0.14's reactive Elm Architecture, prioritized by impact.

## When to Apply

Reference these guidelines when:
- Implementing or modifying Iced views, widgets, or layouts
- Working with pane_grid, Canvas, Shader, or Subscription
- Building custom themes or styling components
- Debugging interaction issues (drag/drop, overlays, event routing)
- Reviewing UI code for framework constraint violations

## Nomenclature

App > Window > Shell > Zone > TitleBar > Panel > Canvas > Overlay

## Dev Tools

| Tool | Purpose | Install |
|------|---------|---------|
| `cargo-hot` | Live UI patching without restart | `cargo install cargo-hot` |
| `comet` | Iced debugger: frame metrics, widget tree, message inspector | `cargo install --locked --git https://github.com/iced-rs/comet.git` |

## Rule Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Hard Rules | CRITICAL | `hr-` |
| 2 | Development Practices | HIGH | `dev-` |
| 3 | Cache & Multi-Window | HIGH | `cache-` |
| 4 | Elm Architecture | MEDIUM | `elm-` |
| 5 | Interaction | MEDIUM | `interaction-` |

## Quick Reference

### 1. Hard Rules (CRITICAL)

- `hr-widget-tree-consistency` - Never conditionally wrap widgets; always wrap, conditionally enable
- `hr-pick-area-geometry` - TitleBar content must use Shrink width to preserve pick area
- `hr-scroll-state` - Use sensor.on_show for initial dimensions, not scrollable.on_scroll
- `hr-overlay-state-isolation` - Overlay layers must not affect base layer widget structure
- `hr-single-message` - One message per interaction; use state machines for composites
- `hr-view-is-pure` - view() is pure function of State; no side effects or hidden state
- `hr-minimum-pane-size` - PaneGrid::min_size is uniform; use container for per-pane minimums
- `hr-titlebar-event-ordering` - Title bar events fire before body events in same frame

### 2. Development Practices (HIGH)

- `dev-validate-api` - Verify API against docs; Iced 0.14 has breaking changes from 0.13
- `dev-reactive-discipline` - Never redraw from view(), invalidate caches explicitly, batch ~16ms
- `dev-instrument-budgets` - iced::debug::time on every function with a performance budget
- `dev-no-redundant-subscriptions` - Extend existing event listeners before adding new ones
- `dev-press-and-hold` - button fires on release; use mouse_area for true mouse-down
- `dev-smoke-test` - Run app after UI changes; clippy misses runtime panics

### 3. Cache & Multi-Window (HIGH)

- `cache-trace-staleness` - Enumerate every mutation path that can stale cached state
- `cache-extend-event-paths` - Extend existing global event path over parallel subscriptions
- `cache-regression-tests` - Test each non-obvious invalidation or source-window gate

### 4. Elm Architecture (MEDIUM)

- `elm-state-in-root` - Message enum and State struct stay in root module
- `elm-extraction` - Extract feature-gated, cohesive, or >30-line State subsets to modules

### 5. Interaction (MEDIUM)

- `interaction-overlay-starvation` - Cursor overlays starve underlying drag targets
- `interaction-pane-drag-feedback` - Keep pane_grid drag feedback inside pane subtree

## How to Use

Read individual rule files for detailed explanations and code examples:

```
rules/hr-widget-tree-consistency.md
rules/dev-reactive-discipline.md
```

Each rule file contains:
- Brief explanation of why it matters
- Incorrect code example with explanation
- Correct code example with explanation

## Resources

Documentation lookup order: local skill files → ctx7 CLI → web fallback.

### ctx7 CLI

| Library | ctx7 ID | Use For |
|---------|---------|---------|
| Iced 0.14 | `/websites/rs_iced_0_14_0` | Widgets, Theme, Canvas, Shader, Subscription, pane_grid |
| tokio | `/websites/rs_tokio` | Async runtime, channels, streams |
| wgpu | `/websites/rs_wgpu` | GPU rendering, shader pipelines |

### Web

| Source | URL | Use For |
|--------|-----|---------|
| Iced API docs | `https://docs.iced.rs/iced/` | API reference (tracks master — may serve unreleased APIs) |
| Iced examples | `https://github.com/iced-rs/iced/tree/master/examples` | Reference implementations |

## Full Compiled Document

For the complete guide with all rules expanded, plus chart rendering, subscriptions, theming, and API reference: `AGENTS.md`
