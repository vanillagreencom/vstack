# Benchmarking

Record, query, and detect regressions in benchmark results using a structured JSON schema.

## Structure

- `scripts/bench.sh` - Main entry point (query, record, regression, list, summary)
- `scripts/parsers/parse-criterion` - Criterion stdout → schema v3 JSON
- `scripts/parsers/parse-iai` - iai-callgrind stdout → schema v3 JSON
- `rules/` - Regression classification and recording rules
- **`SKILL.md`** - Quick-reference index for skill-aware harnesses
- **`AGENTS.md`** - Full compiled document for all harnesses

## Configuration

| Variable | Purpose | Default |
|----------|---------|---------|
| `BENCH_RESULTS_DIR` | Override results directory | `$PROJECT_ROOT/benchmarks/results` |
| `BENCH_COMPONENT_MAP` | Custom component mapping script | `$PROJECT_ROOT/benchmarks/component-map.sh` |

## Adding a Parser

1. Create `scripts/parsers/parse-<tool>`
2. Read tool output from stdin
3. Extract operation names and metrics
4. Map to components via `map_component()` (load project override or use prefix as default)
5. Emit schema v3 JSON via `bench.sh record`

## Component Mapping

Default: benchmark group prefix = component name. Override by placing `benchmarks/component-map.sh` in project root:

```bash
map_component() {
    case "$1" in
        my-prefix-*) echo "my_component" ;;
        *)           echo "$1" ;;
    esac
}
```

## Impact Levels

- `CRITICAL` - Regression classification errors cause missed blockers
- `HIGH` - Recording errors cause missing/corrupt data
- `MEDIUM` - Schema inconsistencies cause cross-tool comparison failures
