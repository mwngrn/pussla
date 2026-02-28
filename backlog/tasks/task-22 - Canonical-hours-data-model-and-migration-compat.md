---
id: task-22
title: Canonical hours data model and migration compatibility
status: Done
assignee:
  - codex
created_date: '2026-02-28 10:00'
updated_date: '2026-02-28 10:00'
labels:
  - project-management
  - backend
  - data-model
  - migration
  - REQ_PUSSLA_017
  - REQ_PUSSLA_018
  - REQ_PUSSLA_022
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Make hours canonical for planning data and API writes while preserving compatibility for legacy percentage-based entries.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
 - [x] #1 Write path accepts and persists `planned_hours` and `capacity_hours` as canonical values.
 - [x] #2 Read path exposes hours + derived percentage for dashboard/frontends.
 - [x] #3 Legacy `load`-only entries remain readable and are mapped deterministically.
 - [x] #4 Validation/linting supports canonical hours and detects over-allocation by hours vs capacity.
 - [x] #5 Automated tests cover both canonical-hours and backward-compatibility cases.
<!-- AC:END -->
