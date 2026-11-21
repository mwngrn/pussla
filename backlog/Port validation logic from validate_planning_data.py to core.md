---
id: TASK-45.3
aliases:
- TASK-45.3
priority: 48
---

# Port validation logic from validate_planning_data.py to core/
#story/done Port all validation rules currently implemented in src/dashboard/../validate_planning_data.py (repo root src/validate_planning_data.py) into the core/ Rust crate: schema validation (required…

Requires < [[Scaffold core Rust crate with domain models and frontmatter parsing]]

Parent: [[Rust core-first architecture for Pussla (framkalla pattern)]]

## Description

Port all validation rules currently implemented in src/dashboard/../validate_planning_data.py (repo root src/validate_planning_data.py) into the core/ Rust crate: schema validation (required frontmatter fields per ADR-006 for people/roles/skills), over-allocation checks (sum of load > 100 for any person-week per ADR-005's derived-percent rule), role_id format/uniqueness, alias uniqueness, and any other checks currently run by CI (see .github/workflows/ for how validation is invoked today).

Write equivalent Rust unit tests for each rule, using tst-data/ as fixtures plus new fixtures for the failure cases (e.g. an intentionally over-allocated person, an invalid role_id).

## Acceptance Criteria
- ✓ Every validation rule in src/validate_planning_data.py has an equivalent Rust implementation in core/
- ✓ Each rule has at least one passing-case and one failing-case unit test
- ✓ cargo test passes for the full validation module

## Final Summary

Ported src/validate_planning_data.py to core/src/validation.rs (plus a shared core/src/iso_week.rs for ISO-week parsing/regex, reused by dashboard.rs too, and core/src/rounding.rs for Python-compatible round-half-to-even, needed for exact parity — see task-45.4's summary for how that surfaced).

Ported: validate_roles (schema, role_id regex/uniqueness, unknown-field warnings), validate_skills_catalog (canonical_skills/synonyms), validate_people (schema, skills/canonical-skill warnings, allocation shape, planned_hours vs load consistency check, over-allocation with fail/warn toggle), validate_projects (schema, ISO week fields, non-empty body), check_pii_leaks (real_name substring + email/phone regex), and validate_dataset() tying them together (roles -> skills -> people -> projects -> cross-references -> PII scan), matching validate_planning_data.py's main() sequencing.

Cross-checked against the actual Python script, not just re-derived logic: ran `python3 src/validate_planning_data.py --planning-dir tst-data/planning --identity-dir tst-data/identity` and got "Validation failed with 16 error(s) and 0 warning(s)" — tst-data/ genuinely has over-allocated people (by design, feeds the dashboard's overbooked_slots metric) and a few markdown bodies that trip the phone-number regex false positive (same class of false positive already known from identity-shield's own PII scanner). The Rust port reproduces exactly 16 errors against the same data (test: matches_python_reference_error_count).

11 unit tests total for this module (role/skills/people/project validation, PII leak detection, legacy load-only rows, invalid ISO weeks), all passing.

Not ported: the CLI argument parsing / stdout WARNING:/ERROR: printing from validate_planning_data.py's main() — core/ exposes composable Result-bearing functions instead; a CLI or Tauri command wrapping them is a separate concern (task-45.6).
