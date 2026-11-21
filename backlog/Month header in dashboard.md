---
id: task-1
aliases:
- task-1
priority: 1
---

# Month header in dashboard
#story/done Add grouped calendar headers above week columns in the dashboard heatmap so users can scan planning by year and month while keeping week-level detail.

## Description

Add grouped calendar headers above week columns in the dashboard heatmap so users can scan planning by year and month while keeping week-level detail.

Scope:
- Add a top header row for year grouping.
- Add a second header row for month grouping (month name only, no year).
- Show monthly allocation percentage in the month header for each month group.
- Keep the existing week header row underneath.
- Ensure grouping handles year boundaries correctly.

## Acceptance Criteria
- ✓ Dashboard table renders a top year header row above month and week headers.
- ✓ Dashboard table renders a month header row between year and week headers.
- ✓ Year headers span exactly the number of week columns that belong to that year.
- ✓ Month headers span exactly the number of week columns that belong to that month within each year.
- ✓ Month labels show month only (for example `Jan`, `Feb`) without year.

- ✓ Month headers also show the allocation percentage for that month group.
- ✓ Existing week-level header and heatmap data rendering remain intact.

- ✓ Behavior is covered by tests for year/month grouping logic including year boundary cases and monthly header percentage rendering.

## Implementation Plan

1. Inspect current week header rendering in `src/dashboard/index.html` and identify where grouped headers can be inserted.
2. Add a utility that maps each ISO week (`YYYY-Www`) to year and month using ISO week start date.
3. Build grouped year metadata (label + span) and month metadata (label + span) from ordered week list.
4. Add monthly allocation percentage aggregation per month group based on displayed heatmap data.
5. Render three-row table headers: row 1 years, row 2 months (month name + month allocation %), row 3 individual week labels.
6. Add/update tests for grouping logic, year boundary behavior, and monthly header percentage rendering.

## Implementation Notes

Started implementation on 2026-02-22: adding 3-row header (year/month/week) and month allocation percentage in month header.

Implemented grouped calendar headers in `src/dashboard/index.html`: year row, month row, week row.

Added month allocation percentage display in month headers, computed from currently displayed users/weeks.

Added shared grouping/aggregation utility `src/dashboard/header_groups.js`.

Added tests in `tests/test_header_groups.js` covering year/month grouping across boundaries and monthly percentage aggregation.

Verification: `node --test tests/test_week_format.js tests/test_header_groups.js` passed; `python -m unittest discover -s tests -p 'test_*.py'` passed (3 tests).
