#!/usr/bin/env bash
set -euo pipefail

FRONTEND_DIR="${FRONTEND_DIR:-src/frontend}"

if ! command -v pnpm >/dev/null 2>&1; then
  echo "Error: pnpm was not found in PATH."
  exit 1
fi

if [ ! -d "${FRONTEND_DIR}" ]; then
  echo "Error: frontend directory not found: ${FRONTEND_DIR}"
  exit 1
fi

if [ ! -f "${FRONTEND_DIR}/package.json" ]; then
  echo "Error: package.json not found in ${FRONTEND_DIR}"
  exit 1
fi

cd "${FRONTEND_DIR}"

echo "Installing frontend dependencies with pnpm install"
pnpm install --frozen-lockfile

echo "Building frontend"
pnpm run build

echo "Frontend build complete: ${FRONTEND_DIR}/dist"
