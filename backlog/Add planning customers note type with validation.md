---
id: TASK-45.8
aliases:
- TASK-45.8
priority: 53
---

# Add planning/customers/ note type with validation
#story/backlog Per ADR-010 (task-45.1): give customers their own note type at planning/customers/<slug>.md, following the same pattern as the role catalog in ADR-006 (planning/roles/<role_id>.md) — frontmatter…

Requires < [[Write ADR-010 and ADR-011, update ADR-008 status]]

Requires < [[Port validation logic from validate_planning_data.py to core]]

Parent: [[Rust core-first architecture for Pussla (framkalla pattern)]]

## Description

Per ADR-010 (task-45.1): give customers their own note type at planning/customers/<slug>.md, following the same pattern as the role catalog in ADR-006 (planning/roles/<role_id>.md) — frontmatter for machine-readable fields (customer_id, name, contacts), Markdown body for freeform notes. Add the type to core/'s domain models (task-45.2) and a validation rule (customer_id format/uniqueness, referenced customer_id on a project must exist) to core/'s validation module (task-45.3). Add at least one example fixture under tst-data/planning/customers/ and reference it from an existing project fixture.

## Acceptance Criteria
- Customer model exists in core/ with serde parsing matching the roles/ pattern
- Validation rejects an unknown customer_id referenced by a project, and a duplicate customer_id
- At least one tst-data/planning/customers/<slug>.md fixture exists and is referenced by an existing project fixture
- cargo test covers the new model and validation rule
