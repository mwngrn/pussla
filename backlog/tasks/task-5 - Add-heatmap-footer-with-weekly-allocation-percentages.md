---
id: task-5
title: Add heatmap footer with weekly allocation percentages
status: Done
assignee:
  - codex
created_date: '2026-02-22 18:07'
updated_date: '2026-02-22 18:28'
labels:
  - dashboard
  - ui
  - reporting
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add a footer row at the bottom of the dashboard heatmap that summarizes allocation percentage per week across the displayed data.

Scope:
- Add footer section below the heatmap grid.
- Show one allocation percentage summary per week column.
- Keep existing heatmap interactions and layout behavior intact.
- Ensure footer values align with week columns shown in the header.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Dashboard renders a footer at the bottom of the heatmap.
- [x] #2 Footer shows allocation percentages per week aligned to each week column.
- [x] #3 Percentage calculation logic for weekly footer is documented and consistent with existing allocation semantics.
- [x] #4 Footer updates correctly when filters/view options change the displayed data.
- [x] #5 No regression in existing heatmap rendering and interactions.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Reuse existing dashboard render flow so footer recomputes on each filter/search update.
2. Compute weekly allocation percentages across displayed users for each week column.
3. Render a `tfoot` footer row with one percentage cell per week column to match alignment.
4. Keep calculation semantics documented and consistent with existing allocation percentages.
5. Verify with JS unit tests and existing Python test suite.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented heatmap footer row in `src/dashboard/index.html` using `tfoot` with month-aligned `colspan` cells.

Footer values reuse `calculateMonthAllocationPercentages(...)`, keeping semantics consistent with month-header percentages.

Documented percentage semantics inline: average `total_load` across displayed user-week slots per month group.

Footer is recomputed on each `render()` call, so search/filter changes update values automatically.

Verification: `node --test tests/test_week_format.js tests/test_header_groups.js` passed; `python -m unittest discover -s tests -p 'test_*.py'` passed (3 tests).

Adjusted implementation based on user clarification: footer now shows weekly allocation percentages (one value per week column), not monthly.

Added `calculateWeekAllocationPercentages(...)` in `src/dashboard/header_groups.js` and wired footer rendering in `src/dashboard/index.html`.

Footer label changed to `Veckobeläggning`; values align 1:1 with week columns.

Added test coverage in `tests/test_header_groups.js` for weekly percentage aggregation.

Verification rerun: `node --test tests/test_week_format.js tests/test_header_groups.js` passed; `python -m unittest discover -s tests -p 'test_*.py'` passed (3 tests).
<!-- SECTION:NOTES:END -->
