---
id: task-42
aliases:
- task-42
priority: 42
---

# Data-folder management in desktop app
#story/backlog Allow users to view, validate, switch, and reopen the active Pussla data folder from the Tauri desktop application without touching internal app files.

## Description

Allow users to view, validate, switch, and reopen the active Pussla data folder from the Tauri desktop application without touching internal app files.

## Acceptance Criteria
- The app shows which data folder is currently active.
- The user can switch to another data folder from the GUI.
- Switching data folders reloads the application state safely and updates validation/status feedback.
- Automated tests cover active-folder display, switching, and invalid-folder handling.
