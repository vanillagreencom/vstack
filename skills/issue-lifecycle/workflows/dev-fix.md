# Fix Lifecycle

> **Dependencies**: `$ISSUE_CLI`, `$VALIDATE_CMD`, `$DECISIONS_CMD` (optional), `$VISUAL_QA_CLI` (optional), `$SCREENSHOT_CLI` (optional), `$VISUAL_QA_TARGET_CMD` (optional), `$VISUAL_QA_FIXTURE` (optional), `$VISUAL_QA_SMOKE_CMD` (optional), `$VISUAL_QA_BATTERY_CMD` (optional)

**The workflow for dev agents receiving review fix delegations.**

---

## 1. Environment Setup

---

## 2. Read Issue Context

```bash
$ISSUE_CLI cache issues get [ISSUE_ID]
$ISSUE_CLI cache comments list [ISSUE_ID]
```

Understand prior work, decisions, and handoff notes before evaluating items.

---

## 3. Process Review Items

For each item in `Review items:`:

1. **Evaluate independently** — each item stands alone

2. **Apply if**: related to parent issue, no new risks

3. **Skip if** pattern conflicts with existing architecture, would break other functionality, does not follow your defined rules or conventions.
   - **Before applying** (decider skill): `$DECISIONS_CMD search "[RELEVANT_KEYWORDS]"` for decisions governing the affected area → if match found, read the full decision file
   - If review item contradicts an active decision, skip with decision reference (e.g., "Skipped — contradicts D010")
   - Expanding scope is OK if it relates to the parent issue/PR

4. **Update docs/skills/patterns** if fix changes documented behavior

5. **For UI lifecycle/cache fixes**: If you introduce cached/mirrored UI state or change window/event handling, trace all invalidation and event-entry paths before returning. Prefer extending existing listeners over adding parallel subscriptions for the same event family, and add regression coverage for the non-obvious paths you touched.

6. **Note in return** if fix reveals deeper issues or if you skipped items — cite decision ID or rule

7. **Report as Blocked** if stuck on same fix 3+ times

Related improvements OK — unrelated changes should become separate issues.

---

## 4. Validate

```bash
# Choose based on change scope:
$VALIDATE_CMD --quick              # Fast: lint, unit tests (comment/minor changes)
$VALIDATE_CMD --fail-fast          # Full but stops at first failure (recommended for first run)
$VALIDATE_CMD                      # Full: build, all tests, docs, benchmarks (significant changes)
# After fixing failures:
$VALIDATE_CMD --recheck            # Only re-runs previously failed checks (skip cached passes)
```

**On failure:**
- **First run**: Use `--fail-fast` to stop early, fix, then `--recheck`
- **Simple + related to your work** → fix it, `--recheck`
- **Complex or unrelated** → still commit your work, note failure in commit message, report in return

### 4.1 Visual QA

**Skip if** the issue does not have the `design` label, or the fix does not touch UI code.

Before running commands:
- If the project defines `$VISUAL_QA_TARGET_CMD`, run it first. Use it to select the correct target for the changed files and to discover any companion validation commands.
- Otherwise, use the current/default target configured in `visual-qa.conf`.

Run a targeted visual check using the visual QA skill:
- **Rendering fix**: `$SCREENSHOT_CLI --no-build` → Read the PNG to verify
- **Map-capable interaction / layout target** (target exposes live map geometry and optional fixtures):
  1. `$VISUAL_QA_CLI doctor`
  2. Start a visual QA session. If the project exposes a representative fixture path, prefer it:
     - `$VISUAL_QA_CLI start --layout "$VISUAL_QA_FIXTURE"`
     - Otherwise: `$VISUAL_QA_CLI start`
  3. `$VISUAL_QA_CLI map`
  4. Use map-first high-level commands to test the affected behavior
  5. Use `locate` only for literal text targets or OCR sanity checks
  6. Capture a screenshot or short recording if it adds evidence
- **Screenshot/OCR-only target** (no live map or layout fixture contract):
  1. `$VISUAL_QA_CLI doctor`
  2. `$VISUAL_QA_CLI start --build`
  3. Use `locate`, `click`, `status`, and `screenshot` to test the affected behavior
  4. Pair this with any project-specific runtime validation command (for example `$VISUAL_QA_SMOKE_CMD`) when available
- **Broader regression risk**: Run `$VISUAL_QA_BATTERY_CMD` when the project defines one; otherwise note that no dedicated visual battery exists

Focus on what the fix changes — not the full checklist.

### 4.2 Commit

```bash
git add -A && git commit -m "[PREFIX]([ISSUE_ID]): [MESSAGE]"
```

| Source | Commit Message |
|--------|----------------|
| `pr-review` | "Address PR review - [brief description]" |
| `qa-review` | "Address QA review - [brief description]" |
| `suggestions` | "Address review suggestions" |

If validation failures exist, append: `[validate: FAILING_CHECK]`

---

## 5. Reflect & Update Skills/Rules

**Skip if** all fixes were one-off issues unlikely to recur (e.g., typo, missing import).

**Trigger**: Any of these during § 3-4:
- Fixed same problem 2+ times (lint, pattern, API usage, test approach)
- Discovered non-obvious gotcha worth remembering
- Spent multiple cycles on something a rule/skill could prevent
- Documentation in skill, rules, patterns, need changed based on discovered optimal approaches

**Action**: Update the source directly.

- **Repeated mistake** → Add rule to project rules or agent definition
- **Reusable pattern** → Add to relevant skill
- **Missing context** → Update architecture doc or reference table
- **Wrong guidance** → Fix incorrect rule, skill, or pattern that caused the issue

Criteria: Would this save 5+ minutes in a future session? If yes, update. One surgical addition per lesson. No verbose examples.

**If you can't update directly** (wrong domain, needs discussion): note in § 6 return with type `[process]`.

---

## 6. Return

**Return exactly**:

<output_format>
| # | Decision | Reasoning |
|---|----------|-----------|
| N | Applied/Skipped/Blocked | [EXPLANATION — cite DXXX or rule if Skipped] |

Commits: [SHAS or "none"]
Validate: [pass or "FAILING: check1, check2"]
</output_format>

Report decision and reasoning for each item. Include commit SHAs and validation status.

**Do NOT** push — orchestrator handles after review.
