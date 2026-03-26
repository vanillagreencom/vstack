# Rust FFI

Safe and correct patterns for Rust Foreign Function Interface boundaries.

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
   - `data-` for String & Data Handling (Section 1)
   - `gen-` for Bindgen & Cbindgen (Section 2)
   - `wrap-` for Safe Wrappers (Section 3)
3. Fill in the frontmatter and content
4. Add the rule to the Quick Reference in `SKILL.md`
5. Add the expanded rule to the appropriate section in `AGENTS.md`

## Impact Levels

- `CRITICAL` - Causes undefined behavior, memory corruption, or unsoundness if violated
- `HIGH` - Causes bugs, API misuse, or architectural problems if violated
- `MEDIUM` - Reduces consistency or wastes time; review friction
