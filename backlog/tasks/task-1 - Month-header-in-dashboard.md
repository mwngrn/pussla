---
id: task-1
title: Month header in dashboard
status: To Do
assignee: []
created_date: '2026-02-22 17:45'
updated_date: '2026-02-22 18:08'
labels:
  - dashboard
  - ui
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add grouped calendar headers above week columns in the dashboard heatmap so users can scan planning by year and month while keeping week-level detail.

Scope:
- Add a top header row for year grouping.
- Add a second header row for month grouping (month name only, no year).
- Show monthly allocation percentage in the month header for each month group.
- Keep the existing week header row underneath.
- Ensure grouping handles year boundaries correctly.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Dashboard table renders a top year header row above month and week headers.
- [ ] #2 Dashboard table renders a month header row between year and week headers.
- [ ] #3 Year headers span exactly the number of week columns that belong to that year.
- [ ] #4 Month headers span exactly the number of week columns that belong to that month within each year.
- [ ] #5 Month labels show month only (for example `Jan`, `Feb`) without year.

- [ ] #6 Month headers also show the allocation percentage for that month group.
- [ ] #7 Existing week-level header and heatmap data rendering remain intact.

- [ ] #8 Behavior is covered by tests for year/month grouping logic including year boundary cases and monthly header percentage rendering.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Inspect current week header rendering in `src/dashboard/index.html` and identify where grouped headers can be inserted.
2. Add a utility that maps each ISO week (`YYYY-Www`) to year and month using ISO week start date.
3. Build grouped year metadata (label + span) and month metadata (label + span) from ordered week list.
4. Add monthly allocation percentage aggregation per month group based on displayed heatmap data.
5. Render three-row table headers: row 1 years, row 2 months (month name + month allocation %), row 3 individual week labels.
6. Add/update tests for grouping logic, year boundary behavior, and monthly header percentage rendering.
<!-- SECTION:PLAN:END -->
