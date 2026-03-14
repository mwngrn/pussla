---
id: task-37
title: Publish planning changes from dashboard
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
Implement the GUI flow for publishing saved planning changes to the shared planning repository with a user-facing change summary and actionable feedback.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The dashboard provides a publish action for saved planning changes.
- [x] #2 The publish flow collects a user-facing change summary before publishing.
- [x] #3 Publishing creates a versioned save and sends it to the configured shared repository.
- [x] #4 The GUI presents actionable success and error feedback for publish failures.
- [x] #5 Automated tests cover successful publish and representative failure cases.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Inspect the current React dashboard shell and local dashboard backend API surface to decide where non-technical sync actions belong.
2. Add backend endpoints for dashboard sync status, publish, and refresh against the local Git repository rooted at the selected data directory.
3. Implement a lightweight GUI flow for publish and refresh with user-facing summaries, dirty-state/conflict warnings, and actionable success/error messaging.
4. Add automated tests for the backend Git workflow and the new frontend-visible API contract.
5. Update docs and backlog notes, then proceed directly to task-38 and finally reconcile the parent task.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added a local dashboard sync backend in `src/dashboard/dashboard_sync.py` and exposed `/api/sync/status` plus `/api/sync/publish` from `src/dashboard/run_dashboard.py`.

Publish now stages only the selected data-root scope, creates a Git commit with a user-provided summary, and pushes to the configured shared target (upstream when present, otherwise a safe remote fallback).

Added a React `Planning Sync` panel with a publish dialog, changed-file visibility, and actionable success/error feedback.

Verification: `python3 -m unittest tests.test_dashboard_sync tests.test_dashboard_startup tests.test_dashboard_editing tests.test_project_management_backend` passed. `npm run build` passed for the React frontend bundle.
<!-- SECTION:NOTES:END -->
