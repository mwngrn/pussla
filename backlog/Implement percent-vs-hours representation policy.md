---
id: task-23
aliases:
- task-23
priority: 23
---

# Implement percent-vs-hours representation policy
#story/done Implement UI policy where utilization-oriented views show derived percentages and project/time-entry editing shows hours.

Requires < [[Canonical hours data model and migration compatibility]]

## Description

Implement UI policy where utilization-oriented views show derived percentages and project/time-entry editing shows hours.

## Acceptance Criteria
 - ✓ Utilization-centric views continue to show `%` values.
 - ✓ Project-centric and time-entry editing flows show and edit `hours`.
 - ✓ Save payloads for editable views include canonical hours fields.
 - ✓ Automated tests validate representation policy behavior.
