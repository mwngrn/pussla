---
id: TASK-45
title: Rust core-first architecture for Pussla (framkalla pattern)
status: In Progress
assignee: []
created_date: '2026-09-09 18:29'
updated_date: '2026-09-09 19:09'
labels: []
dependencies: []
references:
  - docs/decisions/adr-007-non-technical-desktop-packaging.md
  - docs/decisions/adr-008-identity-vault-separation.md
  - docs/decisions/adr-009-pm-first-planning-direction.md
type: task
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Rebuild Pussla's backend on the same pattern as devproject/framkalla: a shared Rust `core/` domain crate consumed directly by the Tauri desktop app (no Python subprocess), with an axum `server/` deferred to a later phase. Today the Tauri shell (`src/frontend/src-tauri/src/main.rs`) just spawns `python3 run_dashboard.py` as a subprocess and points the webview at localhost (ADR-007) — the Python backend (`src/dashboard/pussla_engine.py`, `validate_planning_data.py`, `aggregate_planning_data.py`, `dashboard_sync.py`) still owns all domain logic. This parent task tracks the full migration to native Rust, resolved and approved via docs/decisions/adr-010 and adr-011.

Also resolves the long-open docs/decisions/adr-009-pm-first-planning-direction.md (data model direction) and settles docs/decisions/adr-008-identity-vault-separation.md (currently "Proposed") — both must land as accepted ADRs before/alongside the core crate work, since they fix the schema the Rust code will implement.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 ADR-010 (data model resolution, supersedes ADR-009) and ADR-011 (Rust core/desktop-first architecture, supersedes ADR-007's Python clause) exist in docs/decisions/ and are Accepted
- [ ] #2 ADR-008 status updated from Proposed to Accepted (Option B) with the vault-bridge no-persisted-real-names guard rule added
- [ ] #3 core/ Rust crate exists with domain models, frontmatter/YAML parsing, validation, and aggregation logic, covered by cargo test against tst-data/ fixtures
- [ ] #4 src-tauri/main.rs calls core/ directly via #[tauri::command]s and no longer spawns a Python subprocess
- [ ] #5 src/frontend/src/api/dashboard.ts and its module consumers call Tauri invoke() instead of fetch() to a local HTTP server
- [ ] #6 src/dashboard/ Python backend and its setup scripts are removed once parity is verified
- [ ] #7 planning/customers/<slug>.md note type exists with validation, matching the role-catalog pattern from ADR-006
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Progress through task-45.7 (ADRs, core/ crate with domain models/validation/aggregation/write-path/sync all cross-checked against live Python runs, Tauri commands wired, frontend rewired to invoke()) is complete and verified (cargo test/clippy clean, npm run build clean). Remaining: task-45.8 (customer note type) and task-45.9 (Python removal + doc updates, explicitly gated on Mathias's manual `npm run tauri:dev` verification, not just green builds).
<!-- SECTION:NOTES:END -->
