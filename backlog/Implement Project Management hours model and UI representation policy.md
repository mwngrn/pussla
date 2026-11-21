---
id: task-21
aliases:
- task-21
priority: 21
---

# Implement Project Management hours model and UI representation policy
#story/done Umbrella task for implementing canonical hours-based planning and the percent-vs-hours representation policy.

Requires < [[Canonical hours data model and migration compatibility]]

Requires < [[Implement percent-vs-hours representation policy]]

Requires < [[Project management grid, grouped headers, footer, and summary]]

Requires < [[Milestones CRUD, validation, and cross-view consistency]]

## Description

Umbrella task for implementing canonical hours-based planning and the percent-vs-hours representation policy.

Scope:
- Canonical storage/write paths in hours (`planned_hours`, `capacity_hours`) with backward compatibility for legacy `load`.
- Utilization-oriented views display derived percentages.
- Project-centric and time-entry views display and edit hours.
- Ensure requirements and tests are synchronized.

## Acceptance Criteria
 - ✓ Subtasks for data/API, frontend representation, milestones, costs, and assignment states are defined and linked.
 - ✓ Canonical hours write/read path is implemented and verified with tests.
 - ✓ Representation policy (`REQ_PUSSLA_040`) is implemented and tested.
 - ✓ Project management features (`REQ_PUSSLA_024-032`) have tracked implementation tasks with verifiable outcomes.
 - ✓ Requirement-test mapping is updated and consistent.

## Implementation Notes

Implemented scope includes canonical hours persistence, project-centric week-grid planning with grouped headers, milestone CRUD via week-grid interactions, cost rollups, and tentative/committed assignment states.

Follow-up work remains for expanded frontend automated coverage (`task-29`) and deeper capacity exception modeling (`REQ_PUSSLA_021`).
