---
name: benchmarking
description: "Benchmark recording, querying, and regression detection with schema v3 JSON. Invoke as /benchmarking bench to run full benchmark suite. Use when recording benchmark results, checking for performance regressions, comparing commits, or managing benchmark baselines. Supports Criterion, iai-callgrind, and HdrHistogram harnesses."
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Benchmarking

Record, query, and detect regressions in benchmark results using a structured JSON schema. Supports multiple measurement tools and auto-detects project components.

## When to Apply

Reference these guidelines when:
- Recording benchmark results from Criterion, iai-callgrind, or custom harnesses
- Checking for performance regressions between commits
- Managing baselines for optimization work
- Querying historical benchmark data
- Setting up CI gates for performance

## Entry Point

```bash
scripts/bench.sh <command> [options]
```

## Commands

| Command | Purpose |
|---------|---------|
| `query <component> [--days N]` | Query benchmark history (default: 30 days) |
| `latest <component>` | Most recent result for component |
| `baseline <component>` | Baseline (oldest in 90-day window or tagged) |
| `record <component> <json>` | Record a new result (schema v3 JSON) |
| `regression <component> [--threshold N]` | Check for regression (exit 1 = regressed) |
| `compare <component> <commit1> <commit2>` | Compare two commits |
| `list` | List all components with results |
| `summary` | Latest results for all components |

## Recording

```bash
# Criterion → wall_time
cargo bench 2>&1 | scripts/parsers/parse-criterion [--dry-run]

# iai-callgrind → instruction_count + estimated_cycles
cargo bench --features iai --bench iai_benchmarks 2>&1 | scripts/parsers/parse-iai [--dry-run]

# Manual recording (any tool)
scripts/bench.sh record my_component '{"schema_version":3,...}'
```

## Regression Detection

```bash
scripts/bench.sh regression my_component --threshold 5
scripts/bench.sh regression my_component --baseline path/to/baseline.json
```

Exit codes: `0` = pass/improvement, `1` = regression detected.

## Rule Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Regression Classification | CRITICAL | `reg-` |
| 2 | Recording | HIGH | `rec-` |
| 3 | Schema | MEDIUM | `schema-` |

## Quick Reference

### 1. Regression Classification (CRITICAL)

- `reg-classify-every-regression` - Every regressed operation must be classified; no silent omissions
- `reg-hot-path-blocks-merge` - Hot-path regressions are merge blockers
- `reg-cold-path-report` - Cold-path regressions must be reported with justification
- `reg-environmental-rebaseline` - Re-baseline before treating env-change regressions as code issues

### 2. Recording (HIGH)

- `rec-metric-kind-match` - Baseline and latest must use same metric_kind; cross-kind rejected
- `rec-environment-capture` - Always capture platform, kernel, CPU, governor

### 3. Schema (MEDIUM)

- `schema-v3-required` - All results must use schema v3 format
- `schema-metric-semantics` - wall_time mean ≠ percentile; instruction_count is deterministic

## Skill Command

When invoked as `/benchmarking bench`, run the comprehensive benchmark and regression workflow:

### `/benchmarking bench`

**Follow every step exactly.** Do not skip steps, delegate, or deviate.

#### 1. Run Benchmarks

Run each benchmark file separately, piping output through the appropriate parser for automatic recording:

```bash
# For each benchmark file in the project:
$BUILD_CMD bench --bench <benchmark_name> 2>&1 | scripts/parsers/parse-criterion
```

Run benchmark files **individually** — combined runs may exceed timeouts for large suites. Each parser extracts timing values, maps operations to components via `map_component()`, and records schema v3 JSON via `scripts/bench.sh record`.

For deterministic instruction-count benchmarks (if supported):
```bash
$BUILD_CMD bench --bench <iai_benchmark> 2>&1 | scripts/parsers/parse-iai
```

#### 2. Verify Recorded Results

```bash
scripts/bench.sh list
```

Verify all expected components show results. If any show 0 results, re-check benchmark output.

#### 3. Check Regressions

```bash
# For each component with results:
scripts/bench.sh regression <component> --threshold 5
```

If only one commit exists (first run), "no comparison available" is expected.

**If regression detected** (exit code 1): Classify every regressed operation per the Regression Classification rules. Every regressed operation MUST appear in the report — silent omission is forbidden.

#### 4. Compare Against Budgets

Read project performance budgets (if defined). Compare key operations against targets. Present a summary table:

```
| Component | Operation | Measured | Budget | Status |
|-----------|-----------|----------|--------|--------|
```

#### 5. Report

Output: benchmarks recorded per component, regression table with classifications, budget compliance, file paths of results.

## Configuration

| Variable | Purpose | Default |
|----------|---------|---------|
| `BENCH_RESULTS_DIR` | Override results directory | `$PROJECT_ROOT/benchmarks/results` |
| `BENCH_COMPONENT_MAP` | Path to custom component mapping script | `$PROJECT_ROOT/benchmarks/component-map.sh` |

## Component Mapping

Parsers map benchmark names to component directories. Default: use the benchmark group prefix as the component name. Override by placing a `benchmarks/component-map.sh` in your project root that defines `map_component()`.

## Dependencies

- `jq` for JSON processing
- `bash` 4+ (uses associative arrays in some commands)

## Full Compiled Document

For the complete guide with schema examples, regression classification table, and recording patterns: `AGENTS.md`
