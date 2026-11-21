---
id: task-41
aliases:
- task-41
priority: 41
---

# First-run setup wizard and data-folder bootstrap
#story/backlog Add a first-run onboarding flow in the Tauri desktop app that lets non-technical users choose or create a Pussla data folder and bootstraps the required planning structure when needed.

## Description

Add a first-run onboarding flow in the Tauri desktop app that lets non-technical users choose or create a Pussla data folder and bootstraps the required planning structure when needed.

## Acceptance Criteria
- First run offers the user a choice between selecting an existing data folder and creating a new one.
- Creating a new data folder bootstraps the required planning and identity structure with valid starter content.
- The app validates an existing data folder and shows actionable guidance when required content is missing or malformed.
- Automated tests cover successful bootstrap and representative validation failures.
