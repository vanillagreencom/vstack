# Rust Async Patterns

Async runtime internals, concurrency composition, and task management patterns for Rust.

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
   - `future-` for Future & Poll Model (Section 1)
   - `tokio-` for Tokio Runtime (Section 2)
   - `compose-` for Select & Join (Section 3)
   - `pat-` for Async Patterns (Section 4)
3. Fill in the frontmatter and content
4. Add the rule to the Quick Reference in `SKILL.md`
5. Add the expanded rule to the appropriate section in `AGENTS.md`

## Impact Levels

- `CRITICAL` - Runtime panics, deadlocks, data loss, or silent correctness bugs if violated
- `HIGH` - Performance degradation, resource leaks, or architectural problems
- `MEDIUM` - Debugging difficulty, inconsistency, or wasted developer time
