---
id: task-40
aliases:
- task-40
priority: 40
---

# Tauri desktop app shell for local Pussla services
#story/done Package the current frontend and backend into a Tauri desktop application shell that starts local services automatically and presents the UI in an app window for non-technical users.

## Description

Package the current frontend and backend into a Tauri desktop application shell that starts local services automatically and presents the UI in an app window for non-technical users.

## Acceptance Criteria
- ✓ Pussla can be launched as a Tauri desktop application without starting scripts manually.
- ✓ The Tauri app starts the required local service layer automatically and displays the UI in an application window.
- ✓ The Tauri shell uses the external selected data folder rather than bundling planning data inside the app.
- ✓ Automated or scripted verification covers Tauri desktop startup and local service lifecycle behavior.

## Implementation Plan

1. Inspect the current frontend build, startup flow, and local environment to determine the thinnest viable Tauri shell around the existing React frontend and Python backend.
2. Add a Tauri app skeleton configured to host the built React frontend and start/stop the local Python dashboard service.
3. Pass the selected data folder path into the local service startup path using the existing single-data-root model.
4. Add scripted verification for the Tauri shell wiring and document how to run/build the desktop shell in the repository.
5. Keep scope limited to the shell and service lifecycle for task-40; defer onboarding, folder switching, and installer polish to their dedicated tasks.

## Implementation Notes

Implemented an initial Tauri shell under `src/frontend/src-tauri/` with a loading window, local-port reservation, Python backend startup, backend readiness polling, and shutdown cleanup.

The shell resolves an external data folder from `--data-dir`, `PUSSLA_DATA_DIR`, repo `tst-data` in development, or `~/Pussla-data` as a fallback if present.

Updated `src/frontend/package.json` with `tauri:dev` and `tauri:build` scripts and documented the shell workflow in `README.md`.

Verification so far: `npm run build` passes with the new Tauri files present, and `cargo fmt` passes for the Rust shell code.

Blocker for full Tauri verification in this environment: `cargo check` fails on missing Linux system libraries required by Tauri (`glib-2.0` via `pkg-config`). The shell should be re-verified on a machine with Tauri GTK/WebKit dependencies installed.

Linux Tauri prerequisites were verified on Ubuntu 24.04 (`glib-2.0`, `webkit2gtk-4.1`, and `libsoup-3.0` now resolve via `pkg-config`).

Added a desktop icon at `src/frontend/src-tauri/icons/icon.png` so Tauri context generation succeeds.

`cargo check` now passes for `src/frontend/src-tauri`, so the remaining work is functional desktop-run verification and feature completion rather than missing toolchain dependencies.

End-to-end verification completed with `PUSSLA_DATA_DIR=/home/mathias/dev/pussla/tst-data npm run tauri:dev`. The Tauri shell built, launched, started the local backend, and opened a working desktop window against the selected external data folder.
