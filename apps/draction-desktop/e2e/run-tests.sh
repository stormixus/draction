#!/usr/bin/env bash
set -euo pipefail

echo "=== Draction E2E Test Suite ==="
echo ""

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

cd "$PROJECT_ROOT"

# ── 1. Rust unit + integration tests ──
echo "── 1/3 Rust tests ──"
cargo test --all -- --nocapture 2>&1 | tail -5
echo ""

# ── 2. API integration tests (full pipeline) ──
echo "── 2/3 API integration tests ──"
cargo test -p draction-app-core --test api_integration_test -- --nocapture 2>&1 | tail -10
echo ""

# ── 3. Frontend type-check + build ──
echo "── 3/3 Frontend check ──"
pnpm --filter draction-desktop exec tsc --noEmit 2>&1
echo ""

echo "=== All E2E tests passed ==="
