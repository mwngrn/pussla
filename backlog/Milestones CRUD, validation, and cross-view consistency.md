---
id: task-25
aliases:
- task-25
priority: 25
---

# Milestones CRUD, validation, and cross-view consistency
#story/done Implement milestone management with CRUD operations, date validation/order, and consistency across project detail and timeline overview.

Requires < [[Canonical hours data model and migration compatibility]]

## Description

Implement milestone management with CRUD operations, date validation/order, and consistency across project detail and timeline overview.

## Acceptance Criteria
 - ✓ Users can create, edit, and remove date-based milestones.
 - ✓ Invalid dates are rejected and milestones are shown in chronological order.
 - ✓ Milestone updates are reflected consistently across all milestone views.
 - ✓ Project date range derivation/override logic is implemented and visible in UI.
 - ✓ Automated tests cover milestone CRUD and consistency behavior.
