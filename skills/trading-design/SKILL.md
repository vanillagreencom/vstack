---
name: trading-design
description: Professional trading UI design principles — density, color theory, panel architecture, data display conventions, and component philosophy. Stack-agnostic guide for building interfaces that prioritize signal through noise. Use when designing, implementing, or reviewing trading UI.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "2.0.0"
---

# Professional Trading UI Design

Stack-agnostic design principles for professional trading applications. Sierra Chart density + Vercel dark refinement + ShadCN component clarity. Dark-primary canvas, two-hue directional color, opacity-driven depth. Every pixel earns its place.

## When to Apply

Reference these guidelines when:
- Designing or implementing trading UI panels and layouts
- Choosing colors, spacing, typography, or elevation approaches
- Displaying prices, P&L, positions, orders, or alerts
- Building trading-specific components or widgets
- Reviewing design consistency, density, or accessibility
- Making architectural decisions about panel systems or component approaches

## Rule Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Design Philosophy | CRITICAL | `phil-` |
| 2 | Visual Language | CRITICAL | `vis-` |
| 3 | Typography & Density | CRITICAL | `type-` |
| 4 | Layout & Panels | HIGH | `layout-` |
| 5 | Data Display | HIGH | `data-` |
| 6 | Interaction Design | HIGH | `ix-` |
| 7 | Component Philosophy | MEDIUM | `comp-` |
| 8 | Accessibility | MEDIUM | `a11y-` |

## Quick Reference

### 1. Design Philosophy (CRITICAL)

- `phil-identity` — Target aesthetic (Sierra Chart + Vercel + ShadCN), reference platforms, anti-patterns to avoid
- `phil-density-first` — Density as primary constraint, pixel accountability, benchmarks, information hierarchy through density
- `phil-signal-noise` — Signal-through-noise ratio, noise budgets, earned pixels, animation philosophy

### 2. Visual Language (CRITICAL)

- `vis-two-hue-system` — Two chromatic hues for direction only; all other variation from neutral at graduated opacities
- `vis-opacity-depth` — Opacity as the primary visual variable; one hue many roles; consistency rule
- `vis-near-black-foundation` — Dark-primary canvas philosophy; data prominence; surface layering; theming beyond dark
- `vis-surface-elevation` — 5-level elevation ladder; no shadows or gradients; elevation in context

### 3. Typography & Density (CRITICAL)

- `type-dual-font` — Sans for chrome, monospace for all data; tabular/lining figures; the sans/mono boundary
- `type-alignment-hierarchy` — Decimal alignment, size hierarchy, column layout principles
- `type-semantic-tokens` — Token architecture, naming conventions, enforcement as code quality practice

### 4. Layout & Panels (HIGH)

- `layout-panel-architecture` — Tiling not floating, priority-based allocation, shell structure, panel composition
- `layout-panel-states` — Five required states (loading, empty, error, disconnected, data); disconnected state safety
- `layout-responsive` — Collapse sequence, priority ordering, minimum panel sizes

### 5. Data Display (HIGH)

- `data-display-conventions` — Price, position, order, P&L display patterns; alerts; stale data; confirmation dialogs

### 6. Interaction Design (HIGH)

- `ix-keyboard-first` — Keyboard shortcuts for all actions, focus management, tooltip requirements
- `ix-error-prevention` — Confirmation flows, prevention over confirmation, validation, recovery

### 7. Component Philosophy (MEDIUM)

- `comp-design-approach` — Composed vs custom rendering; ShadCN model compressed; trading widget patterns; component checklist

### 8. Accessibility (MEDIUM)

- `a11y-requirements` — Contrast ratios, never-color-alone, focus indicators, cross-platform rendering

## How to Use

Read individual rule files for detailed principles and guidance:

```
rules/phil-density-first.md
rules/vis-two-hue-system.md
rules/data-display-conventions.md
```

Each rule file contains:
- The principle and why it matters
- Detailed guidance with tables and examples
- Anti-patterns to avoid

## Resources

### Web

| Source | URL | Use For |
|--------|-----|---------|
| WCAG contrast | `https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum.html` | Accessibility contrast ratios |

## Full Compiled Document

For the complete guide with all rules expanded: `AGENTS.md`
