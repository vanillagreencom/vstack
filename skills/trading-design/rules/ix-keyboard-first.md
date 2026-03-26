---
title: Keyboard-First Interaction
impact: HIGH
impactDescription: Mouse-dependent workflows slow traders down in time-critical moments
tags: interaction, keyboard, shortcuts, focus, navigation
---

## Keyboard-First Interaction

**Impact: HIGH (mouse-dependent workflows slow traders down in time-critical moments)**

Professional traders minimize mouse usage. Every critical action should be reachable by keyboard. The mouse is a fallback, not the primary input method.

### Principles

- **Every action has a shortcut** — order placement, cancellation, panel navigation, symbol switching. If a trader does it frequently, it must have a keyboard shortcut.
- **Shortcuts are discoverable** — tooltips on all buttons include the keyboard shortcut. A help overlay (accessible via `?` or similar) shows all shortcuts.
- **Focus is always visible** — a visible focus ring on every interactive element, every time. If focus is invisible, keyboard users are lost. The focus ring should be high-contrast against the dark background.
- **Focus order is logical** — tab order follows visual layout, not DOM order or widget tree order. In a trading context: order entry fields tab in the sequence a trader fills them (symbol → side → quantity → price → submit).
- **Escape always cancels** — in any modal, dialog, dropdown, or input state, Escape returns to the previous state. This is universal in professional tools.

### Trading-Specific Keyboard Patterns

| Action | Expected Pattern |
|--------|-----------------|
| Place order | Shortcut submits the current order entry form |
| Cancel all orders | Panic shortcut with confirmation (one extra keystroke, not a dialog) |
| Cancel last order | Single shortcut, no confirmation for speed |
| Flatten position | Shortcut to close all positions in current symbol |
| Switch symbol | Type-ahead search that activates from any context |
| Navigate panels | Directional shortcuts to move focus between panels |
| Cycle panel tabs | Tab-like shortcuts within a panel group |

### Focus Management

- **Focus trap in modals** — when a confirmation dialog is open, focus cycles within the dialog. Tab doesn't escape to background panels.
- **Return focus on close** — when a dropdown or dialog closes, focus returns to the element that triggered it.
- **Panel focus** — panels should have a "focused" state that is visually distinct (subtle border highlight or header emphasis). The currently focused panel receives keyboard shortcuts.

### Tooltips

Every icon-only button must have a tooltip. The tooltip must include:
1. **Action name** — what this button does
2. **Keyboard shortcut** — how to trigger this without the mouse

Tooltips appear after a brief hover delay (300-500ms) to avoid visual noise during fast mouse movement. They disappear immediately on mouse leave.
