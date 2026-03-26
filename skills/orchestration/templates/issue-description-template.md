# Issue Description Template

Standard format for issue tracker descriptions created by workflows. Match the structured style of existing well-authored issues.

## Template

```markdown
**Research**: [RESEARCH_REF]
**Decision [DXXX]**: [DECISION_PATH]
**Source**: [ORIGIN_CONTEXT]

[DESCRIPTION — 1-3 sentences explaining the problem or improvement]

## Requirements

* [REQUIREMENT_1]
* [REQUIREMENT_2]
* [REQUIREMENT_3]

## Context

- **Location**: `[FILE_PATH]`
```

## Field Mapping

| Placeholder | Source (audit input) | Notes |
|-------------|---------------------|-------|
| `[ORIGIN_CONTEXT]` | `"PR review suggestion ([found_by])"` or `"architecture planning"` etc. | Always include provenance |
| `[DESCRIPTION]` | `items[].description` | Use directly — agents write 2-3 sentences |
| `[REQUIREMENT_*]` | `items[].recommendation` | Use directly — agents write `* bullet` list |
| `[FILE_PATH]` | `items[].location` | Backtick-wrapped file path. **Never include line numbers** — they go stale as code changes. Use function/struct names for precision. |
| `[RESEARCH_REF]` | input `research_ref`, or inherit from parent description | Top of description. Omit if none |
| `[DXXX]` | input `decision_ref`, or inherit from parent description | After Research line. Omit if none |
| `[DECISION_PATH]` | Path to decision document in project decision documents | Full path to decision file |

## Rules

1. **Always use heredoc** (`cat <<'EOF'`) for `--description` — never inline single-line strings
2. **Use fields directly** — review agents write issue-quality `description` and `recommendation`; no expansion needed at assembly time
3. **Omit empty lines** — drop Research, Decision, Context lines with no data
4. **No escaped newlines in heredoc** — write actual newlines. JSON `\n` sequences become real newlines when the agent writes the heredoc content
5. **Check decisions before creating** — `$DECISIONS_CMD search "[RELEVANT_KEYWORDS]"` to check if the proposed approach is governed by an active decision. Description must not contradict it. Reference the decision at the top of the description

## CLI Usage

```bash
$ISSUE_CLI issues create \
  --title "[TITLE]" \
  --project "[PROJECT]" \
  --labels "[LABELS]" \
  --priority [PRIORITY] \
  --estimate [ESTIMATE] \
  --parent [PARENT_ID] \
  --description "$(cat <<'EOF'
**Source**: PR review suggestion (test-review)

Connection pool capacity growth path is untested. A burst of concurrent
requests exceeding the initial pool size would trigger reallocation,
and the cap at max_connections*4 is untested.

## Requirements

* Create unit test with mock server returning near-capacity connections
* Verify pool grows correctly and caps at configured maximum
* Exercise concurrent request bursts

## Context

- **Location**: `src/pool/connection_pool.rs` (`ConnectionPool::grow`)
EOF
)"
```
