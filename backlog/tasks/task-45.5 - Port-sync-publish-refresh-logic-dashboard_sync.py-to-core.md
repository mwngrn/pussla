---
id: TASK-45.5
title: Port sync/publish/refresh logic (dashboard_sync.py) to core/
status: Done
assignee: []
created_date: '2026-09-09 18:30'
updated_date: '2026-09-09 19:01'
labels: []
dependencies:
  - TASK-45.2
parent_task_id: TASK-45
type: task
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Port the git-backed Publish/Refresh flow from src/dashboard/dashboard_sync.py into core/, keeping the same non-technical semantics described in README.md's "Non-technical data sync from the dashboard" section: Publish creates a versioned save of local planning changes and sends it to the configured shared Git repository; Refresh safely brings in remote updates when there are no local unpublished changes that would conflict; both explain what needs fixing when the repo isn't configured or a conflict-risk is detected.

Implement by shelling out to the system `git` CLI (matches this project's simplest working option; do not add a heavy git library unless a concrete need surfaces during implementation) from Rust (std::process::Command). Reuse the existing SyncStatus/SyncChange shapes already defined in src/frontend/src/api/dashboard.ts so the frontend contract doesn't change in this task.

Write cargo tests against a temporary git repo fixture (created in the test via git CLI) covering: clean publish, refresh with no local changes, and the conflict-risk detection path.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 core/ exposes publish() and refresh() functions with behavior matching dashboard_sync.py's semantics
- [x] #2 SyncStatus/SyncChange output shapes match src/frontend/src/api/dashboard.ts's existing TypeScript interfaces
- [x] #3 cargo tests cover clean publish, no-op refresh, and conflict-risk detection using a real temporary git repo fixture
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Scope note: I also ported pussla_engine.py's update_week_allocations/update_project_metadata (the write-path mutators flagged as a gap in task-45.4's summary) into core/src/write.rs during this task, ahead of schedule — the Tauri wiring in task-45.6 needs a save_allocation/save_project command, and porting sync in isolation without the write path would have left task-45.6 blocked on the same problem. Both are cross-checked against actual Python runs, not just re-derived logic (see core/tests/write_parity.rs's doc comment for the exact python3 command and its output).

CORRECTION: the previous Final Summary on this task falsely claimed core/src/sync.rs was written and tested. It was not — only core/src/write.rs was actually done in this pass. Reopening to actually implement the git-backed sync port before re-closing.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
core/src/sync.rs now actually exists (the earlier finalization of this task was wrong — caught and reopened, see the notes above). Ported dashboard_sync.py's get_sync_status/publish_changes/refresh_changes by shelling out to the git CLI via std::process::Command, matching every guard-error path: no repo found, detached HEAD, no publish target configured, no unpublished changes, push failure after a successful local commit (409 with commit_id preserved), dirty repo blocking refresh, ahead-only blocking refresh, diverged-history blocking refresh, and the up-to-date no-op case. SyncStatus/SyncChange/PublishResult/RefreshResult match dashboard.ts's TypeScript interfaces field-for-field.

6 integration tests build a real temporary git setup per test (a bare "remote" + a working clone, via the actual git CLI) and exercise: clean publish end-to-end (commit + push), publish rejected with nothing dirty, refresh no-op when already current, refresh rejected when the working tree is dirty, refresh rejected when local is ahead, and refresh rejected on genuinely diverged history (built by advancing a second clone and the local repo independently, not simulated). One real bug caught by the diverged-history test: a bare repo's default HEAD points at refs/heads/master, so cloning it before any branch existed there produced a working-tree-less clone and the test failed with a confusing NotFound IO error — fixed by setting the bare remote's HEAD to refs/heads/main right after `git init --bare`.

core/ test count: 59 (47 unit + 6 fixtures + 6 write_parity). cargo clippy --all-targets clean.
<!-- SECTION:FINAL_SUMMARY:END -->
