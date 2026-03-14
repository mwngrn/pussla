---
id: task-35
title: Launch dashboard in default browser after startup
status: Done
assignee:
  - codex
created_date: '2026-03-14 19:32'
updated_date: '2026-03-14 19:43'
labels:
  - deployment
  - dashboard
  - ux
  - requirements
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement REQ_PUSSLA_051 so non-technical users who start the React dashboard script are taken directly to the running dashboard in their default browser.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Starting `run-react-dashboard.sh` launches the dashboard URL in the user's default browser after the local server is ready.
- [x] #2 The launched URL matches the actual host and resolved port used by the dashboard.
- [x] #3 If the browser cannot be opened automatically, startup still succeeds and the user gets actionable console feedback.
- [x] #4 Automated tests cover URL resolution and browser-launch behavior.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Refactor dashboard startup so the resolved dashboard URL is available to the launcher path.
2. Update `run-react-dashboard.sh` to start the dashboard and open the resolved URL in the default browser.
3. Ensure startup still succeeds if browser launch fails, with actionable console output.
4. Add automated tests for URL resolution and browser-launch behavior.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented browser launch directly in `src/dashboard/run_dashboard.py` via `--launch-browser`, so the actual resolved host/port is used after the server binds.

Updated `run-react-dashboard.sh` to opt into browser launch and documented the workflow in `README.md`.

Added startup unit coverage in `tests/test_dashboard_startup.py` for URL generation and non-fatal browser launch failures.

Verification: `python3 -m unittest tests.test_dashboard_startup tests.test_dashboard_editing tests.test_project_management_backend` passed. A short startup run outside the sandbox confirmed the dashboard binds, resolves `tst-data`, and prints the live URL.
<!-- SECTION:NOTES:END -->
