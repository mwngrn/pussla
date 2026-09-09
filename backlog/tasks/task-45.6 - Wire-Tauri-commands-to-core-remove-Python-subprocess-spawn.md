---
id: TASK-45.6
title: 'Wire Tauri commands to core/, remove Python subprocess spawn'
status: Done
assignee:
  - claude-code/claude-sonnet-5
created_date: '2026-09-09 18:30'
updated_date: '2026-09-09 19:06'
labels: []
dependencies:
  - TASK-45.3
  - TASK-45.4
  - TASK-45.5
parent_task_id: TASK-45
type: task
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Rewrite src/frontend/src-tauri/src/main.rs: remove all Python-subprocess machinery (BackendProcess, spawn_backend, reserve_port, wait_for_backend, resolve_runtime_paths' python-script resolution) and the window.location.replace(url) redirect-to-localhost flow. Replace with #[tauri::command] functions that call core/ directly in-process: get_dashboard_data, list_people, get_person, save_allocation, list_projects, get_project, save_project, validate, sync_status, sync_publish, sync_refresh — one command per current dashboard.ts API call (read src/frontend/src/api/dashboard.ts fully first to enumerate the exact current surface).

Keep the existing data-folder resolution/first-run logic (ADR-007: user-selected external data folder, remembered in app settings) — only the "how do we run the backend" mechanism changes, not the data-folder UX.

Verify with `cargo check` and `cargo clippy` in src-tauri/ (do not run `pnpm tauri dev` yourself — hand Mathias the exact command for his own manual verification per this repo's AGENTS.md/dev-wow tester split).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 main.rs no longer references python3, Child process spawning, or port reservation
- [x] #2 One #[tauri::command] exists per current dashboard.ts API call, calling core/ in-process
- [x] #3 cargo check and cargo clippy pass in src-tauri/ with no new warnings
- [x] #4 Data-folder selection/first-run/remembered-folder behavior from ADR-007 is unchanged
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Rewrote src/frontend/src-tauri/src/main.rs from scratch: removed all Python-subprocess machinery (BackendProcess, spawn_backend, reserve_port, wait_for_backend, dashboard_script/static_dir resolution, the window.location.replace(url) redirect) and replaced it with 7 #[tauri::command]s calling pussla-core in-process, matching the exact API surface dashboard.ts actually calls (verified by reading the whole file, not the task description's guessed list — the real surface is 6 fetch()-based functions, not the originally-assumed list/get_person/list_projects style):
- get_dashboard_data(include_pii) -> DashboardData
- validate_dataset() -> DatasetValidation (added — dashboard.ts doesn't call this yet, but task-45.7 will need somewhere to surface validation, and core::validation was otherwise unreachable from the desktop app)
- sync_status / sync_publish(summary) / sync_refresh -> SyncStatus/PublishResult/RefreshResult (wrapped in an `{ok: true, ...}` envelope to match PublishResult/RefreshResult's TS shape)
- update_allocation(payload) / update_project(payload) -> wrapping core::write's results into {ok, updated: {...}} matching UpdateAllocationResult/UpdateProjectResult's TS shape

Added pussla-core as a path dependency (`../../../core`) in src-tauri/Cargo.toml, plus chrono (with the "clock" feature, only needed here — core's own chrono dependency doesn't need it since ISO-week math is calendar-only) for the generated_at timestamp.

Non-trivial bit: ProjectUpdatesPayload's hourly_rate/start_week_override/end_week_override need to distinguish "field not sent" (leave alone) from "field sent as null" (clear it) to match core::write::ProjectMetadataUpdate's Option<Option<T>> contract — implemented via the standard serde `#[serde(default, deserialize_with = "...")]` double-Option trick (deserialize_present()).

Kept ADR-007's data-folder resolution (--data-dir flag, PUSSLA_DATA_DIR env, tst-data/ dev fallback, ~/Pussla-data) verbatim — only the "how do we serve the backend" mechanism changed. Also had to `npm install && npm run build` in src/frontend/ to produce dist/ so tauri-build's bundle.resources glob would resolve at all (cargo check/build fails without it, unrelated to my changes — dist/ simply didn't exist yet in this checkout).

cargo check and cargo clippy --all-targets both clean (fixed one needless-borrow lint). The 2 pre-existing unit tests for parse_flag_value still pass. Did not run `pnpm/npm run tauri:dev` myself (dev-wow: agent doesn't own long-lived GUI processes) — that's Mathias's manual verification step, run `npm run tauri:dev` from src/frontend/.
<!-- SECTION:FINAL_SUMMARY:END -->
