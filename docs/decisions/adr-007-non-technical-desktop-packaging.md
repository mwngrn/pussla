# Design Choice: Non-Technical Desktop Packaging with External Data Folder

## Status
**Accepted**

## Context
Pussla currently runs as a local web application with a React frontend and a Python backend. This is workable for technical users, but it still exposes too much operational complexity for non-technical planners:

- starting scripts manually
- understanding local server behavior
- managing a separate browser window
- knowing where planning data lives
- using Git concepts such as commit, push, and pull

At the same time, the core Pussla model depends on a user-owned planning dataset that should remain portable, inspectable, backup-friendly, and separable from the installed application.

## Decision
Pussla shall be packaged as a **desktop application** for non-technical users, while keeping the planning data in an **external user-selected data folder**.

The preferred desktop packaging technology shall be **Tauri**.

The desktop application shall:

- bundle the frontend and backend into one installed product
- start the local service layer automatically
- present the UI in an app window rather than requiring the user to open a browser manually
- guide the user through selecting or creating a data folder on first run
- expose non-technical collaboration actions such as `Publish` and `Refresh` instead of raw Git terminology

The initial desktop delivery target shall be **Linux**.

The external data folder shall remain the source of truth for planning content and contain the planning and identity structures needed by the app.

## Rationale

### 1. Better fit for non-technical users
A desktop app matches the mental model of normal business software: install, open, choose data, start working.

### 2. Preserve planning-as-code
Keeping data outside the installed app preserves portability, Git traceability, backup friendliness, and compatibility with technical workflows.

### 3. Safer upgrades
Application updates and planning data updates remain separate concerns. Reinstalling or upgrading the app must not risk user planning data.

### 4. Clearer multi-dataset support
An explicit external data folder makes it possible to switch between datasets without reinstalling the application or modifying internal app files.

### 5. Controlled simplification of Git
Non-technical users should interact with collaboration workflows through outcome-oriented actions like `Publish changes` and `Get latest planning data`, while the app handles Git operations internally.

## Packaging Direction

### Recommended near-term approach
Package the current architecture as a **Tauri desktop shell** around the existing local backend and React frontend.

- keep the existing Python backend logic
- keep the existing React frontend
- use Tauri for desktop windowing, native OS integration, installer packaging, and app lifecycle
- embed startup, local service hosting, and path handling inside the desktop app

This minimizes rewrite cost while making the product installable and understandable for non-technical users.

### Why Tauri over Electron
Tauri is preferred because it is a better fit for Pussla's current architecture and intended users.

- smaller install footprint
- lower runtime memory usage
- better fit for a thin desktop shell around existing local services
- native support for desktop capabilities such as file dialogs and window management without requiring a full Chromium bundle per app

Electron remains technically possible, but it is not the preferred direction for this product unless later constraints force a change.

### Data-folder model
The user chooses a data folder that contains or will contain:

- `planning/`
- `identity/`
- `planning/people/`
- `planning/projects/`
- `planning/roles/`
- `planning/skills.md`

The application owns the UX for validating and bootstrapping this structure.

### First-run scope
The first-run flow shall focus on opening an existing data folder or creating a new one.

- bootstrap shall create a complete default planning and identity structure
- the active data folder shall be remembered in local app settings
- invalid data folders shall block normal planning views and present repair guidance
- shared sync setup shall remain a later explicit workflow, not a bootstrap prerequisite

## Consequences

- A desktop shell layer must be introduced and maintained.
- The team must own a small amount of Tauri/Rust configuration in addition to the existing React and Python code.
- First-run onboarding becomes a product requirement, not an optional convenience.
- The application must manage local service lifecycle, path validation, and sync errors internally.
- Release engineering must support distributable installers for target operating systems.
- The sync model must be communicated in non-technical language while still preserving Git-backed traceability underneath.

## Non-Goals

- This decision does not require replacing Git as the underlying collaboration mechanism.
- This decision does not require rewriting the backend away from Python immediately.
- This decision does not require moving planning data into an internal database owned by the app.

## Follow-on Work

The delivery should be split into:

1. Desktop app shell and packaging
2. First-run setup and data-folder bootstrap
3. Data-folder selection, validation, and switching
4. Non-technical shared-sync setup and guidance
5. Installer and release workflow
