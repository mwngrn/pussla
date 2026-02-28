---
id: task-23
title: Implement percent-vs-hours representation policy
status: Done
assignee:
  - codex
created_date: '2026-02-28 10:00'
updated_date: '2026-02-28 10:00'
labels:
  - project-management
  - frontend
  - ux
  - REQ_PUSSLA_040
  - REQ_PUSSLA_016
dependencies:
  - task-22
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement UI policy where utilization-oriented views show derived percentages and project/time-entry editing shows hours.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
 - [x] #1 Utilization-centric views continue to show `%` values.
 - [x] #2 Project-centric and time-entry editing flows show and edit `hours`.
 - [x] #3 Save payloads for editable views include canonical hours fields.
 - [x] #4 Automated tests validate representation policy behavior.
<!-- AC:END -->
