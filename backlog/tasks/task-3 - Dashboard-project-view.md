---
id: task-3
title: Dashboard project view
status: To Do
assignee: []
created_date: '2026-02-22 17:48'
updated_date: '2026-02-22 18:01'
labels:
  - dashboard
  - project-management
  - reporting
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add a project-centric dashboard view to support project managers with capacity and cost planning.

Scope:
- Add a project view mode where a selected project is the primary filter.
- Show who is allocated to the selected project and their weekly allocation for that project.
- Show weekly project totals and full-period total in hours.
- Convert `%` allocation to hours using 40h/week at 100%.
- Totals default to all weeks in the dataset.
- Keep existing person-centric view available.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Dashboard supports switching between current person view and new project view.
- [ ] #2 Project view provides a project selector with available projects from planning data.
- [ ] #3 In project view, rows show only people allocated to the selected project.
- [ ] #4 Cells in project view show allocated hours for selected project per person/week (not percentages).
- [ ] #5 Hours conversion uses 40h/week as the basis for 100% load.
- [ ] #6 Project view shows a weekly total-hours summary and a full-range total-hours summary for the selected project across all weeks in the dataset by default.
- [ ] #7 Person-centric view behavior remains unchanged.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Define data contract additions needed from `/api/dashboard-data` for project-centric rendering.
2. Decide hours conversion basis (for example 40 hours/week per 100% load) and make it explicit/configurable.
3. Extend backend aggregation (`src/dashboard/pussla_engine.py`) to compute project-week-person hours and totals.
4. Add frontend controls and rendering path in `src/dashboard/index.html` for project mode and project selection.
5. Implement project-mode summaries (weekly totals + full-range total) and empty-state handling.
6. Add/update tests for aggregation correctness and view rendering assumptions.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Decisions confirmed: use 40h/week conversion basis; totals default to all weeks in dataset.
<!-- SECTION:NOTES:END -->
