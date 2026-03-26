---
title: Message Gate Pattern
impact: CRITICAL
impactDescription: Agents process non-delegation messages (notifications, system messages) as directives
tags: delegation, spawn, message-gate
---

## Message Gate Pattern

**Impact: CRITICAL (agents process non-delegation messages as directives)**

Every agent spawn prompt must include a mandatory message gate: for EVERY message, the agent scans for a `Task prefix:` line. No prefix found → go idle immediately. This positive gatekeeper (check for X before acting) is more robust than negative filtering (ignore Y).

Without it, agents process task-list notifications and other non-delegation messages, producing incorrect work. The delegation arrives separately via a message containing a `Task prefix:` line. The agent extracts the prefix, checks the task list, and finds PENDING tasks whose subject starts with that prefix.

The spawn prompt template must be copied verbatim (fill placeholders only). Paraphrasing has historically dropped the gate instruction.
