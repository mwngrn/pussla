---
id: TASK-45.2
title: Scaffold core/ Rust crate with domain models and frontmatter parsing
status: Done
assignee:
  - claude-code/claude-sonnet-5
created_date: '2026-09-09 18:29'
updated_date: '2026-09-09 18:36'
labels: []
dependencies:
  - TASK-45.1
parent_task_id: TASK-45
type: task
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create a new Rust crate at pussla/core/ (Cargo workspace member, or standalone crate referenced by path from src-tauri's Cargo.toml — decide during implementation) holding pure domain logic with no I/O-framework dependency, mirroring devproject/framkalla's core/ crate shape.

Model the existing data as-is (do not change the schema — that's ADR-010's job, already resolved by task-45.1 which this task depends on): Person (alias, role_id, skills, allocations[]), Allocation (project, weeks[] ISO-week strings, planned_hours, capacity_hours, load, state: tentative|committed), Project (project_id, name, owner_alias, start_week/end_week, status, capacity_target, team_aliases[], milestones[] {id,title,date}, activities[] {id,label,start_date,end_date}, hourly_rate, scope/risks body text), Role (role_id, name, body), Identity (alias -> real_name mapping, kept in a separate module/type so it's never in the same struct as planning data).

Parse frontmatter+YAML+Markdown-body files (reference tst-data/planning/people/*.md and tst-data/planning/projects/*.md as fixtures for real shapes) into these models. Use serde + serde_yaml (or gray_matter-equivalent hand-rolled frontmatter splitter) for parsing.

Write cargo unit tests loading the existing tst-data/ fixtures and asserting correct parsing of every field currently in use (including edge cases already present: inline YAML flow-lists like `team_aliases: [MossRiver]`, quoted vs unquoted ISO weeks like "2026-W16" vs 2026-W16).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 core/ crate compiles with cargo check
- [x] #2 Person, Allocation, Project, Role, Identity types exist with serde (de)serialization matching current YAML frontmatter shapes
- [x] #3 cargo test in core/ passes, parsing every file under tst-data/planning/ and tst-data/identity/ without error
- [x] #4 Identity data is never held in the same struct/module as Person/Project/Allocation data (enforced by module boundary, not just convention)
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created core/ (Cargo package `pussla-core`) with:
- core/src/frontmatter.rs — hand-rolled `---`-delimited YAML+body splitter and typed parse() helper (serde_yaml), no external frontmatter crate needed.
- core/src/planning.rs — Person, Allocation, AllocationState. planned_hours/capacity_hours/state are Option<> because tst-data/planning/people/PixelSmith.md is real pre-ADR-005 data with only `load` — a real edge case caught by running tests against every fixture rather than a hand-picked one.
- core/src/project.rs — Project, Milestone, Activity.
- core/src/role.rs — Role.
- core/src/identity.rs — Identity, isolated per ADR-008: no imports from planning/project, and a fixtures.rs test greps identity.rs to keep that boundary enforced going forward, not just documented.
- core/src/lib.rs — module wiring + doc comment stating the ADR-008 boundary.

19 tests total (13 unit + 6 integration in core/tests/fixtures.rs), all passing: every file under tst-data/planning/{people,projects,roles}/ and tst-data/identity/ parses via the matching typed model; skills.md (a different shape, not one of the five models) is checked as generic YAML since it's still under tst-data/planning/. cargo check and cargo clippy are both clean, zero warnings.

.gitignore needed no change — task-45.10's target requirement was already met by the existing unanchored `target/` rule (matches at any depth, confirmed via `git status --porcelain` showing no core/target/ entries), so I closed task-45.10 as well rather than leaving a no-op task open.

Deviation from the task description: planned_hours/capacity_hours/state became Option<f64>/Option<AllocationState> instead of required fields, discovered via the fixture-parsing tests. This is a necessary correction, not a scope change — ADR-005 explicitly anticipates legacy percent-only rows.
<!-- SECTION:FINAL_SUMMARY:END -->
