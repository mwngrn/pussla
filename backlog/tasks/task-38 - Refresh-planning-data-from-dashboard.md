---
id: task-38
title: Refresh planning data from dashboard
status: Done
assignee:
  - codex
created_date: '2026-03-14 19:32'
updated_date: '2026-03-14 19:54'
labels:
  - dashboard
  - workflow
  - collaboration
  - requirements
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the GUI flow for refreshing local planning data from the shared planning repository with clear handling when local unpublished changes would be affected.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The dashboard provides a refresh action for planning data.
- [x] #2 Refresh retrieves and applies remote changes from the shared repository.
- [x] #3 The GUI warns when local unpublished changes would be affected by refresh.
- [x] #4 The GUI presents actionable success and error feedback for refresh failures.
- [x] #5 Automated tests cover successful refresh and representative warning/error scenarios.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Reuse the same dashboard sync backend added for task-37 to expose refresh behavior safely.
2. Surface refresh controls and local-change warnings in the GUI so non-technical users understand what will happen before remote updates are applied.
3. Add automated coverage for successful refresh, blocked refresh due to local unpublished changes, and representative Git error cases.
4. Update docs and backlog notes before closing the task.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Refresh reuses the same dashboard sync backend and safely fetches from the shared Git target before applying fast-forward updates.

The GUI now warns and blocks refresh when local unpublished working-tree changes or unpublished commits would be affected, and reports actionable errors for divergence or missing remote configuration.

Added integration-style Git tests in `tests/test_dashboard_sync.py` covering successful refresh, blocked refresh with local changes, publish success, and sync status behavior.

Verification: `python3 -m unittest tests.test_dashboard_sync tests.test_dashboard_startup tests.test_dashboard_editing tests.test_project_management_backend` passed. `npm run build` passed for the React frontend bundle.
<!-- SECTION:NOTES:END -->
