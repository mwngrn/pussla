#!/usr/bin/env bash
set -euo pipefail

# Sets up the development environment for Sphinx-needs docs.

VENV_DIR="${VENV_DIR:-venv}"

log() {
  printf '%s\n' "$1"
}

skip() {
  printf 'Skipping: %s\n' "$1"
}

ensure_python_package() {
  local module_name="$1"
  local package_name="$2"

  if python -c "import importlib.util, sys; sys.exit(0 if importlib.util.find_spec('${module_name}') else 1)"; then
    skip "${package_name} already installed"
  else
    log "Installing ${package_name}..."
    python -m pip install "${package_name}"
  fi
}

ensure_system_dependencies() {
  if ! command -v apt-get >/dev/null 2>&1; then
    return
  fi

  local missing_pkgs=()
  local required_pkgs=("python3-venv" "libpango-1.0-0" "libharfbuzz0b" "libpangoft2-1.0-0" "libpangocairo-1.0-0")

  for pkg in "${required_pkgs[@]}"; do
    if ! dpkg -s "$pkg" >/dev/null 2>&1; then
      missing_pkgs+=("$pkg")
    fi
  done

  if [ ${#missing_pkgs[@]} -gt 0 ]; then
    log "Installing missing system dependencies: ${missing_pkgs[*]}..."
    local sudo_cmd=""
    if [ "$(id -u)" -ne 0 ]; then
      sudo_cmd="sudo"
    fi
    $sudo_cmd apt-get update
    $sudo_cmd apt-get install -y "${missing_pkgs[@]}"
  fi
}

log "Setting up Sphinx-needs development environment..."

ensure_system_dependencies

if ! command -v python3 >/dev/null 2>&1; then
  log "Error: python3 is required but was not found."
  exit 1
fi

if [ -x "${VENV_DIR}/bin/python" ]; then
  skip "virtual environment already exists at ${VENV_DIR}"
else
  log "Creating virtual environment at ${VENV_DIR}..."
  python3 -m venv "${VENV_DIR}"
fi

# shellcheck disable=SC1090
source "${VENV_DIR}/bin/activate"

log "Using Python: $(python --version)"
log "Using pip: $(python -m pip --version)"

ensure_python_package "sphinx" "sphinx"
ensure_python_package "sphinx_needs" "sphinx-needs"
ensure_python_package "sphinx_simplepdf" "sphinx-simplepdf"
ensure_python_package "weasyprint" "weasyprint"

log "Done."
