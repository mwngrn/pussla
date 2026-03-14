---
id: task-42
title: Data-folder management in desktop app
status: To Do
assignee: []
created_date: '2026-03-14 20:11'
updated_date: '2026-03-14 20:18'
labels:
  - desktop
  - ux
  - data-model
  - workflow
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Allow users to view, validate, switch, and reopen the active Pussla data folder from the Tauri desktop application without touching internal app files.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The app shows which data folder is currently active.
- [ ] #2 The user can switch to another data folder from the GUI.
- [ ] #3 Switching data folders reloads the application state safely and updates validation/status feedback.
- [ ] #4 Automated tests cover active-folder display, switching, and invalid-folder handling.
<!-- AC:END -->
