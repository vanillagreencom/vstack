# Rust no_std Development

Guidelines for `#![no_std]` Rust development covering environment tiers, runtime handlers, portable library design, embedded patterns, and testing strategies.

## Structure

```
rust-no-std/
├── SKILL.md          # Quick-reference index for Claude Code and skill-aware harnesses
├── AGENTS.md         # Full compiled document for Codex, Copilot, Gemini CLI, etc.
├── README.md         # This file — human-facing documentation
└── rules/
    ├── _sections.md  # Section metadata (impact, description, prefix)
    ├── _template.md  # Template for creating new rules
    ├── env-*.md      # Environment Tiers rules (CRITICAL)
    ├── rt-*.md       # Panic & Allocator rules (CRITICAL)
    ├── lib-*.md      # Portable Library Design rules (HIGH)
    ├── embed-*.md    # Embedded Patterns rules (HIGH)
    └── test-*.md     # Testing rules (MEDIUM)
```

## Creating a New Rule

1. Copy `rules/_template.md` to `rules/{prefix}-{slug}.md`
2. Fill in the YAML frontmatter: `title`, `impact`, `tags`
3. Write the rule body with incorrect/correct examples
4. Add the rule to the appropriate section in `SKILL.md` (Quick Reference)
5. Add the full expanded content to `AGENTS.md`

Naming convention: `{section-prefix}-{kebab-case-slug}.md`

Examples:
- `env-core-alloc-std.md` (Environment Tiers section, prefix `env`)
- `rt-panic-handler.md` (Panic & Allocator section, prefix `rt`)
- `lib-feature-gated-std.md` (Portable Library Design section, prefix `lib`)

## Impact Levels

| Level | Meaning |
|-------|---------|
| **CRITICAL** | Violating this rule causes build failures, undefined behavior, or hardware faults. Must always be followed. |
| **HIGH** | Violating this rule causes significant portability, correctness, or maintainability issues. Follow unless there is a documented reason. |
| **MEDIUM** | Best practice that improves code quality, testability, or developer experience. Follow when practical. |
