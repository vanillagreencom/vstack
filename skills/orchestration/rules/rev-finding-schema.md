---
title: Review Finding Schema Compliance
impact: MEDIUM
impactDescription: Malformed review JSON breaks automated fix routing and issue creation
tags: review, schema, JSON
---

## Review Finding Schema Compliance

**Impact: MEDIUM (malformed review JSON breaks automated fix routing and issue creation)**

All review and QA agents must output JSON following the review finding schema: `agent`, `timestamp`, `verdict` (pass or action_required), `summary`, `blockers[]`, `suggestions[]`, `questions[]`, and optional `qa_metadata`. Verdict is `action_required` if blockers exist, `pass` otherwise.

Each item requires: `id`, `title` (5-10 words), `location` (file path with function/struct names, no line numbers), `description`, `recommendation`, `priority` (1-4), `estimate` (1-5). Suggestions also require `category` (fix or issue).
