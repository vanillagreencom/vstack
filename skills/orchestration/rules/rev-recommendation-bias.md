---
title: Recommendation Categorization
impact: MEDIUM
impactDescription: Miscategorized findings create noise (fix items tracked as issues) or lost work (issue items applied as quick fixes)
tags: review, categorization, fix-vs-issue
---

## Recommendation Categorization

**Impact: MEDIUM (miscategorized findings create noise or lost work)**

For each review suggestion, evaluate in order:

1. **Actionable?** Must have specific deliverable, observable impact, and bounded scope. Vague items ("Add logging", "Consider X") → omit.
2. **Related?** Semantic relevance to the issue or changes (not just file presence). Unrelated → issue regardless of size. Doc/reference updates for changed code → always fix.
3. **Size?** Small, apply directly → fix. Needs delegation, tracking, or history → issue.

When uncertain: prefer fix for related suggestions, prefer issue for relevance questions. "Low priority" does not mean omit — track if actionable.
