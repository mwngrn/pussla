---
id: task-30
aliases:
- task-30
priority: 30
---

# PM end-to-end verification and closure
#story/done Run final integration verification for project management scope and close umbrella task(s) with documented outcomes.

Requires < [[Project management grid, grouped headers, footer, and summary]]

Requires < [[Milestones CRUD, validation, and cross-view consistency]]

Requires < [[Project cost rollups from planned hours and rates]]

Requires < [[Assignment states tentative vs committed]]

Requires < [[PM frontend tests for hours editing and representation policy]]

## Description

Run final integration verification for project management scope and close umbrella task(s) with documented outcomes.

## Acceptance Criteria
- ✓ Backend and frontend test suites pass for PM scope.
- ✓ Requirement coverage and known gaps are documented.
- ✓ Remaining non-delivered requirements are explicitly tracked for follow-up.
- ✓ Umbrella task is updated with completion status and evidence.

## Implementation Notes

Requirement coverage and status were synchronized in `reqs/source/project-management.rst` and `reqs/source/tests.rst`.

Known/explicit follow-ups:
- `task-29`: dedicated frontend automated tests for project management representation and interactions.
- `REQ_PUSSLA_021`: configurable per-person capacity overrides/calendar exceptions beyond the current default-capacity handling.

Umbrella closure evidence was added to `task-21`; this task remains open while follow-up verification and additional frontend tests are pending.
