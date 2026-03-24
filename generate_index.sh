#!/usr/bin/env bash
# Copyright (C) 2026 Parity Technologies (UK) Ltd.
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Generate an index.html page linking to all benchmark comparison reports.
#
# Usage:
#   ./generate_index.sh [reports_dir]
#
# Default reports_dir: ../results/reports (relative to this script)
# Output: index.html in the reports directory
#
# Run this after compare_runs.sh to create a front page for GitHub Pages.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPORTS_DIR="${1:-$SCRIPT_DIR/comparison-reports}"

if [ ! -d "$REPORTS_DIR" ]; then
    echo "Error: reports directory not found: $REPORTS_DIR"
    exit 1
fi

INDEX="$REPORTS_DIR/index.html"

# Collect all HTML reports (exclude index.html itself)
REPORTS=()
while IFS= read -r -d '' f; do
    name=$(basename "$f")
    [ "$name" = "index.html" ] && continue
    REPORTS+=("$name")
done < <(find "$REPORTS_DIR" -maxdepth 1 -name '*.html' -print0 | sort -z)

if [ ${#REPORTS[@]} -eq 0 ]; then
    echo "No HTML reports found in $REPORTS_DIR"
    exit 1
fi

# Generate index.html
cat > "$INDEX" <<'HEADER'
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<title>Benchmark Reports — polkadot-rest-api vs sidecar</title>
<style>
  * { margin: 0; padding: 0; box-sizing: border-box; }
  body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, monospace;
    background: #0d1117; color: #c9d1d9; padding: 40px 24px; max-width: 900px; margin: 0 auto;
  }
  h1 { text-align: center; color: #e6edf3; margin-bottom: 8px; font-size: 1.6em; }
  .subtitle { text-align: center; color: #8b949e; margin-bottom: 32px; font-size: 0.9em; }
  .reports { list-style: none; }
  .endpoint-group {
    background: #161b22; border: 1px solid #30363d; border-radius: 8px;
    margin-bottom: 16px; overflow: hidden;
  }
  .endpoint-header {
    display: flex; align-items: center; padding: 14px 20px; gap: 12px;
    border-bottom: 1px solid #30363d;
  }
  .endpoint-header .icon { font-size: 1.4em; }
  .endpoint-header .name { font-size: 1.05em; font-weight: 600; color: #d0d4da; }
  .endpoint-header .path { font-size: 0.8em; color: #3fb950; font-weight: 600; margin-left: 8px; }
  .scenarios { list-style: none; }
  .scenarios li { border-top: 1px solid #21262d; }
  .scenarios li:first-child { border-top: none; }
  .scenarios li a {
    display: flex; align-items: center; padding: 10px 20px 10px 48px;
    color: #c9d1d9; text-decoration: none; gap: 12px; transition: background 0.15s;
  }
  .scenarios li a:hover { background: #1c2129; color: #e6edf3; }
  .scenario-tag { font-weight: 600; font-size: 0.9em; }
  .scenario-tag.light_load { color: #7dac7d; }
  .scenario-tag.medium_load { color: #7a9ec2; }
  .scenario-tag.heavy_load { color: #c4993e; }
  .scenario-tag.stress_test { color: #c27171; }
  .arrow { color: #484f58; font-size: 1.1em; margin-left: auto; }
  .meta { text-align: center; color: #484f58; font-size: 0.75em; margin-top: 32px; }
</style>
</head>
<body>
<h1>Benchmark Reports</h1>
<p class="subtitle">polkadot-rest-api vs substrate-api-sidecar</p>
<div class="reports">
HEADER

# First pass: parse all reports into "endpoint|scenario|filename" lines
PARSED=()
for report in "${REPORTS[@]}"; do
    base="${report%.html}"
    base="${base#comparison_}"

    scenario=""
    endpoint="$base"
    for s in light_load medium_load heavy_load stress_test; do
        if [[ "$base" == *"_${s}" ]]; then
            scenario="$s"
            endpoint="${base%_${s}}"
            break
        fi
    done

    PARSED+=("${endpoint}|${scenario}|${report}")
done

# Get unique endpoints in order
ENDPOINTS=()
for entry in "${PARSED[@]}"; do
    ep="${entry%%|*}"
    # Check if already in ENDPOINTS
    found=0
    for existing in ${ENDPOINTS[@]+"${ENDPOINTS[@]}"}; do
        [ "$existing" = "$ep" ] && found=1 && break
    done
    [ "$found" -eq 0 ] && ENDPOINTS+=("$ep")
done

# Second pass: render grouped by endpoint
for endpoint in "${ENDPOINTS[@]}"; do
    endpoint_display="${endpoint//_/ }"

    cat >> "$INDEX" <<EOF
  <div class="endpoint-group">
    <div class="endpoint-header">
      <span class="icon">📊</span>
      <span class="name">$endpoint</span>
    </div>
    <ul class="scenarios">
EOF

    # Render scenarios in severity order: light → medium → heavy → stress → unknown
    for ordered_scenario in light_load medium_load heavy_load stress_test ""; do
        for entry in "${PARSED[@]}"; do
            ep="${entry%%|*}"
            [ "$ep" != "$endpoint" ] && continue
            rest="${entry#*|}"
            scenario="${rest%%|*}"
            filename="${rest#*|}"
            [ "$scenario" != "$ordered_scenario" ] && continue
            scenario_display="${scenario//_/ }"

            if [ -n "$scenario" ]; then
                cat >> "$INDEX" <<EOF
      <li><a href="$filename"><span class="scenario-tag $scenario">$scenario_display</span><span class="arrow">→</span></a></li>
EOF
            else
                cat >> "$INDEX" <<EOF
      <li><a href="$filename"><span class="scenario-tag">view report</span><span class="arrow">→</span></a></li>
EOF
            fi
        done
    done

    cat >> "$INDEX" <<EOF
    </ul>
  </div>
EOF
done

cat >> "$INDEX" <<'FOOTER'
</div>
<p class="meta">
  Generated by generate_index.sh |
FOOTER

echo "  $(date +%Y-%m-%d\ %H:%M:%S)" >> "$INDEX"

cat >> "$INDEX" <<'FOOTER2'
</p>
</body>
</html>
FOOTER2

echo "Generated: $INDEX (${#REPORTS[@]} reports)"
