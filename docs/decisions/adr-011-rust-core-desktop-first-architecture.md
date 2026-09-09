# Design Choice: Rust Core, Desktop-First Architecture

## Status
**Accepted** — supersedes the "keep the existing Python backend logic" and "does not require rewriting the backend away from Python immediately" clauses of [ADR-007](adr-007-non-technical-desktop-packaging.md).

## Context
ADR-007 packaged Pussla as a Tauri desktop shell wrapped around the existing Python backend and React frontend, explicitly as a near-term, minimal-rewrite step: Tauri's `main.rs` spawns `python3 run_dashboard.py` as a subprocess, reserves a local port, waits for the backend to come up, and then redirects the webview to `http://127.0.0.1:<port>`. All domain logic (parsing, validation, aggregation, sync) still lives in `src/dashboard/pussla_engine.py`, `validate_planning_data.py`, `aggregate_planning_data.py`, and `dashboard_sync.py`.

This is a shell around a separate process, not a native application — different from the pattern already proven in `devproject/framkalla` (Tauri 2 + React desktop, `core/` shared Rust domain crate, `server/` axum HTTP API, all in one repo), where the desktop app calls domain logic in-process with no subprocess or language boundary. Mathias asked for Pussla to move to the same shape: Rust `core/` first, Tauri desktop consuming it directly next, and an axum `server/` (web-UI, non-Tauri access) deferred to a later phase — desktop first, server later, but the core crate built from the start so the server phase is additive, not a second rewrite.

## Decision
Adopt the `framkalla` shape for Pussla:

1. **`core/`** — a new Rust crate holding all domain logic with no I/O-framework dependency: models (per [ADR-010](adr-010-pm-first-data-model-resolution.md)), frontmatter/YAML parsing, validation (replacing `validate_planning_data.py`), aggregation (replacing `aggregate_planning_data.py` and the aggregation parts of `pussla_engine.py`), and git-backed sync (replacing `dashboard_sync.py`, shelling out to the `git` CLI rather than a heavy git library). Consumed directly by the desktop app now, and by `server/` once that phase starts.
2. **Desktop first.** `src/frontend/src-tauri/src/main.rs` stops spawning Python. It calls `core/` in-process via `#[tauri::command]` functions, one per current `dashboard.ts` API call. The existing data-folder selection/first-run UX from ADR-007 (external user-selected data folder, remembered in app settings) is unchanged — only the backend execution mechanism changes.
3. **Frontend** (`src/frontend/src/api/dashboard.ts` and its module consumers) swaps `fetch()` against localhost for `@tauri-apps/api/core`'s `invoke()`. The existing TypeScript interfaces (`DashboardData`, `ProjectContext`, `SyncStatus`, etc.) are kept as the contract `core/` serializes against, so the UI layer doesn't need a parallel rewrite.
4. **Server is deferred, not skipped.** An axum `server/` crate (matching framkalla's `/api/v1` shape) is planned but not built in this phase. Because `core/` is written framework-agnostic from the start, adding `server/` later is additive.
5. **Python is deleted, not kept as a fallback**, once `core/` reaches feature parity and its tests pass (see task-45.9). This matches `dev-wow`'s "no compatibility shims once the replacement works" convention — ADR-007's "does not require rewriting the backend away from Python immediately" no longer holds now that the rewrite is the explicit plan.

## Rationale

### 1. Removes a whole failure class
Subprocess spawning, port reservation, and "wait for backend to come up" polling (`main.rs`'s `spawn_backend`/`reserve_port`/`wait_for_backend`) are all failure modes that don't exist once the desktop app calls `core/` in-process.

### 2. One pattern across Mathias's Tauri/Rust projects
`framkalla` already proved this shape works for a desktop-first personal tool that may later grow a server. Reusing it here means less to re-derive next time, and any core-crate lessons from framkalla (e.g. its auth pattern, later reused by `lasa`) are available as design reference for Pussla's own `server/` phase when it starts.

### 3. Desktop-first sequencing matches actual priority
Mathias explicitly wants the desktop app working well before investing in a server — building `core/` framework-agnostic from day one means that sequencing doesn't cost a second migration later.

### 4. No fallback avoids a permanent dual-maintenance burden
Keeping Python "just in case" would mean two implementations of the same domain logic drifting apart. Once `core/` has equivalent test coverage, Python is removed outright (task-45.9), gated on Mathias's manual verification, not kept indefinitely.

## Consequences
- `core/` becomes a new Cargo crate in the repo, with its own `cargo test` suite (per `dev-wow`'s testing division: the agent owns deterministic/isolable logic — parsing, validation, aggregation).
- `src-tauri/Cargo.toml` gains a path dependency on `core/`.
- `.gitignore` gains `core/target/` (task-45.10).
- `src/dashboard/`, `requirements.txt`, and `setup-python-venv.sh` are removed once parity is verified (task-45.9) — README.md, `agent.md`, and `AGENTS.md` are updated in the same step.
- `server/` (axum) remains a named, planned, but not-yet-scheduled phase — revisit once the desktop app is stable.

## Non-Goals
- This ADR does not build `server/` now.
- This ADR does not change the on-disk data format — that is ADR-010's scope.
- This ADR does not change the ADR-007 data-folder/first-run UX.

## References
- [ADR-007 — Non-technical desktop packaging (superseded in part by this ADR)](adr-007-non-technical-desktop-packaging.md)
- [ADR-010 — PM-first data model resolution](adr-010-pm-first-data-model-resolution.md)
- `devproject/framkalla` (vault) — the `core/` + Tauri + axum shape this ADR adopts
