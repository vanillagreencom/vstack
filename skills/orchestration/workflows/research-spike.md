# Research Spike Workflow

> **Dependencies**: `$ISSUE_CLI`

Human-initiated research with full agent consultation and asset preparation.

## Inputs

| Context | Source | Required |
|---------|--------|----------|
| `topic` | User input (standalone) or caller context | Yes |
| `issue_id` | Caller context | No (standalone creates new) |
| `type` | Caller context | No (standalone discovers) |
| `project` | Caller context | No (standalone discovers) |

## 1. Discover Topic

### 1.1 Get Topic

Prompt user (plain text): **"What research are you conducting?"**

User provides 1-2 sentence description directly.

### 1.2 Ask Clarifying Questions

Ask user with relevant questions from:

| Category | Questions |
|----------|-----------|
| **Motivation** | What prompted this? (bug, feature, vendor update, curiosity) |
| **Scope** | "Should we?" vs "How do we?" |
| **Baseline** | Current state? (version, pattern, existing approach) |
| **Blockers** | What would make this a no-go? |

Select 2-3 most relevant questions. Adapt wording to topic. Infer research type from description.

### 1.3 Ask Topic-Specific Follow-ups

**Skip if** initial answers are sufficient.

Ask any additional clarifying questions needed before proceeding, based on topic and initial answers.

## 2. Check Stack & Prior Research

### 2.1 Identify Affected Domains

1. **Identify domains** from topic + answers -- infer from component paths (project-configurable).

2. **Present identified domains** with reasoning:
   ```
   Affected domains:
   * [DOMAIN] - [REASON]
   * [DOMAIN] - [REASON]

   Confirm? (Y/n/adjust)
   ```

3. **If user adjusts**, update list accordingly.

### 2.2 Check Related Research

1. **Search for related research**:
   ```bash
   $ISSUE_CLI cache issues list --label research --max --search "[TOPIC_KEYWORDS]"
   ```

2. **If matches found**:
   1. **Read the findings file** - project research docs `[ISSUE_ID]/findings.md`
   2. **Extract ALL key findings** - summary paragraph, bullet lists, Go/No-Go sections
   3. **Set [PRIOR_RESEARCH]** - extracted findings for handoff in § 3.3

3. **Notify user**:
   ```
   Prior Research: [ISSUE_ID] - [TITLE]
   ```

## 3. Create Issue & Prepare Assets

Hand off to full research-issue workflow with context gathered above.

### 3.1 Query Active Project

```bash
PROJECT=$($ISSUE_CLI cache projects list --state started --first)
```

### 3.2 Determine Type from Domain Count

| Domain Count | Type |
|--------------|------|
| 1 | Targeted |
| 2+ | Pervasive |

User can override if scope warrants Strategic (initiative-level, 10+ issues).

### 3.3 Run Research-Issue Workflow

Run Skill: `⤵ /research-issue § 1-5 → § 4` with context:
- `topic`: from § 1.1
- `questions`: merged from § 1.2-1.3
- `domains`: confirmed labels from § 2.1
- `project`: from § 3.1
- `blocked_issue`: (none -- spike has no blocker)
- `type`: from § 3.2
- `prior_research`: extracted findings from § 2.2, or empty

## 4. Present to User

<output_format>

### 📚 RESEARCH SPIKE READY

| Field | Value |
|-------|-------|
| Issue | [RESEARCH_ISSUE_ID] - Research: [TOPIC] |
| Type | [TYPE] |
| Project | [PROJECT] |

### 🎯 AFFECTED DOMAINS

| Domain | Reason |
|--------|--------|
| ☑ [DOMAIN] | [REASON] |

### 📖 PRIOR RESEARCH REFERENCED

| Reference |
|-----------|
| → [prior issue]: [TITLE] |

### ✅ ASSETS CREATED

| Asset |
|-------|
| ✓ [RESEARCH_DOCS_PATH]/[RESEARCH_ISSUE_ID]/prompt.txt |
| ✓ [RESEARCH_DOCS_PATH]/[RESEARCH_ISSUE_ID]/context-[TOPIC].md |

### 📋 NEXT STEPS

| # | Step |
|---|------|
| 1 | Review prompt.txt - refine questions if needed |
| 2 | Execute research (external session) |
| 3 | Add findings: [RESEARCH_DOCS_PATH]/[RESEARCH_ISSUE_ID]/findings.md |
| 4 | Complete: /research-complete [RESEARCH_ISSUE_ID] |
</output_format>

**END**: Research spike complete. User executes research externally, then runs `/research-complete`.
