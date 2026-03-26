# Cargo Workflows & Build Optimization

Workspace management, build tooling, compilation performance, and release/CI configuration for Rust projects.

## Structure

- `rules/` - Individual rule files (one per rule)
  - `_sections.md` - Section metadata (titles, impacts, descriptions)
  - `_template.md` - Template for creating new rules
  - `prefix-description.md` - Individual rule files
- **`SKILL.md`** - Quick-reference index for skill-aware harnesses
- **`AGENTS.md`** - Full compiled document for all harnesses

## Creating a New Rule

1. Copy `rules/_template.md` to `rules/prefix-description.md`
2. Choose the appropriate area prefix:
   - `ws-` for Workspace Management (Section 1)
   - `tool-` for Build Tooling (Section 2)
   - `perf-` for Build Performance (Section 3)
   - `ci-` for Release & CI (Section 4)
3. Fill in the frontmatter and content
4. Add the rule to the Quick Reference in `SKILL.md`
5. Add the expanded rule to the appropriate section in `AGENTS.md`

## Impact Levels

- `HIGH` - Causes bugs, CI failures, or architectural drift if violated
- `MEDIUM` - Reduces consistency or wastes time; review friction
