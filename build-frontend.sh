#!/usr/bin/env bash
set -euo pipefail

FRONTEND_DIR="${FRONTEND_DIR:-src/frontend}"

if ! command -v npm >/dev/null 2>&1; then
  echo "Error: npm was not found in PATH."
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

if [ -f "package-lock.json" ]; then
  echo "Installing frontend dependencies with npm ci"
  npm ci
else
  echo "Installing frontend dependencies with npm install"
  npm install
fi

echo "Building frontend"
npm run build

echo "Frontend build complete: ${FRONTEND_DIR}/dist"
