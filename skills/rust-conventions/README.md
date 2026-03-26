# Rust Conventions

Style, testing, structure, and completeness rules for Rust codebases.

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
   - `style-` for Style & Formatting (Section 1)
   - `struct-` for Code Structure (Section 2)
   - `test-` for Testing (Section 3)
   - `complete-` for Completeness (Section 4)
   - `nav-` for Navigation (Section 5)
   - `gotcha-` for Gotchas (Section 6)
3. Fill in the frontmatter and content
4. Add the rule to the Quick Reference in `SKILL.md`
5. Add the expanded rule to the appropriate section in `AGENTS.md`

## Impact Levels

- `HIGH` - Causes bugs, CI failures, or architectural drift if violated
- `MEDIUM` - Reduces consistency or wastes time; review friction
