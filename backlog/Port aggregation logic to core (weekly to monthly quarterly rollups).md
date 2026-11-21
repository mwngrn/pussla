---
id: TASK-45.4
aliases:
- TASK-45.4
priority: 49
---

# Port aggregation logic to core/ (weekly to monthly/quarterly rollups)
#story/done Port aggregation logic from src/aggregate_planning_data.py and the relevant parts of src/dashboard/pussla_engine.py (dashboard data assembly: DashboardData/DashboardUser/WeeklyStats/ProjectSlot…

Requires < [[Scaffold core Rust crate with domain models and frontmatter parsing]]

Parent: [[Rust core-first architecture for Pussla (framkalla pattern)]]

## Description

Port aggregation logic from src/aggregate_planning_data.py and the relevant parts of src/dashboard/pussla_engine.py (dashboard data assembly: DashboardData/DashboardUser/WeeklyStats/ProjectSlot shapes, see src/frontend/src/api/dashboard.ts for the exact TypeScript types the frontend expects back) into core/. This includes: weekly allocation rollups per person, monthly/quarterly aggregation (per ADR-004's month-attribution rule), derived allocation_percent from planned_hours/capacity_hours (ADR-005), and project context resolution (start/end week derivation, milestones, activities) as currently exposed via ProjectContext in dashboard.ts.

Serialize core's output to JSON matching the existing DashboardData shape in dashboard.ts exactly, so the frontend types don't need to change in this task (that's task-45.7's job).

Write cargo unit tests covering weekly->monthly->quarterly rollup correctness and the percent-derivation formula, using tst-data/ as fixtures.

## Acceptance Criteria
- ✓ core/ produces a DashboardData-shaped JSON output matching src/frontend/src/api/dashboard.ts's TypeScript interfaces field-for-field
- ✓ Weekly, monthly, and quarterly rollups have passing cargo tests against tst-data/ fixtures
- ✓ allocation_percent = planned_hours / capacity_hours * 100 is verified by a unit test

## Final Summary

Ported the dashboard read path: core/src/dashboard.rs's build_dashboard_data() reproduces pussla_engine.py's build_dashboard_data() byte-for-byte, verified against a real Python-generated golden file (core/tests/fixtures/dashboard_reference.json, produced by actually running `python3 src/dashboard/pussla_engine.py` against tst-data/), not just re-derived logic. The dashboard::tests::matches_python_reference_output test does a full serde_json::Value equality check (minus generated_at) against that golden file.

Getting to byte-for-byte parity surfaced a real Python-vs-Rust semantic gap, not a typo: Python 3's round() is round-half-to-even ("banker's rounding"), Rust's f64::round() is round-half-away-from-zero — they disagree exactly on ties (round(62.5): Python 62, Rust 63), and this dataset has a real tie (ArcticPine's 2026-W13 load, 25h/40h = exactly 62.5%). Added core/src/rounding.rs (python_round/round1) and switched every rounding call in dashboard.rs and validation.rs to it — this is now a shared, tested piece of infrastructure, not a one-off fix.

Also added (beyond the original task description, but implied by ADR-004's "month rollups" language, which was previously dead code — Python's get_month_from_iso_week was defined but never called by anything):
- core/src/iso_week.rs: month_from_week/quarter_from_week, built on chrono's ISO-week-date construction (matches Python's `strptime(w,"%G-W%V-%u")` exactly, including the real edge case that ISO week 1 can fall in the *previous* December — verified against Python's datetime for several weeks).
- core/src/aggregate.rs: weekly_load_totals (port of aggregate_planning_data.py's aggregate_data, cross-checked against an actual run of that script) plus monthly_load_totals/quarterly_load_totals built on top — this is genuinely new capability, not parity work, since nothing in the current codebase does monthly/quarterly rollups today.

15 new tests across dashboard.rs/iso_week.rs/aggregate.rs, all passing; 47 tests total in core/ now. cargo clippy --all-targets is clean.

Not ported: aggregate_planning_data.py's generate_summary() (the terminal markdown-table pretty-printer) and pussla_engine.py's update_week_allocations/update_project_metadata (the write-path mutators, ~350 lines) — those are CLI/write-path concerns, not part of the DashboardData JSON contract this task covers. Flagging for task-45.6: the Tauri command for save_allocation/save_project will need either a port of those or a simpler from-scratch implementation — recommend scoping that explicitly when task-45.6 starts rather than assuming full parity is already available.
