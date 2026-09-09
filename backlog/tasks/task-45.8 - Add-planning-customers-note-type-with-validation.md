---
id: TASK-45.8
title: Add planning/customers/ note type with validation
status: To Do
assignee: []
created_date: '2026-09-09 18:30'
labels: []
dependencies:
  - TASK-45.1
  - TASK-45.3
parent_task_id: TASK-45
type: task
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Per ADR-010 (task-45.1): give customers their own note type at planning/customers/<slug>.md, following the same pattern as the role catalog in ADR-006 (planning/roles/<role_id>.md) — frontmatter for machine-readable fields (customer_id, name, contacts), Markdown body for freeform notes. Add the type to core/'s domain models (task-45.2) and a validation rule (customer_id format/uniqueness, referenced customer_id on a project must exist) to core/'s validation module (task-45.3). Add at least one example fixture under tst-data/planning/customers/ and reference it from an existing project fixture.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Customer model exists in core/ with serde parsing matching the roles/ pattern
- [ ] #2 Validation rejects an unknown customer_id referenced by a project, and a duplicate customer_id
- [ ] #3 At least one tst-data/planning/customers/<slug>.md fixture exists and is referenced by an existing project fixture
- [ ] #4 cargo test covers the new model and validation rule
<!-- AC:END -->
