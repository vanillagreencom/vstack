# Sections

This file defines all sections, their ordering, impact levels, and descriptions.
The section ID (in parentheses) is the filename prefix used to group rules.

---

## 1. Measurement Fundamentals (mf)

**Impact:** CRITICAL
**Description:** Core principles for accurate latency measurement. Violations produce misleading data that leads to wrong optimization decisions.

## 2. Benchmarking (bench)

**Impact:** HIGH
**Description:** Patterns for Criterion, Divan, and iai-callgrind benchmarks in Rust. Incorrect setup produces noisy or non-representative results.

## 3. Runtime Monitoring (mon)

**Impact:** HIGH
**Description:** Continuous latency tracking with HdrHistogram and percentile validation. Ensures production systems stay within performance budgets.

## 4. Profiling (prof)

**Impact:** MEDIUM
**Description:** CPU profiling and flamegraph analysis for identifying hot spots. Guides optimization effort to the right locations.
