# Rust Cross-Compilation

Target configuration, cross-compilation tools, static binary recipes, and multi-platform CI for Rust projects.

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
   - `target-` for Target Configuration (Section 1)
   - `tool-` for Cross-Compilation Tools (Section 2)
   - `static-` for Static Binaries (Section 3)
   - `ci-` for Testing & CI (Section 4)
3. Fill in the frontmatter and content
4. Add the rule to the Quick Reference in `SKILL.md`
5. Add the expanded rule to the appropriate section in `AGENTS.md`

## Impact Levels

- `HIGH` - Causes build failures, broken binaries, or deployment issues if violated
- `MEDIUM` - Reduces CI reliability or wastes time; testing friction
