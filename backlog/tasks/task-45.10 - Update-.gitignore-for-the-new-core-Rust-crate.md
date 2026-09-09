---
id: TASK-45.10
title: Update .gitignore for the new core/ Rust crate
status: Done
assignee: []
created_date: '2026-09-09 18:30'
updated_date: '2026-09-09 18:36'
labels: []
dependencies:
  - TASK-45.2
parent_task_id: TASK-45
type: chore
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Per dev-wow's rule that .gitignore is updated whenever a new stack layer is added: add core/target/ (and any other Cargo build artifacts not already covered by the existing src-tauri Rust .gitignore entries) to .gitignore once core/ exists (task-45.2).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 git status shows no core/target/ artifacts as untracked after a cargo build in core/
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
No change needed: .gitignore already has an unanchored `target/` rule (line 78, no leading/mid slash, so it matches at any depth — not the slash-anchoring trap noted elsewhere in dev-system-arkitektur). Verified with `git status --porcelain` after `cargo build`/`cargo test` in core/: no core/target/ files appear as untracked.
<!-- SECTION:FINAL_SUMMARY:END -->
