---
id: task-41
title: First-run setup wizard and data-folder bootstrap
status: To Do
assignee: []
created_date: '2026-03-14 20:11'
updated_date: '2026-03-14 20:18'
labels:
  - desktop
  - ux
  - bootstrap
  - data-model
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add a first-run onboarding flow in the Tauri desktop app that lets non-technical users choose or create a Pussla data folder and bootstraps the required planning structure when needed.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 First run offers the user a choice between selecting an existing data folder and creating a new one.
- [ ] #2 Creating a new data folder bootstraps the required planning and identity structure with valid starter content.
- [ ] #3 The app validates an existing data folder and shows actionable guidance when required content is missing or malformed.
- [ ] #4 Automated tests cover successful bootstrap and representative validation failures.
<!-- AC:END -->
