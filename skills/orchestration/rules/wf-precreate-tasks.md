---
title: Pre-Create All Workflow Tasks
impact: CRITICAL
impactDescription: Lost position after context compaction — cannot resume workflow
tags: workflow, tasks, compaction
---

## Pre-Create All Workflow Tasks

**Impact: CRITICAL (lost position after context compaction — cannot resume workflow)**

Run `workflow-sections` to extract section headers from workflow markdown, then create all tasks before executing any section. The task list serves as a durable position anchor that survives context compaction (conversation history is discarded but task state persists).

Two-step ordering: extract section data first, then create tasks. For delegation workflows with a team context, the team must exist before task creation or tasks go to the wrong scope and are invisible to teammates.
