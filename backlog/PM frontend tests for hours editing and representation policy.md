---
id: task-29
aliases:
- task-29
priority: 29
---

# PM frontend tests for hours editing and representation policy
#story/done Add frontend tests to verify hours editing in project/time-entry views and percent display in utilization-oriented views.

Requires < [[Implement percent-vs-hours representation policy]]

Requires < [[Project management grid, grouped headers, footer, and summary]]

## Description

Add frontend tests to verify hours editing in project/time-entry views and percent display in utilization-oriented views.

## Acceptance Criteria
- ✓ Tests verify project/time-entry views render and edit hours.
- ✓ Tests verify utilization views render derived percentages.
- ✓ Tests verify project weekly footer totals in hours.

## Implementation Notes

Added frontend logic-focused coverage in ``tests/test_representation_policy.mjs`` for:
- hours/percent conversion behavior,
- ISO date-to-week mapping used in week-grid milestone placement,
- milestone grouping/sorting by displayed week columns.
