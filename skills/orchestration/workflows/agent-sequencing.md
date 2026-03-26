# Agent Sequencing

Determines blocking relations and delegation order for cross-domain issues. Existing blocking relations on issues take precedence over these rules.

## How to Apply

1. **Infer agent** from label or component location. Each project defines its own agent-to-path mappings (e.g., `backend/` → backend agent, `frontend/` → frontend agent, `docs/` → docs agent).
2. **Identify candidate pairs** from the sequential requirements table below.
3. **Confirm with Creates ↔ Consumes**: Only set blocking if agent A creates types, APIs, or modules that agent B consumes. No data flow = no blocking, regardless of agent ordering.
4. **Same-project only**: Use `blocks`/`blocked_by` within a project. Use `related` cross-project.
5. **Set relations on parents**, not children, when issues are bundled.

## Default Sequential Requirements

| Pattern | Why |
|---------|-----|
| Data producer → Data consumer (if data dependency) | Consumers need types/APIs from producers first |
| `*` → Generalist (runs last) | May reference changes from any domain |

## Default Parallel Candidates

Agents operating on independent code paths with no shared data can run in parallel. Common safe pairs:

| Can Parallel | Why |
|--------------|-----|
| Backend + Frontend (no shared types) | Independent output paths |
| Backend + Design/Docs | No code dependency |
| Frontend + Design/Docs | Implementation + spec |

## Parallel Safety Checks

Before parallel execution, confirm:
- No shared file modifications between agents
- No type/value flow between agent domains
- No manifest/config dependency conflicts (e.g., both modifying Cargo.toml, package.json)
- No existing worktrees or open PRs on overlapping code
