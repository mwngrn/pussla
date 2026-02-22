---
id: task-2
title: Change the week format to YYwWW
status: To Do
assignee: []
created_date: '2026-02-22 17:46'
updated_date: '2026-02-22 18:05'
labels:
  - dashboard
  - formatting
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Update week labels in the dashboard header to show only week numbers for readability.

Scope:
- Change only presentation format in the dashboard UI.
- Keep backend/API canonical week format unchanged (`YYYY-Www`).
- Render week labels as week number only in `Www` format (for example `W01`, `W23`, `W53`).
- Ensure formatting is applied consistently anywhere week labels are displayed in the dashboard.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Backend/API continues to use canonical ISO week keys in format `YYYY-Www`.
- [ ] #2 Dashboard week labels show week number only in `Www` format and omit year entirely.
- [ ] #3 Week columns remain ordered correctly across year boundaries.
- [ ] #4 Formatting is covered by tests for representative week values (including `W01`, `W52`, `W53`).
- [ ] #5 No regression in heatmap cell alignment or interactions after label format change.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add a formatter function in dashboard frontend code that converts canonical week keys to week-number-only labels.
2. Replace direct use of raw week keys in header rendering with formatted week number labels.
3. Verify sorting still uses canonical keys and not display strings.
4. Validate interactions and layout alignment with the new shorter labels.
5. Add/update tests for formatter behavior across edge weeks and year transitions.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Decision confirmed: week header should show week number only in `Www` format (e.g. `W01`) and skip year entirely. Year context is provided by higher-level year/month headers.
<!-- SECTION:NOTES:END -->
