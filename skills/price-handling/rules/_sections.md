# Sections

This file defines all sections, their ordering, impact levels, and descriptions.
The section ID (in parentheses) is the filename prefix used to group rules.

---

## 1. Core Rules (core)

**Impact:** CRITICAL
**Description:** Non-negotiable f64 price handling constraints. Violations cause incorrect comparisons, silent rounding errors, or precision loss that corrupt order prices and P&L calculations.

## 2. Boundaries (boundary)

**Impact:** HIGH
**Description:** Where and when to round, validate, or format prices. Rounding at the wrong boundary silently destroys feed precision or submits invalid orders.

## 3. Type Design (type)

**Impact:** MEDIUM
**Description:** How to structure price-related types — newtypes, symbol metadata, display precision. Violations cause precision embedded in the wrong place or accidental `==` on f64.

## 4. Feed Ingestion (feed)

**Impact:** MEDIUM
**Description:** Patterns for normalizing price data from different feed formats (doubles, strings, scaled integers) to f64 at ingest without precision loss.
