---
id: task-22
aliases:
- task-22
priority: 22
---

# Canonical hours data model and migration compatibility
#story/done Make hours canonical for planning data and API writes while preserving compatibility for legacy percentage-based entries.

## Description

Make hours canonical for planning data and API writes while preserving compatibility for legacy percentage-based entries.

## Acceptance Criteria
 - ✓ Write path accepts and persists `planned_hours` and `capacity_hours` as canonical values.
 - ✓ Read path exposes hours + derived percentage for dashboard/frontends.
 - ✓ Legacy `load`-only entries remain readable and are mapped deterministically.
 - ✓ Validation/linting supports canonical hours and detects over-allocation by hours vs capacity.
 - ✓ Automated tests cover both canonical-hours and backward-compatibility cases.
