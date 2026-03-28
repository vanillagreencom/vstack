# Label Management Reference

## Exclusivity Rules

See project label taxonomy for full taxonomy and colors.

**Key rule**: Labels in a parent group (Agent, Platform) are exclusive — only ONE per issue. Labels without a parent (Stack, Workflow, Classification) allow multiples.

**"labelIds not exclusive child labels" error** = You tried to use multiple labels from Agent or Platform group.

## When to Create Labels

**Authorization rule**: Never create any label unprompted. All label creation requires explicit user authorization.

**Create when**:
- New agent added (requires agent definition first)
- New stack component introduced
- New workflow state needed

**Do NOT create when**:
- Existing label covers the use case
- One-off categorization (use description instead)
- No clear owner or purpose defined

## Label Ownership

| Label Type | Owner | Approval | Notes |
|------------|-------|----------|-------|
| `agent:*` | tpm | Yes | Requires project agent definition |
| Stack | tpm | Yes | Architectural change |
| Workflow | tpm | Yes | Operational, but still requires user authorization |
| Classification | tpm | Yes | Operational, but still requires user authorization |
| Platform | tpm | Yes | Architectural change |

## Creating Agent Labels

Agent labels are special — MUST have agent definition AND parent group.

### Process

1. **Create the agent definition** in project agent definitions
2. **Update** project label taxonomy
3. **tpm** creates label:
   ```bash
   $ISSUE_CLI labels create --name "agent:[NAME]" --color "#9C27B0" --parent "Agent"
   ```

**TPM should NOT create any labels unprompted** — even workflow or classification labels require explicit user authorization. `agent:*` labels additionally require the agent definition and taxonomy entry to exist first.

## Creating Other Labels

```bash
# Workflow labels (no parent - independent)
$ISSUE_CLI labels create --name "needs-[ACTION]" --color "#757575"

# Classification labels (no parent - independent)
$ISSUE_CLI labels create --name "[TYPE]" --color "#E53935"

# Stack labels (requires review)
$ISSUE_CLI labels create --name "[STACK_NAME]" --color "#FF6B35"
```

After creating, update project label taxonomy.

## Label Lifecycle

### Deprecating

1. Remove from active issues (reassign)
2. Archive in issue tracker (don't delete — preserves history)
3. Mark deprecated in taxonomy

### Renaming

Avoid renaming — creates confusion. Instead:
1. Create new label with correct name
2. Migrate issues from old to new
3. Archive old label

## Checklist: New Label

Before:
- [ ] No existing label covers this
- [ ] Determined parent group (exclusive) or none (independent)
- [ ] Color consistent with category
- [ ] Explicit user authorization obtained

After:
- [ ] Label created in issue tracker
- [ ] Taxonomy updated
- [ ] Announced in handoff/comment
