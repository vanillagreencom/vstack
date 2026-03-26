# eBPF/Aya Observability

Kernel-level observability with eBPF and the Aya framework in Rust.

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
   - `aya-` for Aya Framework (Section 1)
   - `trace-` for bpftrace Diagnostics (Section 2)
   - `debug-` for Verifier & Debugging (Section 3)
3. Fill in the frontmatter and content
4. Add the rule to the Quick Reference in `SKILL.md`
5. Add the expanded rule to the appropriate section in `AGENTS.md`

## Impact Levels

- `HIGH` - Causes bugs, verifier rejections, or production incidents if violated
- `MEDIUM` - Reduces debuggability or wastes time; harder to diagnose issues
