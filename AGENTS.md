# vstack

Cross-harness distribution system for AI coding skills, agents, and hooks. Installs into Claude Code, Cursor, OpenCode, and Codex via a Rust CLI.

## Repo Layout

```
cli/src/
├── main.rs              CLI entry, clap subcommands (add, remove, list, init, check)
├── agent.rs             Agent parsing, skill/hook matching heuristics
├── skill.rs             Skill parsing, frontmatter dependency resolution
├── hook.rs              Hook parsing (YAML-in-comments frontmatter from .sh files)
├── config.rs            Lock file (JSON), project root detection, global/project scope
├── mapping.rs           vstack.toml config loader — skill/hook-to-agent mappings
├── installer.rs         Symlink/copy logic, per-harness hook installation, removal
├── harness/
│   ├── mod.rs           Harness enum, detection, routing
│   ├── claude.rs        → .claude/agents/*.md (skills + hooks frontmatter, "Load These Skills")
│   ├── cursor.rs        → .cursor/rules/*.mdc (description + alwaysApply + skills section)
│   ├── opencode.rs      → .opencode/agents/*.md (YAML frontmatter + skills section)
│   └── codex.rs         → .codex/agents/*.toml (developer_instructions + skills section)
└── tui/
    ├── mod.rs           3-step install flow, Installed tab, removal, update confirmation
    ├── multiselect.rs   Selection state, scroll, toggle, confirm dialog
    └── render.rs        Ratatui rendering (header, list, status, help bar, dialog overlay)

vstack.toml              Skill/hook-to-agent mapping config (read by CLI at install time)
agents/                  12 canonical agents — role field drives per-harness access control
skills/                  29 skill packages — each has SKILL.md with optional dependencies
hooks/                   6 safety hooks — bash scripts with YAML comment headers
skill-templates/         Templates for new skills
```

## Key Design Decisions

- **Everything is discovered dynamically.** The CLI scans `agents/`, `skills/`, `hooks/` at runtime. No hardcoded lists.
- **Canonical source is harness-agnostic.** Agents, skills, and hooks contain no harness-specific syntax. Translation happens in `cli/src/harness/`.
- **Agent `role` drives access control.** `reviewer` → read-only/subagent. `engineer` → full access/primary. `manager` → analysis only.
- **Skill dependencies use frontmatter.** `dependencies: { required: [...], optional: [...] }` in SKILL.md.
- **Hooks diverge by harness.** Claude Code gets native shell hooks + settings.json + agent frontmatter. Cursor gets safety `.mdc` rules. OpenCode gets `.opencode/agents/*.md` + instructions. Codex gets inline prose in `developer_instructions`.
- **Skill/hook attribution is config-driven.** `vstack.toml` maps skills to agents (by name prefix + explicit overrides) and hooks to agents (by event type + role). Agents get a `skills:` frontmatter field and a "Load These Skills" body section.
- **Reconciliation is automatic.** After every `vstack add`, all installed agents are regenerated with the current full set of installed skills and hooks. Adding a skill after an agent updates that agent.
- **Project root walks up from CWD.** `config::project_root()` finds `.vstack-lock.json`, `.claude/`, `.cursor/`, `.codex/`, `.opencode/`, or `.agents/` by walking parent dirs — works from subdirectories.

## Formats

### Agent frontmatter (`agents/*.md`)
```yaml
name: rust
description: ...
model: opus          # opus | sonnet | haiku
role: engineer       # engineer | reviewer | manager
color: orange
```

### Skill frontmatter (`skills/*/SKILL.md`)
```yaml
name: orchestration
description: ...
license: MIT
user-invocable: true
dependencies:
  required: [linear, github, worktree]
```

### Hook header (`hooks/*.sh`)
```bash
# ---
# name: block-bare-cd
# event: PreToolUse       # PreToolUse | PostToolUse | PostCompact | TaskCompleted
# matcher: Bash           # Bash | Edit|Write | (empty for all)
# description: ...
# safety: ...
# timeout: 30             # optional, seconds
# ---
```

### Mapping config (`vstack.toml`)
```toml
[agent-skills]
iced = ["iced-rs", "trading-design", "price-handling"]

[role-skills]
engineer = ["issue-lifecycle", "github", "worktree"]
reviewer = ["issue-lifecycle"]

[hook-events]
"PreToolUse:Bash" = "all"
"PostToolUse:Edit|Write" = ["engineer"]
"PostCompact:" = "all"
```

### Project customization (`vstack.toml` at project root)

The same `vstack.toml` (or a separate one in the target project) can include per-agent customization sections. These survive `vstack add` updates — they are re-applied from config on every install/reconciliation.

```toml
# Attach local skills to agents (name + description shown in "Load These Skills")
[custom-skills]
rust = [
  { name = "my-testing", description = "Custom integration testing patterns" },
]

# "When to Use" guidance injected after the agent intro
[agent-guidance]
rust = "Use when working on backend Rust services and performance-critical code paths."

# Additional instructions appended at the bottom of the agent file
[agent-instructions]
rust = """
Always run clippy before committing.
Prefer zero-copy APIs in hot paths.
"""
```

## Per-Harness Model Mapping

| Canonical | Claude Code | OpenCode | Codex |
|-----------|-------------|----------|-------|
| `opus` | `opus[1m]` | `openai/gpt-5.4` | `gpt-5.4` (xhigh) |
| `sonnet` | `sonnet` | `openai/gpt-5.4` | `gpt-5.4` (high) |
| `haiku` | `haiku` | `openai/gpt-5.4` | `gpt-5.4` (medium) |

## Rules

- **No project-specific references.** Zero mentions of specific apps, crate names, paths, or tools in `agents/`, `skills/`, `hooks/`.
- **Validate ctx7 IDs.** Every library ID in SKILL.md ctx7 tables must resolve via `npx ctx7@latest docs <id> "test"`.
- **Test after CLI changes.** `cd cli && cargo test` for unit tests. For integration: `cargo run -- add .. --all --copy` into a temp dir.
- **Hooks must be portable.** No hardcoded paths. Scripts should work in any Rust project (or degrade gracefully).
- **Child workflows return JSON to parent.** Subagent workflows (project-management, issue-lifecycle) output JSON in `<output_format>` tags — the calling primary agent writes files.
- **Never bump the CLI version.** The version in `cli/Cargo.toml` and the matching GitHub release tag are managed manually by the user. Do not change the version or create releases unless explicitly asked.

## Build & Test

```bash
cd cli && cargo build                    # build
cd cli && cargo test                     # 21 unit tests
cd cli && cargo run -- add .. --all -y   # integration test against this repo
```
