---
id: TASK-45.7
aliases:
- TASK-45.7
priority: 52
---

# Rewire frontend API layer from fetch() to Tauri invoke()
#story/done Update src/frontend/src/api/dashboard.ts and every module under src/frontend/src/modules/ that calls it (planning/, people/, allocations/, project-management/, projects/, system/SyncPanel.tsx,…

Requires < [[Wire Tauri commands to core , remove Python subprocess spawn]]

Parent: [[Rust core-first architecture for Pussla (framkalla pattern)]]

## Description

Update src/frontend/src/api/dashboard.ts and every module under src/frontend/src/modules/ that calls it (planning/, people/, allocations/, project-management/, projects/, system/SyncPanel.tsx, settings/) to call `@tauri-apps/api/core`'s `invoke()` against the new Tauri commands from task-45.6, instead of `fetch()` against a local HTTP server. Keep the existing TypeScript interfaces (DashboardData, ProjectContext, ProjectSlot, WeeklyStats, DashboardUser, DashboardMetrics, RawAllocation, SyncStatus, SyncChange) unchanged — core/ already serializes matching JSON per task-45.4/45.5.

This task assumes the app only runs inside Tauri going forward (no bare-browser/local-HTTP-server mode) — confirm this assumption holds by checking whether run-react-dashboard.sh / running the frontend standalone in a browser is still a requirement before removing the fetch path; if Mathias still wants a browser-only dev mode, flag it back rather than silently dropping it.

## Acceptance Criteria
- ✓ dashboard.ts and all its consumers call invoke() instead of fetch()
- ✓ No behavior change in any module's UI — same data shapes flow through unchanged
- ✓ TypeScript compiles with no new errors (npm run build in src/frontend/)

## Final Summary

Rewired src/frontend/src/api/dashboard.ts's 6 fetcher functions (fetchDashboardData, fetchSyncStatus, publishPlanningChanges, refreshPlanningData, updateAllocation, updateProject) from fetch() against a local HTTP server to `@tauri-apps/api/core`'s invoke() against the task-45.6 Tauri commands, via a small invokeOrThrow<T>() helper that normalizes invoke()'s string rejections into thrown Error objects (matching the previous fetch-path's `throw new Error(...)` behavior so existing catch/`.message` call sites keep working unchanged).

Added `#[tauri::command(rename_all = "snake_case")]` to all 7 commands in src-tauri/src/main.rs (a follow-on fix needed here, not anticipated in task-45.6 — Tauri's default JS-side argument casing is camelCase, which would have required `includePii`/etc. on the JS side and silently drifted from this codebase's snake_case JSON convention everywhere else).

Confirmed via grep that every one of the 9 consumer files (App.tsx, SettingsPage.tsx, SyncPanel.tsx, ProjectsPage.tsx, ProjectManagementPage.tsx, PeoplePage.tsx, AllocationsPage.tsx, GanttPage.tsx, EditWeekDialog.tsx, UtilizationPage.tsx) only imports dashboard.ts's exported functions/types by name — none needed any changes, since the exported function signatures and TypeScript interfaces (DashboardData, SyncStatus, PublishResult, etc.) are unchanged. This made the task much smaller in practice than "every module" suggested: one file edited, nine untouched.

`npm run build` (tsc -b && vite build) passes with zero new errors. grep confirms no remaining `fetch("/api/...")` calls anywhere in src/.

Resolved the task's own flagged question (run-react-dashboard.sh / browser-only mode): it's not being kept, but not by a decision made in this task — it depends entirely on src/dashboard/run_dashboard.py, which ADR-011 (approved by Mathias) already commits to deleting once desktop parity is verified (task-45.9), and the axum server/ that could serve a browser-only mode without Tauri is explicitly deferred to a later phase. So browser-only mode goes away as a consequence of the already-approved architecture, not an independent call made here — task-45.9's existing scope (remove run-react-dashboard.sh alongside the Python backend) already covers it.
