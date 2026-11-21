---
id: task-31
aliases:
- task-31
priority: 31
---

# Implement hybrid dashboard startup and path resolution
#story/done Implement hybrid startup behavior for the local dashboard so non-developers can start from the dataset folder without flags, while advanced users can pass an explicit data root via `--data-dir`.

## Description

Implement hybrid startup behavior for the local dashboard so non-developers can start from the dataset folder without flags, while advanced users can pass an explicit data root via `--data-dir`.

## Acceptance Criteria
- ✓ Dashboard startup supports auto-detection of data roots from current working directory.
- ✓ Dashboard startup supports explicit path flags (`--data-dir`) and explicit flags take precedence over auto-detection.
- ✓ Startup logs show resolved planning and identity paths so dataset selection is transparent.
- ✓ Invalid/missing data roots fail fast with actionable error messages including expected structure and example commands.
- ✓ Automated tests cover auto-detect success/failure, explicit-path precedence, and error messaging behavior.
- ✓ README/startup docs are updated with both startup modes and examples.

## Implementation Plan

1. Refactor `src/dashboard/run_dashboard.py` so explicit startup uses a single `--data-dir` and auto-detect remains available.
2. Add deterministic path resolution and startup validation for planning and identity folders under the selected data root.
3. Keep startup output explicit about resolved planning and identity paths.
4. Add tests for auto-detect, explicit `--data-dir` precedence, and actionable startup failures.
5. Update startup docs and related scripts to match the single-data-root model.

## Implementation Notes

Refactored dashboard startup around a single data-root model: startup now auto-detects from the current directory or `./tst-data`, while explicit startup uses `--data-dir`.

Added deterministic validation and actionable startup errors for missing planning/identity structure, while keeping resolved planning and identity paths visible in startup output.

Updated `README.md` to document auto-detect, explicit `--data-dir`, and the React dashboard shortcut behavior.

Added `tests/test_dashboard_startup.py` to cover auto-detect, explicit precedence, legacy `planing/`, validation errors, and browser-launch support.

Verification: `python3 -m unittest tests.test_dashboard_startup tests.test_dashboard_editing tests.test_project_management_backend` passed. A short startup run outside the sandbox confirmed successful binding and resolved-path logging.
