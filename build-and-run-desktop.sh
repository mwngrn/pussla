#!/usr/bin/env bash
set -euo pipefail

FRONTEND_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/src/frontend"
BINARY="${FRONTEND_DIR}/src-tauri/target/release/pussla"

if ! command -v npm >/dev/null 2>&1; then
  echo "Error: npm was not found in PATH."
  exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "Error: cargo (Rust) was not found in PATH. Install via https://rustup.rs"
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

echo "Building Tauri desktop app..."
npm run tauri:build

echo "Build complete. Starting Pussla..."
exec "${BINARY}"
