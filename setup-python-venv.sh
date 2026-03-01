#!/usr/bin/env bash
set -euo pipefail

# Create/update a local Python virtual environment and install requirements.
# Usage:
#   source ./setup-python-venv.sh          # creates, installs, and activates in current shell
#   ./setup-python-venv.sh                 # creates and installs (cannot persist activation)

VENV_DIR="${VENV_DIR:-venv}"
REQ_FILE="${REQ_FILE:-requirements.txt}"

if ! command -v python3 >/dev/null 2>&1; then
  echo "Error: python3 was not found in PATH."
  exit 1
fi

if [ ! -f "${REQ_FILE}" ]; then
  echo "Error: requirements file not found: ${REQ_FILE}"
  exit 1
fi

if [ ! -x "${VENV_DIR}/bin/python" ]; then
  echo "Creating virtual environment in ${VENV_DIR}/"
  python3 -m venv "${VENV_DIR}"
else
  echo "Using existing virtual environment in ${VENV_DIR}/"
fi

# shellcheck disable=SC1090
source "${VENV_DIR}/bin/activate"

echo "Installing dependencies from ${REQ_FILE}"
python -m pip install --upgrade pip
python -m pip install -r "${REQ_FILE}"

if [[ "${BASH_SOURCE[0]}" != "${0}" ]]; then
  echo "Virtual environment is active in this shell: ${VIRTUAL_ENV}"
else
  echo "Done. Activate with: source ${VENV_DIR}/bin/activate"
fi
