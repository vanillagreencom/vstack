# Screenshot-Only Target Sweep

Use this when a target supports screenshots and OCR-style interactions, but does not expose a live layout/map contract.

## When A Single Screenshot Is Not Enough

Do not stop at one screenshot when the change affects:
- a component viewer or showcase target
- a design-system package with multiple states or variants
- shared visual tokens used across multiple surfaces
- layout, clipping, spacing, alignment, or overflow behavior

In those cases, capture a representative sweep across the affected categories or states.

## Minimum Sweep Pattern

1. Identify the affected categories or component families.
2. Capture at least one representative state from each affected category.
3. Include edge states when relevant:
   - empty
   - loading
   - disabled
   - error
   - selected or active
   - dense or overflow-prone
4. Inspect every capture for:
   - clipping
   - alignment drift
   - overflow or wrapping regressions
   - missing focus, hover, or selection affordances
   - contrast regressions

## Preferred Automation

If the project defines a dedicated sweep command, run it:

```bash
$VISUAL_QA_SWEEP_CMD
```

If no sweep command exists, perform the representative sweep manually with `screenshot`, `start`, `locate`, `click`, and any project-specific runtime checks.

## Output Expectation

Report the sweep scope, the states covered, and whether any captures showed clipping, alignment, or overflow issues.
