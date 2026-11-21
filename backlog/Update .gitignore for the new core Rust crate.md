---
id: TASK-45.10
aliases:
- TASK-45.10
priority: 55
---

# Update .gitignore for the new core/ Rust crate
#story/done Per dev-wow's rule that .gitignore is updated whenever a new stack layer is added: add core/target/ (and any other Cargo build artifacts not already covered by the existing src-tauri Rust…

Requires < [[Scaffold core Rust crate with domain models and frontmatter parsing]]

Parent: [[Rust core-first architecture for Pussla (framkalla pattern)]]

## Description

Per dev-wow's rule that .gitignore is updated whenever a new stack layer is added: add core/target/ (and any other Cargo build artifacts not already covered by the existing src-tauri Rust .gitignore entries) to .gitignore once core/ exists (task-45.2).

## Acceptance Criteria
- ✓ git status shows no core/target/ artifacts as untracked after a cargo build in core/

## Final Summary

No change needed: .gitignore already has an unanchored `target/` rule (line 78, no leading/mid slash, so it matches at any depth — not the slash-anchoring trap noted elsewhere in dev-system-arkitektur). Verified with `git status --porcelain` after `cargo build`/`cargo test` in core/: no core/target/ files appear as untracked.
