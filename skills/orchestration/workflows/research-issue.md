# Research Issue Workflow

> **Dependencies**: `$ISSUE_CLI`, `$DECISIONS_CMD` (decider skill, optional), `scripts/workflow-state`

Create research issue in issue tracker and prepare assets for human execution.

## Inputs

| Context | Source | Required |
|---------|--------|----------|
| `topic` | Caller (start § 3.3 or research-spike § 1.1) | Yes |
| `questions` | Caller (agent consultation or user input) | Yes |
| `domains` | Caller (list of domain labels) | Yes |
| `project` | Caller or query | Yes |
| `blocked_issue` | Caller (start § 2) | No (spikes have none) |
| `type` | Caller (Targeted/Pervasive/Strategic) | Yes |
| `prior_research` | Caller (research-spike § 2.2 findings) | No |
| `consultation_agent_name` | Caller (start § 3.3 agent name) | No |
| `research_paths` | Caller (project research docs paths) | No |
| `decision_ids` | Caller (DXXX references) | No |
| `batch_issues` | Caller (list of per-issue context: {topic, questions, domains, blocked_issue, type, consultation_agent_name, research_paths, decision_ids}) | No |

When `batch_issues` is set, single-issue fields are ignored. `project` is shared across all entries.

## 1. Create Issue

### 1.1 Batch Mode

**Skip if** `batch_issues` not set → use single-issue fields

For each entry in `batch_issues`, run § 1.2-1.4 with that entry's fields. Collect `[RESEARCH_ISSUE_ID]` per entry. After loop → § 2 with all collected IDs.

### 1.2 Validate Inputs

Confirm all required variables are set. If [TYPE] not provided, determine from [DOMAINS]:

| Domain Count | Type |
|--------------|------|
| 1 | Targeted |
| 2+ | Pervasive |

Strategic requires explicit caller designation (initiative-level scope).

### 1.3 Create Issue

Create issue using input variables.

```bash
$ISSUE_CLI issues create \
  --title "Research: [TOPIC]" \
  --project "[PROJECT]" \
  --labels "agent:human,research,[DOMAINS]" \
  --priority 2 \
  --estimate 1 \
  --description "[DESCRIPTION]"
```

**[DESCRIPTION]** template:
```
## Summary
[1-2 sentence summary of TOPIC]

## Questions
[QUESTIONS]
[TYPE_SECTION]
## Expected Decision
Next available DXXX via `$DECISIONS_CMD next-id` (decider skill)
```

**[TYPE_SECTION]** — insert based on [TYPE]:

| Type | Section |
|------|---------|
| Targeted | (omit) |
| Pervasive | `## Affected Domains` with domain list and reasons |
| Strategic | `## Creates Roadmap` with scope and phases |

Capture returned identifier as `[RESEARCH_ISSUE_ID]`.

### 1.4 Add Blocking Relation (if applicable)

**Skip if** [BLOCKED_ISSUE_ID] not set (self-initiated spike).

Blocking relations are managed via the issue tracker's relation system -- never via description text.

```bash
$ISSUE_CLI issues add-relation [RESEARCH_ISSUE_ID] --blocks [BLOCKED_ISSUE_ID]
```

CLI enforces same-project constraint for blocking relations.

Asset paths added in § 3 after preparation.

---

## 2. Prepare Assets

Gather domain-specific context for the research prompt.

### 2.1 Batch Mode

**Skip if** single issue

Apply § 2.2 mapping per issue. Spawn § 2.3 consultations for ALL issues in parallel (one sub-agent per {issue, domain} pair). Collect results, then run § 2.4-2.5 per issue sequentially. After all → § 3.

### 2.2 Map Domain Agents

Map domain labels to agent types -- infer from component paths (project-configurable).

### 2.3 Consult Domain Agents (Parallel)

**Purpose**: Asset preparation — gather context for research prompt. NOT impact analysis (that happens in research-complete § 5).

**Delegate to each domain agent** from § 2.2 (parallel sub-agent calls).

#### If `consultation_agent_name` set (from start § 3.3)

Send message to existing agent: [CONSULTATION_AGENT_NAME]. Agent's full context is preserved from the initial consultation. Follow exactly, fill placeholders, add nothing else. Omit lines/sections with empty placeholders.

<delegation_format>
Research issue created: [RESEARCH_ISSUE_ID] - [TOPIC]

Draft your domain's contribution for the research assets:
1. Precise questions from your domain perspective
2. Context to extract from your docs (inline, no external refs)
3. Scope constraints from your expertise
4. Relevant prior decisions or patterns

Reply with structured sections for each item.
</delegation_format>

#### If no `consultation_agent_name` (spike, multi-domain, or agent terminated)

Start fresh with full context. **Delegation prompt:** Follow exactly, fill placeholders, add nothing else. Omit lines/sections with empty placeholders.

<delegation_format>
Research: [RESEARCH_ISSUE_ID] - [TOPIC]

Blocked Issue: [BLOCKED_ISSUE_ID]
Read blocked issue context: `$ISSUE_CLI cache issues get [BLOCKED_ISSUE_ID]`

Read: [RESEARCH_PATHS]
Read: [project decision documents]/INDEX.md
Read: [project decision documents]/[DECISION_ID]-*.md
Prior findings (inline):
[PRIOR_RESEARCH]

Read relevant architecture docs and in-project code for context.

Draft your domain's contribution:
1. Precise questions from your domain perspective
2. Context to extract from your docs (inline, no external refs)
3. Scope constraints from your expertise
4. Relevant prior decisions or patterns

Reply with structured sections for each item.
</delegation_format>

### 2.4 Assemble Assets

Create project research docs directory for `[RESEARCH_ISSUE_ID]/`:

**prompt.txt** - Merge agent questions into structured prompt:
- Research objective (1 sentence)
- Context summary (2-3 sentences)
- Attached files list with descriptions
- Questions (prioritized, merged from agents)
- Scope constraints
- Deliverables

**context-{topic}.md** - From agent extractions:
- Self-contained (no external references, no issue IDs, no file paths)
- Include prior research findings inline if referenced
- Extract relevant architecture content directly
- Ensure external research agent has all necessary context in regards to the research prompt. If there is anything they need to know to execute the research effectively, add it.

### 2.5 Validate Self-Containment

| Bad | Good |
|-----|------|
| "See docs/architecture/module.md" | Extract content into context file |
| "Reference [ISSUE_ID] findings" | Include findings inline |
| "per project rules" | State the rule directly |
| "Message Bus Design (D001)" | "Message Bus Design" |

---

## 3. Complete

Assets complete. Update issue description with asset paths, then set state to Todo.

**If batch**: Repeat per issue. After all: "Research assets ready for [ID1], [ID2], ...".

1. **Get current description**: `$ISSUE_CLI cache issues get [RESEARCH_ISSUE_ID] | jq -r '.description'`

2. **Append to description**:
   ```
   ## Assets
   - [RESEARCH_DOCS_PATH]/[RESEARCH_ISSUE_ID]/prompt.txt
   - [RESEARCH_DOCS_PATH]/[RESEARCH_ISSUE_ID]/context-*.md

   ## Output
   Save findings to: [RESEARCH_DOCS_PATH]/[RESEARCH_ISSUE_ID]/findings.md

   ## Completion
   `/research-complete [RESEARCH_ISSUE_ID]`
   ```

3. **Update**: `$ISSUE_CLI issues update [RESEARCH_ISSUE_ID] --description "[FULL_DESCRIPTION]"`

4. **Set state**: `$ISSUE_CLI issues update [RESEARCH_ISSUE_ID] --state "Todo"`

Present to user: "Research assets ready for [RESEARCH_ISSUE_ID]".

---

## 4. Asset Quality Checklist

- [ ] prompt.txt follows project research prompt template
- [ ] All context files are self-contained
- [ ] No external references in any file
- [ ] Questions refined by domain agents
- [ ] Deliverables are specific and actionable

## 5. Return State

**If managed** (`lifecycle: "managed"`):
   1. **Get task** on last task → description shows return section.
   2. **Continue there immediately**, do not stop.

**If standalone** (`lifecycle: "self"`):

**END** — research issue created.
