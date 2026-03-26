---
title: Title Bar Event Ordering
impact: CRITICAL
impactDescription: State cleared by body handler overwrites title bar state
tags: pane_grid, titlebar, event_ordering, update
---

## Title Bar Event Ordering

**Impact: CRITICAL (state cleared by body handler overwrites title bar state)**

In `pane_grid::Content::update`, the title bar is processed before the body. When the cursor crosses from body to title bar in a single frame, title bar messages (e.g., `TabBarEntered`) fire before body messages (e.g., `PaneBodyExited`). Do not unconditionally clear state in body-exit handlers that the title bar just established.
