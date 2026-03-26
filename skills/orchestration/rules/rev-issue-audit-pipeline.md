---
title: Issue Audit Pipeline
impact: MEDIUM
impactDescription: Review findings lost — suggestions categorized as issues never get created as tracked work
tags: review, audit, issues
---

## Issue Audit Pipeline

**Impact: MEDIUM (review findings lost — suggestions never get created as tracked work)**

Review agents output JSON with findings. The orchestrator collects these, transforms suggestions where `category=issue` into audit input, and delegates to a TPM agent for issue creation. Sources include: suggestions (non-blocking improvements), escalated blockers (dev couldn't fix), planned items (roadmap), and discovered work (dev-identified future work).

Each audit item requires: index, title, location (no line numbers), description (2-3 sentences), recommendation (bullet-list requirements), priority, estimate, category, found_by, and origin. Dependency fields (blocks_items, blocked_by_items) are populated when implementation order is known.
