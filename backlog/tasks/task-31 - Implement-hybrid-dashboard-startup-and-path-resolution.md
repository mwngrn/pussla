---
id: task-31
title: Implement hybrid dashboard startup and path resolution
status: To Do
assignee: []
created_date: '2026-03-01 00:00'
updated_date: '2026-03-01 00:00'
labels:
  - deployment
  - dashboard
  - cli
  - requirements
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement hybrid startup behavior for the local dashboard so non-developers can start from the dataset folder without flags, while advanced users can pass explicit paths.

Requirement mapping:
- `REQ_PUSSLA_044`: hybrid startup mode (auto-detect + explicit paths)
- `REQ_PUSSLA_045`: explicit path precedence and deterministic resolution
- `REQ_PUSSLA_046`: actionable startup errors when paths/data roots are invalid
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Dashboard startup supports auto-detection of data roots from current working directory.
- [ ] #2 Dashboard startup supports explicit path flags (`--data-dir`, `--planning-dir`, `--identity-dir`) and explicit flags take precedence over auto-detection.
- [ ] #3 Startup logs show resolved planning and identity paths so dataset selection is transparent.
- [ ] #4 Invalid/missing data roots fail fast with actionable error messages including expected structure and example commands.
- [ ] #5 Automated tests cover auto-detect success/failure, explicit-path precedence, and error messaging behavior.
- [ ] #6 README/startup docs are updated with both startup modes and examples.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Refactor path resolution in `src/dashboard/run_dashboard.py` to support cwd auto-detect plus explicit override precedence.
2. Add startup readiness checks for required planning/identity structure before serving.
3. Add clear error formatting with remediation examples for both modes.
4. Add unit tests for path resolution and startup error cases.
5. Update README and relevant docs with new startup guidance.
<!-- SECTION:PLAN:END -->
