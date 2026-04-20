#!/usr/bin/env bash
set -euo pipefail

# Generate a benchmark report markdown file.
# Usage: ./scripts/bench-report.sh <version>
# Example: ./scripts/bench-report.sh 0.1.2
# Output: benchmarks/reports/v<version>.md

VERSION="${1:-}"
if [[ -z "$VERSION" ]]; then
    echo "usage: $0 <version>" >&2
    echo "example: $0 0.1.2" >&2
    exit 1
fi

REPORT_DIR="benchmarks/reports"
REPORT_FILE="${REPORT_DIR}/v${VERSION}.md"
mkdir -p "$REPORT_DIR"

if [[ -f "$REPORT_FILE" ]]; then
    echo "error: $REPORT_FILE already exists. Delete it first to regenerate." >&2
    exit 1
fi

# --- Detect environment ---
echo "-> detecting hardware..."

DATE=$(date +%Y-%m-%d)
RUST_VERSION=$(rustc --version | awk '{print $2}')

if [[ "$(uname)" == "Darwin" ]]; then
    CPU=$(sysctl -n machdep.cpu.brand_string 2>/dev/null || echo "unknown")
    CORES=$(sysctl -n hw.ncpu 2>/dev/null || echo "?")
    MEM_BYTES=$(sysctl -n hw.memsize 2>/dev/null || echo "0")
    MEM_GB=$((MEM_BYTES / 1073741824))
    MODEL=$(sysctl -n hw.model 2>/dev/null || echo "Mac")
    HARDWARE="${MODEL} — ${CPU}, ${MEM_GB} GB"
else
    CPU=$(lscpu 2>/dev/null | grep "Model name" | sed 's/.*: *//' || echo "unknown")
    CORES=$(nproc 2>/dev/null || echo "?")
    MEM_KB=$(grep MemTotal /proc/meminfo 2>/dev/null | awk '{print $2}' || echo "0")
    MEM_GB=$((MEM_KB / 1048576))
    HARDWARE="${CPU}, ${MEM_GB} GB"
fi

echo "  hardware: $HARDWARE"
echo "  rust: $RUST_VERSION"
echo "  cores: $CORES"

# --- Generate fixtures (skip if they exist) ---
if [[ -f "benchmarks/fixtures/bench_small.xlsx" && -f "benchmarks/fixtures/bench_medium.xlsx" && -f "benchmarks/fixtures/bench_large.xlsx" ]]; then
    echo "-> fixtures already exist, skipping generation"
else
    echo "-> generating fixtures..."
    cargo run -p xlstream-benchmarks --release --bin generate-fixtures -- --tier all --output-dir benchmarks/fixtures/ 2>&1 | tail -1
fi

# --- Helper: extract median from criterion output ---
# Criterion prints: "time:   [67.538 ms 67.815 ms 68.051 ms]"
# We want the middle value + unit: "67.815 ms"
extract_median() {
    grep "time:" | head -1 | sed 's/\x1b\[[0-9;]*m//g' | \
        sed 's/.*\[\([^]]*\)\].*/\1/' | \
        awk '{print $3, $4}'
}

# --- Helper: run CLI tier benchmark, extract duration + RSS ---
# Usage: run_tier <fixture> <workers>
# Outputs: "duration_ms rss_mb" (space-separated)
run_tier() {
    local fixture="$1" workers="$2"
    if [[ ! -f "$fixture" ]]; then
        echo "n/a n/a"
        return
    fi
    local output="/tmp/bench_$(basename "$fixture" .xlsx)_${workers}w.xlsx"
    local raw
    raw=$(target/release/xlstream evaluate "$fixture" -o "$output" -w "$workers" --verbose 2>&1 | \
        sed 's/\x1b\[[0-9;]*m//g' | grep "evaluate complete")
    local ms rss
    ms=$(echo "$raw" | sed 's/.*duration_ms=//' | awk '{print $1}')
    rss=$(echo "$raw" | sed 's/.*rss_mb=//' | awk '{print $1}')
    echo "${ms:-0} ${rss:-0}"
}

format_time() {
    local ms="$1"
    if [[ "$ms" == "0" || "$ms" == "n/a" ]]; then echo "n/a"; return; fi
    awk "BEGIN {
        if ($ms < 1000) printf \"%.1f ms\", $ms;
        else printf \"%.2f s\", $ms/1000;
    }"
}

format_rss() {
    local mb="$1"
    if [[ "$mb" == "0" || "$mb" == "n/a" ]]; then echo "—"; return; fi
    if [[ "$mb" -ge 1000 ]]; then
        awk "BEGIN {printf \"%.1f GB\", $mb/1000}"
    else
        echo "${mb} MB"
    fi
}

# --- Build release binary ---
echo "-> building release binary..."
cargo build --release -p xlstream-cli 2>&1 | tail -1

# --- Run tier benchmarks via CLI (single pass each) ---
echo "-> running small tier (1 worker)..."
read SMALL_1W_MS SMALL_1W_RSS <<< "$(run_tier benchmarks/fixtures/bench_small.xlsx 1)"
echo "-> running small tier (4 workers)..."
read SMALL_4W_MS SMALL_4W_RSS <<< "$(run_tier benchmarks/fixtures/bench_small.xlsx 4)"

echo "-> running medium tier (1 worker)..."
read MEDIUM_1W_MS MEDIUM_1W_RSS <<< "$(run_tier benchmarks/fixtures/bench_medium.xlsx 1)"
echo "-> running medium tier (4 workers)..."
read MEDIUM_4W_MS MEDIUM_4W_RSS <<< "$(run_tier benchmarks/fixtures/bench_medium.xlsx 4)"

echo "-> running large tier (1 worker)..."
read LARGE_1W_MS LARGE_1W_RSS <<< "$(run_tier benchmarks/fixtures/bench_large.xlsx 1)"
echo "-> running large tier (8 workers)..."
read LARGE_8W_MS LARGE_8W_RSS <<< "$(run_tier benchmarks/fixtures/bench_large.xlsx 8)"

# Format for table
SMALL_1W=$(format_time "$SMALL_1W_MS")
SMALL_4W=$(format_time "$SMALL_4W_MS")
MEDIUM_1W=$(format_time "$MEDIUM_1W_MS")
MEDIUM_4W=$(format_time "$MEDIUM_4W_MS")
LARGE_1W=$(format_time "$LARGE_1W_MS")
LARGE_8W=$(format_time "$LARGE_8W_MS")
RSS_SMALL_1W=$(format_rss "$SMALL_1W_RSS")
RSS_SMALL_4W=$(format_rss "$SMALL_4W_RSS")
RSS_MEDIUM_1W=$(format_rss "$MEDIUM_1W_RSS")
RSS_MEDIUM_4W=$(format_rss "$MEDIUM_4W_RSS")
RSS_LARGE_1W=$(format_rss "$LARGE_1W_RSS")
RSS_LARGE_8W=$(format_rss "$LARGE_8W_RSS")

echo "-> running micro-benchmarks (criterion)..."
MICRO_RAW=$(cargo bench --bench parse --bench arithmetic --bench lookup -p xlstream-benchmarks 2>&1 | sed 's/\x1b\[[0-9;]*m//g')

extract_bench() {
    local name="$1"
    echo "$MICRO_RAW" | grep "$name" -A3 | grep "time:" | head -1 | \
        sed 's/.*\[\([^]]*\)\].*/\1/' | awk '{print $3, $4}'
}

PARSE=$(extract_bench "parse_30")
ARITH_ADD=$(extract_bench "/add")
ARITH_MUL=$(extract_bench "/mul")
ARITH_DIV=$(extract_bench "/div")
CONCAT=$(extract_bench "concat")
VLOOKUP=$(extract_bench "vlookup")

# --- Find previous report for comparison ---
echo "-> looking for previous report..."
PREV_REPORT=""
PREV_VERSION=""
for f in $(ls -r "$REPORT_DIR"/v*.md 2>/dev/null); do
    if [[ "$f" != "$REPORT_FILE" ]]; then
        PREV_REPORT="$f"
        PREV_VERSION=$(basename "$f" .md | sed 's/^v//')
        break
    fi
done

# --- Write report ---
echo "-> writing $REPORT_FILE..."

cat > "$REPORT_FILE" << EOF
# Benchmark Report — v${VERSION}

**Date:** ${DATE}
**Hardware:** ${HARDWARE}
**Rust:** rustc ${RUST_VERSION}
**Profile:** release (LTO fat, codegen-units=1)

## Tier results

| Tier | Rows | Workers | Wall-clock | Peak RSS |
|---|---|---|---|---|
| Small | 10,000 | 1 | ${SMALL_1W:-n/a} | ${RSS_SMALL_1W:-—} |
| Small | 10,000 | 4 | ${SMALL_4W:-n/a} | ${RSS_SMALL_4W:-—} |
| Medium | 100,000 | 1 | ${MEDIUM_1W:-n/a} | ${RSS_MEDIUM_1W:-—} |
| Medium | 100,000 | 4 | ${MEDIUM_4W:-n/a} | ${RSS_MEDIUM_4W:-—} |
| Large | 1,000,000 | 1 | ${LARGE_1W:-n/a} | ${RSS_LARGE_1W:-—} |
| Large | 1,000,000 | 8 | ${LARGE_8W:-n/a} | ${RSS_LARGE_8W:-—} |

## Micro-benchmarks

| Benchmark | Median |
|---|---|
| Parse 30 formulas | ${PARSE:-n/a} |
| Arithmetic (add) | ${ARITH_ADD:-n/a} |
| Arithmetic (mul) | ${ARITH_MUL:-n/a} |
| Arithmetic (div) | ${ARITH_DIV:-n/a} |
| String concat | ${CONCAT:-n/a} |
| VLOOKUP exact (1k) | ${VLOOKUP:-n/a} |
EOF

# --- Auto-compare with previous report ---
if [[ -n "$PREV_REPORT" ]]; then
    echo "-> comparing with $PREV_REPORT..."

    # Convert time string to milliseconds for comparison.
    # Handles: "67.6 ms", "1.66 s", "156 s", "35 us", "16 ns"
    to_ms() {
        local val="$1" unit="$2"
        case "$unit" in
            ns) awk "BEGIN {printf \"%.6f\", $val / 1000000}" ;;
            us|µs) awk "BEGIN {printf \"%.4f\", $val / 1000}" ;;
            ms) awk "BEGIN {printf \"%.2f\", $val}" ;;
            s)  awk "BEGIN {printf \"%.2f\", $val * 1000}" ;;
            *)  echo "0" ;;
        esac
    }

    # Extract wall-clock from a tier row in a report markdown.
    # grep for "| Quick |" etc, pull the 4th column.
    extract_tier() {
        local file="$1" tier="$2" workers="$3"
        grep "| ${tier} " "$file" 2>/dev/null | grep "| ${workers} |" 2>/dev/null | head -1 \
            | awk -F'|' '{print $5}' | xargs || echo ""
    }

    compare_row() {
        local label="$1" tier="$2" workers="$3" new_val="$4"
        local prev_str new_str change

        prev_str=$(extract_tier "$PREV_REPORT" "$tier" "$workers")
        new_str="$new_val"

        if [[ -z "$prev_str" || "$prev_str" == "n/a" || -z "$new_str" || "$new_str" == "n/a" ]]; then
            echo "| ${label} | ${prev_str:-—} | ${new_str:-—} | — |"
            return
        fi

        local prev_val prev_unit new_val_n new_unit prev_ms new_ms
        prev_val=$(echo "$prev_str" | awk '{print $1}')
        prev_unit=$(echo "$prev_str" | awk '{print $2}')
        new_val_n=$(echo "$new_str" | awk '{print $1}')
        new_unit=$(echo "$new_str" | awk '{print $2}')

        prev_ms=$(to_ms "$prev_val" "$prev_unit")
        new_ms=$(to_ms "$new_val_n" "$new_unit")

        if [[ "$prev_ms" == "0" || "$new_ms" == "0" ]]; then
            change="—"
        else
            change=$(awk "BEGIN {
                pct = ($new_ms - $prev_ms) / $prev_ms * 100;
                if (pct >= 0) printf \"+%.1f%%\", pct;
                else printf \"%.1f%%\", pct;
            }")
        fi

        echo "| ${label} | ${prev_str} | ${new_str} | ${change} |"
    }

    {
        echo ""
        echo "## Comparison with v${PREV_VERSION}"
        echo ""
        echo "| Tier | v${PREV_VERSION} | v${VERSION} | Change |"
        echo "|---|---|---|---|"
        compare_row "Small (1w)" "Small" "1" "$SMALL_1W"
        compare_row "Small (4w)" "Small" "4" "$SMALL_4W"
        compare_row "Medium (1w)" "Medium" "1" "$MEDIUM_1W"
        compare_row "Medium (4w)" "Medium" "4" "$MEDIUM_4W"
        compare_row "Large (1w)" "Large" "1" "$LARGE_1W"
        compare_row "Large (8w)" "Large" "8" "$LARGE_8W"
    } >> "$REPORT_FILE"

    # Flag regressions
    echo "" >> "$REPORT_FILE"
    REGRESSIONS=$(grep '+[0-9]' "$REPORT_FILE" | awk -F'|' '{
        gsub(/[^0-9.]/, "", $5);
        if ($5+0 > 20) print $2
    }')
    if [[ -n "$REGRESSIONS" ]]; then
        echo "**WARNING: regressions >20% detected in:${REGRESSIONS}**" >> "$REPORT_FILE"
    else
        echo "No regressions (all within noise threshold)." >> "$REPORT_FILE"
    fi
fi

cat >> "$REPORT_FILE" << 'EOF'

## Changes since previous version

<!-- fill in what shipped in this version -->
EOF

echo "-> done: $REPORT_FILE"
