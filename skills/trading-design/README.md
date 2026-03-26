# Professional Trading UI Design

Stack-agnostic design principles for professional trading applications. Focused on density, signal-through-noise, modular panel architecture, and two-hue directional color. Does not define specific tokens, colors, or pixel values — those belong in your design system. This defines the principles, philosophy, and patterns your design system should embody.

## Structure

- `rules/` - Individual principle files (one per topic)
  - `_sections.md` - Section metadata (titles, impacts, descriptions)
  - `_template.md` - Template for creating new rules
  - `prefix-description.md` - Individual principle files
- **`SKILL.md`** - Quick-reference index for skill-aware harnesses
- **`AGENTS.md`** - Full compiled document for all harnesses

## Sections

| # | Section | Impact | Prefix | Topics |
|---|---------|--------|--------|--------|
| 1 | Design Philosophy | CRITICAL | `phil-` | Identity, density, signal/noise |
| 2 | Visual Language | CRITICAL | `vis-` | Color theory, opacity, dark-primary, elevation |
| 3 | Typography & Density | CRITICAL | `type-` | Dual-font, alignment, semantic tokens |
| 4 | Layout & Panels | HIGH | `layout-` | Docking, states, responsive collapse |
| 5 | Data Display | HIGH | `data-` | Prices, positions, orders, P&L, alerts |
| 6 | Interaction Design | HIGH | `ix-` | Keyboard-first, error prevention |
| 7 | Component Philosophy | MEDIUM | `comp-` | Composed vs custom, widget patterns |
| 8 | Accessibility | MEDIUM | `a11y-` | Contrast, focus, cross-platform |

## Creating a New Rule

1. Copy `rules/_template.md` to `rules/prefix-description.md`
2. Choose the appropriate prefix from the sections table
3. Fill in the frontmatter and content
4. Ensure content is stack-agnostic — no framework code, no concrete token values, explain *why* and *what* not *how* to implement
5. Add the rule to the Quick Reference in `SKILL.md`
6. Add the expanded content to the appropriate section in `AGENTS.md`

## Rule File Structure

```markdown
---
title: Rule Title Here
impact: MEDIUM
impactDescription: Optional description
tags: tag1, tag2
---

## Rule Title Here

**Impact: MEDIUM (optional impact description)**

Principle explanation, guidance, and examples. All content must be stack-agnostic.
```

## File Naming Convention

- Files starting with `_` are special (excluded from build)
- Rule files: `prefix-description.md` (e.g., `phil-density-first.md`)
- Section is inferred from filename prefix
- Rules are sorted alphabetically by title within each section

## Impact Levels

- `CRITICAL` - Foundational principles; violations break visual identity or mislead traders
- `HIGH` - Significant usability or consistency impact; violations cause layout failures, data misreading, or unsafe interaction
- `MEDIUM` - Good practices; violations cause inconsistency or maintenance burden
