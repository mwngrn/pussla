---
id: task-34
title: Add drag-to-apply allocation across multiple weeks in project management
status: Done
assignee: []
created_date: '2026-03-03 17:50'
updated_date: '2026-03-03 17:52'
labels:
  - project-management
  - frontend
  - ux
  - allocation
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Enable dragging across week cells for a selected person in Project Management view to apply the same hours/state allocation over a week range in one action, while retaining single-week click editing.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Users can click-drag across week cells for one person and open a bulk allocation dialog for the selected week range.
- [x] #2 Bulk save applies the same hours and state to each week in the selected range for the active project.
- [x] #3 Single-week click editing still works and is not accidentally triggered when drag selection completes.
- [x] #4 Selected drag range is visually highlighted before save.
- [x] #5 Automated tests cover week-range helper behavior used for bulk application flow.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add drag selection state for allocation cells scoped to one user row and normalize selected week bounds.
2. Render visual highlight for selected person/week range while dragging.
3. Introduce bulk allocation dialog (hours + state) opened on drag-release when range spans multiple weeks.
4. Apply bulk save across selected weeks for active project, preserving existing single-week editor behavior and suppressing accidental single-click open after drag.
5. Add automated tests for week-range helper behavior and run targeted frontend tests.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented drag selection on allocation cells per person row in Project Management view; drag-release across multiple weeks opens a bulk allocation dialog.

Bulk dialog applies same hours/state across the selected inclusive week range using existing updateAllocation API per week.

Preserved single-week click editor and added suppression window after drag release to prevent accidental single-cell dialog opening.

Added visual range highlight during drag and week-range count in bulk dialog.

Added `listWeeksInRange` helper in representation policy and test coverage in `tests/test_representation_policy.mjs`.

Verification: `node --test tests/test_representation_policy.mjs` passed; `npm run -s build` in `src/frontend` passed.
<!-- SECTION:NOTES:END -->
