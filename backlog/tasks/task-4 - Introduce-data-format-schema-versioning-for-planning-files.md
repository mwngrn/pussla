---
id: task-4
title: Introduce data format/schema versioning for planning files
status: To Do
assignee: []
created_date: '2026-02-22 17:51'
updated_date: '2026-02-22 18:01'
labels:
  - data-model
  - validation
  - dashboard
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add explicit versioning to planning data so parsers, validators, and dashboard loading can fail clearly on incompatible formats and evolve safely over time.

Scope:
- Add `schema_version` to allocation YAML and project frontmatter formats.
- Add a root planning manifest (e.g. `tst-data/planning/FORMAT.yaml`) with `dataset_format_version`.
- Update validators/parsers to require and validate versions.
- Define compatibility behavior for unknown/newer versions and missing versions.
- Document upgrade path and versioning policy in docs.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Allocation files require `schema_version` and fail validation if missing/invalid.
- [ ] #2 Project frontmatter requires `schema_version` and fail validation if missing/invalid.
- [ ] #3 Planning root includes `dataset_format_version` manifest read by validator/parser entrypoints.
- [ ] #4 Parser/validator behavior is defined and implemented: missing version => error, unsupported newer major => error, unknown optional fields => non-fatal.
- [ ] #5 Documentation updated with versioning policy and migration guidance for future format changes.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Define version policy: major/minor semantics, supported range, and behavior on unknown/newer versions.
2. Add `dataset_format_version` manifest under planning root and wire parser/validator entrypoints to read it.
3. Add `schema_version` requirement for allocation YAML files and validate type/value.
4. Add `schema_version` requirement for project frontmatter and validate type/value.
5. Update dashboard engine and validation scripts to fail fast with clear actionable errors on incompatible versions.
6. Add tests for missing version, malformed version, supported version, and unsupported newer major.
7. Update documentation with migration steps and examples for version bumps.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Open question: should we support automatic migration helpers (best-effort upgrade) in this iteration or only strict validation with manual migration docs?

Decision pending from user; default proposal for this iteration: strict validation + clear migration docs, and treat automatic migration as a follow-up task.
<!-- SECTION:NOTES:END -->
