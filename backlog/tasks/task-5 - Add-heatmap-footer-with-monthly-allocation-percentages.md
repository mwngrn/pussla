---
id: task-5
title: Add heatmap footer with monthly allocation percentages
status: To Do
assignee: []
created_date: '2026-02-22 18:07'
labels:
  - dashboard
  - ui
  - reporting
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add a footer row at the bottom of the dashboard heatmap that summarizes allocation percentage per month across the displayed data.

Scope:
- Add footer section below the heatmap grid.
- Show one monthly allocation percentage summary per month group.
- Keep existing heatmap interactions and layout behavior intact.
- Ensure footer values align with month groupings shown in headers.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Dashboard renders a footer at the bottom of the heatmap.
- [ ] #2 Footer shows monthly allocation percentages aligned to month column groups.
- [ ] #3 Percentage calculation logic for monthly footer is documented and consistent with existing allocation semantics.
- [ ] #4 Footer updates correctly when filters/view options change the displayed data.
- [ ] #5 No regression in existing heatmap rendering and interactions.
<!-- AC:END -->
