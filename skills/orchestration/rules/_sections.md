# Sections

This file defines all sections, their ordering, impact levels, and descriptions.
The section ID (in parentheses) is the filename prefix used to group rules.

---

## 1. Workflow Execution (wf-)

**Impact:** CRITICAL
**Description:** Task pre-creation, sequential processing, nested workflows

## 2. Delegation (del-)

**Impact:** CRITICAL
**Description:** Spawn patterns, message delivery, task prefix matching

## 3. Agent Lifecycle (life-)

**Impact:** HIGH
**Description:** Agent persistence, re-delegation, shutdown sequencing

## 4. State Management (state-)

**Impact:** HIGH
**Description:** Workflow state files, compaction resilience, recovery

## 5. Coordination (coord-)

**Impact:** MEDIUM
**Description:** Multi-agent sequencing, bundled issues, parallel safety

## 6. Review Pipeline (rev-)

**Impact:** MEDIUM
**Description:** Finding schemas, recommendation categorization, audit
