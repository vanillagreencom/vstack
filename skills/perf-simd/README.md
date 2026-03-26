# Perf SIMD

SIMD intrinsics, auto-vectorization, and runtime dispatch for Rust hot paths.

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
   - `auto-` for Auto-Vectorization (Section 1)
   - `simd-` for Manual SIMD (Section 2)
   - `portable-` for Portable SIMD (Section 3)
3. Fill in the frontmatter and content
4. Add the rule to the Quick Reference in `SKILL.md`
5. Add the expanded rule to the appropriate section in `AGENTS.md`

## Impact Levels

- `HIGH` - Causes bugs, performance regressions, or crashes on unsupported hardware if violated
- `MEDIUM` - Reduces portability or misses optimization opportunities; review friction
