---
id: task-21
title: 'Implement Project Management hours model and UI representation policy'
status: Done
assignee:
  - codex
created_date: '2026-02-28 10:00'
updated_date: '2026-02-28 10:00'
labels:
  - project-management
  - requirements
  - REQ_PUSSLA_017
  - REQ_PUSSLA_018
  - REQ_PUSSLA_019
  - REQ_PUSSLA_022
  - REQ_PUSSLA_040
dependencies:
  - task-22
  - task-23
  - task-24
  - task-25
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Umbrella task for implementing canonical hours-based planning and the percent-vs-hours representation policy.

Scope:
- Canonical storage/write paths in hours (`planned_hours`, `capacity_hours`) with backward compatibility for legacy `load`.
- Utilization-oriented views display derived percentages.
- Project-centric and time-entry views display and edit hours.
- Ensure requirements and tests are synchronized.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
 - [x] #1 Subtasks for data/API, frontend representation, milestones, costs, and assignment states are defined and linked.
 - [x] #2 Canonical hours write/read path is implemented and verified with tests.
 - [x] #3 Representation policy (`REQ_PUSSLA_040`) is implemented and tested.
 - [x] #4 Project management features (`REQ_PUSSLA_024-032`) have tracked implementation tasks with verifiable outcomes.
 - [x] #5 Requirement-test mapping is updated and consistent.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented scope includes canonical hours persistence, project-centric week-grid planning with grouped headers, milestone CRUD via week-grid interactions, cost rollups, and tentative/committed assignment states.

Follow-up work remains for expanded frontend automated coverage (`task-29`) and deeper capacity exception modeling (`REQ_PUSSLA_021`).
<!-- SECTION:NOTES:END -->
