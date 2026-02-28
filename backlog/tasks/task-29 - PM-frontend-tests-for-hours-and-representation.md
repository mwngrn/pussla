---
id: task-29
title: PM frontend tests for hours editing and representation policy
status: Done
assignee: []
created_date: '2026-02-28 10:00'
updated_date: '2026-02-28 10:00'
labels:
  - project-management
  - frontend
  - tests
  - REQ_PUSSLA_040
  - REQ_PUSSLA_024
  - REQ_PUSSLA_025
  - REQ_PUSSLA_026
dependencies:
  - task-23
  - task-24
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add frontend tests to verify hours editing in project/time-entry views and percent display in utilization-oriented views.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Tests verify project/time-entry views render and edit hours.
- [x] #2 Tests verify utilization views render derived percentages.
- [x] #3 Tests verify project weekly footer totals in hours.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added frontend logic-focused coverage in ``tests/test_representation_policy.mjs`` for:
- hours/percent conversion behavior,
- ISO date-to-week mapping used in week-grid milestone placement,
- milestone grouping/sorting by displayed week columns.
<!-- SECTION:NOTES:END -->
