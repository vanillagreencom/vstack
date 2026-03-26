---
title: Per-Item UI Update
impact: MEDIUM
impactDescription: frame drops from excessive redraws
tags: ui, performance, batching
---

## Per-Item UI Update

**Impact: MEDIUM (frame drops from excessive redraws)**

Updating UI elements individually in a loop triggers a redraw per item, causing frame drops when processing collections.

**Detection:** Loop with individual widget updates or state invalidations.

**Fix:** Batch updates and trigger a single invalidation after the batch completes.
