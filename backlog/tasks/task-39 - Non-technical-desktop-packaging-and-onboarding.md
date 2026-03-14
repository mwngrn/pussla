---
id: task-39
title: Non-technical desktop packaging and onboarding
status: To Do
assignee: []
created_date: '2026-03-14 20:11'
updated_date: '2026-03-14 20:18'
labels:
  - desktop
  - deployment
  - ux
  - workflow
  - requirements
dependencies:
  - task-40
  - task-41
  - task-42
  - task-43
  - task-44
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Deliver Pussla as a Tauri-based desktop application for non-technical users with an external user-selected data folder, guided first-run setup, and collaboration flows that do not require terminal usage.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Subtasks are defined for desktop app shell, first-run setup, data-folder management, shared sync setup, and release packaging.
- [ ] #2 The delivered solution runs as an installable desktop application with the planning data kept outside the installed app.
- [ ] #3 Documentation and onboarding material explain how non-technical users install the app, choose a data folder, and collaborate safely.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Deliver the Tauri desktop shell in task-40 so Pussla can run as an installed desktop product.
2. Add first-run setup and data-folder bootstrap in task-41.
3. Add data-folder visibility and switching in task-42.
4. Add shared-sync setup and non-technical collaboration guidance in task-43.
5. Deliver Tauri installer/release packaging in task-44 and reconcile docs/onboarding before closing the parent task.
<!-- SECTION:PLAN:END -->
