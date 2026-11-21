---
id: task-39
aliases:
- task-39
priority: 39
---

# Non-technical desktop packaging and onboarding
#story/backlog Deliver Pussla as a Tauri-based desktop application for non-technical users with an external user-selected data folder, guided first-run setup, and collaboration flows that do not require terminal…

Requires < [[Tauri desktop app shell for local Pussla services]]

Requires < [[First-run setup wizard and data-folder bootstrap]]

Requires < [[Data-folder management in desktop app]]

Requires < [[Non-technical shared sync setup and guidance]]

Requires < [[Installer and release packaging for non-technical users]]

## Description

Deliver Pussla as a Tauri-based desktop application for non-technical users with an external user-selected data folder, guided first-run setup, and collaboration flows that do not require terminal usage.

## Acceptance Criteria
- Subtasks are defined for desktop app shell, first-run setup, data-folder management, shared sync setup, and release packaging.
- The delivered solution runs as an installable desktop application with the planning data kept outside the installed app.
- Documentation and onboarding material explain how non-technical users install the app, choose a data folder, and collaborate safely.

## Implementation Plan

1. Deliver the Tauri desktop shell in task-40 so Pussla can run as an installed desktop product.
2. Add first-run setup and data-folder bootstrap in task-41.
3. Add data-folder visibility and switching in task-42.
4. Add shared-sync setup and non-technical collaboration guidance in task-43.
5. Deliver Tauri installer/release packaging in task-44 and reconcile docs/onboarding before closing the parent task.
