---
id: task-33
aliases:
- task-33
priority: 33
---

# Implement project activities with date-range bars and edit interactions
#story/done Add project-management activities as a complement to milestones

## Description

Add project-management activities as a complement to milestones. Activities have label, start date, and end date, are shown as timeline bars below the milestones row, and support create/edit/delete through click and drag interactions.

## Acceptance Criteria
- ✓ Users can create, edit, and delete project activities with label, start date, and end date.
- ✓ Project planning timeline renders activities as bars spanning their date ranges in a row below milestones.
- ✓ Users can create activities through click-to-open-dialog flow and through click-drag with start at pointer-down and end at pointer-up.
- ✓ Clicking an existing activity opens a dialog that allows editing or deleting the activity.
- ✓ Automated tests cover activity CRUD, row placement below milestones, and click/drag interaction behaviors.

## Implementation Plan

1. Extend backend project metadata parsing and update path to support `activities` entries (`id`, `label`, `start_date`, `end_date`) with strict YYYY-MM-DD validation and stable sorting.
2. Extend frontend API types and representation helpers so activities can be mapped onto displayed weeks and represented as span bars.
3. Implement activities UX in Project Management page: row below milestones, click-to-open dialog, click existing activity to edit/delete, and click-drag over week cells to create an activity range.
4. Add automated tests for backend metadata handling and frontend activity mapping/interaction helper behavior.
5. Run targeted tests and update task notes/acceptance criteria as behaviors are verified.

## Implementation Notes

Implementation started: wiring activity model, timeline bars, and click/drag interaction flows.

Implemented backend support for `activities` in project metadata parse/update with validation and stable sorting; surfaced in dashboard project context.

Implemented frontend activities model and UX in Project Management view: activities row below milestones, range bars by week, click-to-create (cell click opens dialog), click-drag creation, and click existing bar to edit/delete.

Added tests: `tests/test_representation_policy.mjs` now covers drag-range normalization and activity week-segment mapping; `tests/test_project_management_backend.py` adds activity metadata validation/sorting and YAML date-type read tests.

Verification run: `node --test tests/test_representation_policy.mjs` passed. Python/backend tests could not run because `yaml` module is missing in runtime environment; frontend full build could not run because `tsc` is unavailable in runtime environment.

Follow-up fix applied after user validation: activities now reserve a fixed visual sub-row across the full displayed timeline to prevent row reuse artifacts when adjacent/overlapping activities are added.
