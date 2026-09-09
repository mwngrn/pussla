---
id: TASK-45.1
title: 'Write ADR-010 and ADR-011, update ADR-008 status'
status: Done
assignee:
  - claude-code/claude-sonnet-5
created_date: '2026-09-09 18:29'
updated_date: '2026-09-09 18:32'
labels: []
dependencies: []
parent_task_id: TASK-45
type: docs
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Write docs/decisions/adr-010-pm-first-data-model-resolution.md, explicitly resolving/superseding adr-009-pm-first-planning-direction.md's open questions: (1) assignment storage stays person-centric (planning/people/<alias>.md), project notes become the PM-first artifact; (2) project span = ISO dates, allocation rate = ISO weekly buckets, decoupled as today; (3) frontmatter vs body split rule (structured/typed fields in frontmatter, narrative in body) documented explicitly, borrowed from mtd-obs/obs-teamboard conventions but keeping typed frontmatter arrays (e.g. team_aliases) since Pussla isn't Obsidian-rendered; (4) customer becomes its own note type planning/customers/<slug>.md, same pattern as the ADR-006 role catalog; (5) if/when comments are needed on project notes, reuse obs-teamboard's flat chronological '## Comments' bullet list with mention+date attribution; (6) discovery stays scoped to the configured planning root, not a vault-wide tag scan.

Write docs/decisions/adr-011-rust-core-desktop-first-architecture.md, superseding ADR-007's "keep the existing Python backend logic" clause: a new core/ Rust crate holds all domain logic (models, frontmatter/YAML parsing, validation, aggregation), consumed directly by the Tauri desktop app now via #[tauri::command]s (no subprocess), with an axum server/ deferred to a later phase (not built yet). Python (src/dashboard/, requirements.txt, setup-python-venv.sh) is removed once the Rust core reaches parity, not kept as a fallback.

Update docs/decisions/adr-008-identity-vault-separation.md status from "Proposed" to "Accepted (Option B)" and add a Consequences bullet: any local cache/settings file the app writes (last-opened folder, resolved-name display cache, sync state) may only ever contain aliases, never resolved real names — resolution happens in-memory per session. This generalizes the concrete leak vault-bridge shipped and fixed in its own ADR-001 (aliases leaking into a persisted, mobile-synced externalIndex).

Follow the ADR format already used in this repo's docs/decisions/ (Status/Context/Decision/Rationale/Consequences headings, see adr-008 or adr-009 for reference).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 adr-010-pm-first-data-model-resolution.md exists, Status: Accepted, explicitly states it supersedes adr-009
- [x] #2 adr-011-rust-core-desktop-first-architecture.md exists, Status: Accepted, explicitly states it supersedes adr-007's Python-backend clause
- [x] #3 adr-008's Status line reads Accepted (Option B) and its Consequences section includes the no-persisted-real-names guard rule
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Write adr-010 and adr-011 following this repo's existing ADR format (Status/Context/Decision/Rationale/Consequences), modeled on adr-008.md/adr-009.md. Content is already fully specified in the task description (resolved during sysarch planning with Mathias) — no further research needed, this is a docs-writing task. Steps: 1) write adr-010-pm-first-data-model-resolution.md, 2) write adr-011-rust-core-desktop-first-architecture.md, 3) edit adr-008's Status line and Consequences section in place.
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Wrote docs/decisions/adr-010-pm-first-data-model-resolution.md (Accepted, explicitly supersedes adr-009) resolving all of adr-009's open questions: assignments stay person-centric in planning/people/, project span stays dates + allocation rate stays ISO weeks (unchanged, now explicit), frontmatter-vs-body split rule documented (structured fields in frontmatter, teamboard's body-mention convention explicitly NOT imported since Pussla isn't Obsidian-rendered), customer gets its own planning/customers/<slug>.md note type matching the ADR-006 role-catalog pattern, comments (if ever needed) reuse obs-teamboard's flat '## Comments' format, discovery stays scoped to the planning root rather than a vault-wide tag scan.

Wrote docs/decisions/adr-011-rust-core-desktop-first-architecture.md (Accepted, explicitly supersedes ADR-007's Python-backend clauses), specifying the core/ crate + Tauri-direct-call architecture, desktop-first/server-deferred sequencing, and Python removal once parity is verified.

Updated docs/decisions/adr-008-identity-vault-separation.md: Status changed from "Proposed — Option B, awaiting decision" to "Accepted (Option B)"; added a new Consequences bullet ("No-persisted-real-names guard rule") generalizing the vault-bridge externalIndex leak (its own ADR-001) into a hard rule for any Pussla local cache/settings file.

No code changed — this was a docs-only task, verified by reading back the Status lines of all three files.
<!-- SECTION:FINAL_SUMMARY:END -->
