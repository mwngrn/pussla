# Design Choice: Alias-Based People Profiles with Role Catalog and Skill Taxonomy

## Status
**Accepted**

## Context
Staffing requirements need role and skill matching without introducing PII into the planning layer. Existing planning files are organized around alias-based person data, and we need a durable model that can support richer role metadata over time.

## Decision
We standardize staffing data in the planning layer as follows:

- Canonical person profiles are stored as Markdown files with YAML frontmatter in `tst-data/planning/people/<alias>.md`.
- Role definitions are stored as Markdown files with YAML frontmatter in `tst-data/planning/roles/<role_id>.md`.
- Skill taxonomy and synonym mapping are stored in `tst-data/planning/skills.md` as a hybrid format:
  - frontmatter for machine-readable canonical skills and synonym mappings
  - Markdown body for documentation and examples
- Person profiles reference roles via `role_id` and provide `skills` as tags.
- The old `tst-data/planning/allocations/` path is no longer canonical; use `people/`.

## Schema and Validation Rules

### Person profile (`people/<alias>.md`)
- Required frontmatter fields:
  - `alias`
  - `role_id`
  - `skills` (list of strings)

### Role file (`roles/<role_id>.md`)
- Required frontmatter fields:
  - `role_id`
  - `name`
- `role_id` format:
  - `^[A-Za-z0-9]+(?:-[A-Za-z0-9]+)*$`
- `role_id` must be unique across all role files.
- Additional/unknown frontmatter fields:
  - allowed, but linter emits warnings.
- Longer role description is written in Markdown body.

### Skill taxonomy (`skills.md`)
- Canonical skills and synonyms are read from frontmatter.
- Unknown skill terms referenced by people profiles:
  - allowed, but linter emits warnings.
- Matching normalizes skills to lowercase and applies configured synonyms.

## Rationale

### 1. Privacy-safe matching
All staffing matching data remains alias-based in the planning layer, aligned with AI-safe handling.

### 2. Extensible role model
Dedicated role files avoid overloading person profiles and support richer role metadata later.

### 3. Practical skill governance
Central taxonomy with warnings avoids blocking work while still improving data quality.

### 4. Format consistency
Markdown + frontmatter is consistent with existing project and profile conventions.

## Consequences

- Parsers and validators must load `people/`, `roles/`, and `skills.md`.
- Linting gains warning-level checks for unknown role-file fields and unknown skills.
- Existing code paths expecting `planning/allocations/` must be migrated.
