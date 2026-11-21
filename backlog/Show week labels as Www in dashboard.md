---
id: task-2
aliases:
- task-2
priority: 2
---

# Show week labels as Www in dashboard
#story/done Update week labels in the dashboard header to show only week numbers for readability.

## Description

Update week labels in the dashboard header to show only week numbers for readability.

Scope:
- Change only presentation format in the dashboard UI.
- Keep backend/API canonical week format unchanged (`YYYY-Www`).
- Render week labels as week number only in `Www` format (for example `W01`, `W23`, `W53`).
- Ensure formatting is applied consistently anywhere week labels are displayed in the dashboard.

## Acceptance Criteria
- ✓ Backend/API continues to use canonical ISO week keys in format `YYYY-Www`.
- ✓ Dashboard week labels show week number only in `Www` format and omit year entirely.
- ✓ Week columns remain ordered correctly across year boundaries.
- ✓ Formatting is covered by tests for representative week values (including `W01`, `W52`, `W53`).
- ✓ No regression in heatmap cell alignment or interactions after label format change.

## Implementation Plan

1. Add a formatter function in dashboard frontend code that converts canonical week keys to week-number-only labels.
2. Replace direct use of raw week keys in header rendering with formatted week number labels.
3. Verify sorting still uses canonical keys and not display strings.
4. Validate interactions and layout alignment with the new shorter labels.
5. Add/update tests for formatter behavior across edge weeks and year transitions.

## Implementation Notes

Decision confirmed: week header should show week number only in `Www` format (e.g. `W01`) and skip year entirely. Year context is provided by higher-level year/month headers.

Started implementation on 2026-02-22: updating dashboard week header labels to `Www` format while preserving canonical backend week keys.

Implemented in `src/dashboard/index.html` by using shared `formatWeekLabel` for header and details week display.

Added shared formatter in `src/dashboard/week_format.js`.

Added automated formatter test in `tests/test_week_format.js` and verified via `node --test tests/test_week_format.js` (pass).

Verified existing Python tests with `python -m unittest discover -s tests -p 'test_*.py'` (3 passed).
