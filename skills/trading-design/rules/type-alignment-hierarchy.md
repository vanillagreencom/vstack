---
title: Data Alignment and Size Hierarchy
impact: CRITICAL
impactDescription: Traders cannot scan columns quickly when numbers don't align
tags: typography, alignment, hierarchy, decimal, size
---

## Data Alignment and Size Hierarchy

**Impact: CRITICAL (traders cannot scan columns quickly when numbers don't align)**

### Decimal Alignment

In any column of numeric data, the decimal points must align vertically. This is the single most important typographic rule for trading data. It enables:

- **Instant magnitude comparison** — "is this price bigger or smaller?" answered by vertical position of digits, not by reading
- **Change detection** — when scanning a column, misaligned decimals force re-reading; aligned decimals let the eye flow

Implementation approaches (choose based on your stack):
- Right-align numeric columns with consistent decimal places
- Use tabular figures in a monospace font (handles most cases automatically)
- Pad with non-breaking spaces if your rendering engine doesn't support tabular alignment natively
- For mixed-precision instruments, align on the decimal and let trailing digits extend

### Size Hierarchy for Data

Use font size itself (not just opacity) to establish what matters most. In a dense trading panel:

| Level | Relative Size | Use |
|-------|--------------|-----|
| **Primary** | Base + 1-2px | Current price, total P&L, key metric the panel exists to show |
| **Standard** | Base (11-13px) | Most data: quantities, individual prices, order details |
| **Secondary** | Base - 1px | Labels, column headers, timestamps |
| **Tertiary** | Base - 2px | Metadata, IDs, supplementary info |

The difference between levels should be small (1-2px) because the base is already small. Large size jumps waste density. Subtle size differences combined with opacity hierarchy create a legible information stack without wasting vertical space.

### Column Layout Principles

- **Right-align all numeric data** — this is how traders expect to see numbers. Left-aligned numbers in a column are always wrong.
- **Left-align text data** — symbols, names, labels.
- **Fixed column widths** — columns should not resize when data changes. A price going from 99.50 to 100.50 should not cause adjacent columns to shift. Design for the maximum expected width.
- **Header alignment matches data** — if the column data is right-aligned, the header is right-aligned.
