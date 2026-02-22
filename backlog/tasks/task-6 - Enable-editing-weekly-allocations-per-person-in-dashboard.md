---
id: task-6
title: Enable editing weekly allocations per person in dashboard
status: Done
assignee:
  - codex
created_date: '2026-02-22 18:34'
updated_date: '2026-02-22 18:44'
labels:
  - dashboard
  - editing
  - api
  - data-write
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add dashboard functionality to edit a person’s allocations for a selected week and persist updates back to planning YAML files.

Decisions:
- Editing must support multiple projects within the same person-week.
- Overbooking is allowed (total load may exceed 100%).

Scope:
- Add backend write API endpoint(s) for allocation updates.
- Add frontend edit UI from heatmap cells (open editor, modify project rows, save).
- Persist changes in `tst-data/planning/allocations/{alias}.yaml` with safe file write behavior.
- Refresh dashboard data after successful save.
- Keep existing read-only behavior for users who do not perform edits.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Dashboard allows opening an editor for a specific person/week from the heatmap.
- [x] #2 Editor supports adding, updating, and removing multiple project allocations for that person/week.
- [x] #3 Saving writes changes to the correct allocation YAML file and updates only intended week/project entries.
- [x] #4 Overbooking (>100%) is allowed and displayed without blocking save.
- [x] #5 Backend validates payload shape and returns clear errors for invalid requests.
- [x] #6 Dashboard refreshes and shows updated values immediately after successful save.
- [x] #7 Automated tests cover backend write/update behavior and at least one multi-project week edit scenario.

- [x] #8 Editing flow requires explicit user confirmation (Save) before persisting changes to YAML files; unsaved changes are discarded on cancel/close.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add backend write support in dashboard engine to update one alias/week with multiple project allocations while preserving other weeks/projects.
2. Add POST API endpoint in run_dashboard for allocation updates with payload validation and clear error responses.
3. Add frontend modal editor in dashboard UI: open from cell, edit project rows, allow overbooking, explicit Save confirmation, Cancel discard.
4. Persist changes via API, reload dashboard data on success, and show inline errors for failed saves.
5. Add automated tests for backend update logic, including multi-project week updates and validation errors.
6. Mark task acceptance criteria as complete after verification.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Recommendation: do not auto-save on every field change. Use an explicit `Save changes` action in a modal/editor and optional `Cancel` to avoid accidental writes to planning files.

Suggested UX: open editor from cell click, keep local draft changes, show a small dirty-state indicator, require one confirmation click on save, then write + reload data.

If you want stronger guardrails, add optional second confirmation only when total week load changes by a large threshold (for example >=30 points).

Decision confirmed by user: require explicit save confirmation in GUI before writing allocation changes; do not auto-save edits.

Implemented backend update function `update_week_allocations(...)` in `src/dashboard/pussla_engine.py` to replace one alias/week allocation set while preserving other weeks/projects.

Added POST endpoint `/api/allocation/update` in `src/dashboard/run_dashboard.py` with payload validation and clear 400/404/500 error responses.

Implemented frontend modal editor in `src/dashboard/index.html` with multi-project add/remove/edit, explicit Save confirmation (`window.confirm`), and Cancel/close discard.

Overbooking remains allowed; totals can exceed 100% and are displayed without blocking save.

Successful save triggers data reload (`loadData`) so table/headers/footer refresh immediately.

Added backend tests in `tests/test_dashboard_editing.py` including multi-project week update and validation error scenarios.

Verification: `node --test tests/test_week_format.js tests/test_header_groups.js` passed; `python -m unittest discover -s tests -p 'test_*.py'` passed (5 tests).
<!-- SECTION:NOTES:END -->
