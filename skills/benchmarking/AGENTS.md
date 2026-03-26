# Benchmarking

**Version 1.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when recording,
> querying, or analyzing benchmark results. Humans may also find it useful,
> but guidance here is optimized for automation and consistency by
> AI-assisted workflows.

---

## Abstract

Record, query, and detect regressions in benchmark results using a structured JSON schema (v3). Supports Criterion (wall-clock), iai-callgrind (instruction counts), and HdrHistogram (latency distributions). Auto-detects project components via configurable mapping.

## Entry Point

```bash
scripts/bench.sh <command> [options]
```

## Configuration

| Variable | Purpose | Default |
|----------|---------|---------|
| `BENCH_RESULTS_DIR` | Override results directory | `$PROJECT_ROOT/benchmarks/results` |
| `BENCH_COMPONENT_MAP` | Custom component mapping script | `$PROJECT_ROOT/benchmarks/component-map.sh` |

## Recording

### Criterion → wall_time

```bash
cargo bench 2>&1 | scripts/parsers/parse-criterion [--dry-run]
```

Extracts operation names, timing values (mean + CI), maps to components, records as schema v3 JSON with `metric_kind: "wall_time"`.

### iai-callgrind → instruction_count + estimated_cycles

```bash
cargo bench --features iai --bench iai_benchmarks 2>&1 | scripts/parsers/parse-iai [--dry-run]
```

Extracts Ir counts and estimated cycles. Deterministic — immune to timing noise.

### HdrHistogram / Custom Harnesses

```bash
# Pipe JSON lines from your harness
your_harness | while read line; do
  scripts/bench.sh record my_component "$line"
done
```

### Manual Recording

```bash
scripts/bench.sh record my_component '{
  "schema_version": 3,
  "operation": "my_operation",
  "tool": {"name": "criterion", "version": "0.5"},
  "measurements": [{"metric_kind": "wall_time", "unit": "ns",
    "summary": {"estimate": {"stat": "mean", "value": 89000},
                "confidence_interval": {"level": 0.95, "lower": 85000, "upper": 93000}}}]
}'
```

Auto-populated by `bench.sh record`: `schema_version`, `timestamp`, `commit_hash`, `environment.*`

## Querying

```bash
bench.sh list                              # List components with results
bench.sh summary                           # Latest results for all components
bench.sh latest my_component               # Most recent result
bench.sh baseline my_component             # Baseline for comparison
bench.sh query my_component --days 7       # History (default 30 days)
bench.sh compare my_component abc123 def456  # Compare two commits
```

## Regression Detection

```bash
bench.sh regression my_component --threshold 5
bench.sh regression my_component --baseline path/to/baseline.json
```

Compares baseline vs latest using the primary metric: `wall_time` → mean, `instruction_count` → Ir, `latency_distribution` → p99. Metric kind must match — cross-kind comparison is rejected.

Exit codes: `0` = pass/improvement, `1` = regression detected.

Cross-platform and environment change warnings shown automatically when kernel, CPU, or governor differ between baseline and latest.

## Regression Classification

When regression is detected, every regressed operation must be classified. Silent omission is forbidden.

| Classification | Criteria | Action |
|---|---|---|
| **hot-path** | Operations on the critical execution path | **Blocker** — must fix before merge |
| **cold-path** | Startup, teardown, configuration, initialization | **Report with justification** — blocker only if budget exists |
| **intentional** | Regression caused by deliberate architectural change | **Report with decision reference** — not a blocker if documented |
| **environmental** | Regression coincides with environment change warning | **Report with env diff** — re-baseline before treating as code regression |

Rules:
- Classify every regressed operation. No exceptions.
- Never dismiss as "measurement artifact" without evidence.
- Cold-path and intentional regressions still appear in the report — just not blockers.
- If uncertain, default to **hot-path** (blocker).

## Schema v3

### wall_time (Criterion)

```json
{
  "schema_version": 3,
  "component": "my_component",
  "operation": "my_operation",
  "timestamp": 1769991326,
  "commit_hash": "50b9dee",
  "environment": {
    "platform": "linux-x86_64",
    "kernel": "6.19.9",
    "cpu_model": "AMD Ryzen 9 9950X",
    "cpu_governor": "performance"
  },
  "tool": { "name": "criterion", "version": "0.5" },
  "measurements": [{
    "metric_kind": "wall_time",
    "unit": "ns",
    "summary": {
      "estimate": { "stat": "mean", "value": 13.1 },
      "confidence_interval": { "level": 0.95, "lower": 13.0, "upper": 13.2 }
    }
  }]
}
```

### instruction_count (iai-callgrind)

```json
{
  "schema_version": 3,
  "tool": { "name": "iai-callgrind", "version": "0.16.1" },
  "measurements": [
    { "metric_kind": "instruction_count", "unit": "instructions", "summary": { "estimate": { "stat": "Ir", "value": 7247606 } } },
    { "metric_kind": "estimated_cycles", "unit": "cycles", "summary": { "estimate": { "stat": "EstimatedCycles", "value": 9821000 } } }
  ]
}
```

### latency_distribution (HdrHistogram)

```json
{
  "schema_version": 3,
  "tool": { "name": "hdrhistogram_harness", "version": "0.1" },
  "measurements": [{
    "metric_kind": "latency_distribution",
    "unit": "ns",
    "topology": "intra_ccd",
    "distribution": { "p50": 15, "p99": 28, "p999": 95 },
    "coordinated_omission": { "corrected": true, "method": "record_correct", "expected_interval_ns": 20000 }
  }]
}
```

### Metric Kind Semantics

- **wall_time** (Criterion): `estimate.stat = "mean"` — estimated mean, not a true percentile
- **instruction_count** (iai-callgrind): Deterministic Ir count, immune to timing noise
- **estimated_cycles** (iai-callgrind): Simulated cycles (Callgrind simulator)
- **latency_distribution** (HdrHistogram): True percentiles with coordinated omission correction

## File Naming

Historical results (auto-generated): `{timestamp}_{commit}_{op_hash}.json`
Issue baselines (manual): `baseline_{ISSUE_ID}.json`

## Component Mapping

Parsers map benchmark group names to component directories. Default behavior: use the benchmark prefix as the component name. Override by creating `benchmarks/component-map.sh`:

```bash
# benchmarks/component-map.sh
map_component() {
    local prefix="$1"
    case "$prefix" in
        inproc-*|spsc-*)     echo "ipc" ;;
        tick-buffer-*|feed-*) echo "market_data" ;;
        order-*)              echo "order_execution" ;;
        *)                    echo "$prefix" ;;
    esac
}
```

## Setting a Baseline

```bash
# Explicit baseline (preferred)
bench.sh latest my_component > benchmarks/results/my_component/baseline.json

# Automatic: uses oldest result in 90-day window
bench.sh regression my_component --threshold 5
```

## CI Integration

```yaml
# Deterministic gate (iai-callgrind, every PR)
- name: Instruction count regression
  run: cargo bench --features iai --bench iai_benchmarks

# Allocation gate (every PR)
- name: Zero-alloc assertions
  run: cargo test --test alloc_tests

# Wall-clock regression (advisory, nightly)
- name: Wall-clock regression
  run: bench.sh regression my_component --threshold 5
```

## Dependencies

- `jq` for JSON processing
- `bash` 4+
