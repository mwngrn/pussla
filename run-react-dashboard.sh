#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PORT="${1:-8081}"

exec /usr/bin/python3 "${ROOT_DIR}/src/dashboard/run_dashboard.py" \
  --port "${PORT}" \
  --data-dir "${ROOT_DIR}/tst-data" \
  --static-dir "${ROOT_DIR}/src/frontend/dist"
