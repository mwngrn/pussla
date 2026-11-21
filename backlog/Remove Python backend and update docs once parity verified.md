---
id: TASK-45.9
aliases:
- TASK-45.9
priority: 54
---

# Remove Python backend and update docs once parity verified
#story/backlog Once task-45.6 and task-45.7 are done and verified (cargo check/clippy clean, frontend builds, Mathias has manually verified `pnpm tauri dev` works end-to-end): remove src/dashboard/…

Requires < [[Wire Tauri commands to core , remove Python subprocess spawn]]

Requires < [[Rewire frontend API layer from fetch() to Tauri invoke()]]

Parent: [[Rust core-first architecture for Pussla (framkalla pattern)]]

## Description

Once task-45.6 and task-45.7 are done and verified (cargo check/clippy clean, frontend builds, Mathias has manually verified `pnpm tauri dev` works end-to-end): remove src/dashboard/ (pussla_engine.py, validate_planning_data.py, aggregate_planning_data.py, dashboard_sync.py, run_dashboard.py), requirements.txt, setup-python-venv.sh, and any now-dead launcher scripts (check build-and-run-desktop.sh, build-frontend.sh, run-react-dashboard.sh for Python references). Update README.md's "Run Dashboard Locally" and "Tauri desktop shell" sections, agent.md, and AGENTS.md to describe the new core/ + Tauri architecture instead of the Python dashboard. Do not remove anything until Mathias has confirmed manual verification per dev-wow's tester split — this task is gated on his sign-off, not just green CI.

## Acceptance Criteria
- src/dashboard/, requirements.txt, setup-python-venv.sh are deleted from git
- No remaining script references python3/pip/venv for running the app
- README.md, agent.md, AGENTS.md describe the core/ + Tauri architecture, not the Python backend
- Removal only happens after Mathias's explicit manual-verification sign-off is recorded in this task's notes
