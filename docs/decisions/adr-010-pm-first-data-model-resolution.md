# Design Choice: PM-First Data Model Resolution

## Status
**Accepted** — supersedes [ADR-009](adr-009-pm-first-planning-direction.md).

## Context
[ADR-009](adr-009-pm-first-planning-direction.md) proposed reframing Pussla around project management (project/customer/phases/milestones as the primary artifact, allocation as a derived view) and left five open questions unresolved: assignment storage, allocation persistence format, PII surface, customer modeling, and migration.

Looking at the data actually in use (`tst-data/planning/people/*.md`, `tst-data/planning/projects/*.md`), Pussla is already close to what ADR-009 called the "Hybrid" option. This ADR resolves the open questions by formalizing what is already there rather than proposing a big-bang rewrite, and pulls in conventions already proven in the related Obsidian tooling (`mtd-obs`, `obs-teamboard`, `identity-shield`, `vault-bridge`) so Pussla's data model stays consistent with the rest of that ecosystem rather than inventing a third variant.

## Decision

### 1. Assignment storage stays person-centric
Assignments (person ↔ project ↔ span ↔ effort) continue to live in `planning/people/<alias>.md` exactly as today (ADR-001's merge-isolation rationale still applies). Project notes become the PM-first artifact — customer, span, phases/milestones — but do not become the place assignments are authored. At Pussla's actual scale (personal use plus small collaboration), the project-file merge-conflict risk that motivated ADR-009's alternative option does not apply; person-centric files keep the write-isolation win from ADR-001 essentially for free.

### 2. Two time concepts, unchanged
Project *span* is authored as ISO dates (`activities[].start_date`/`end_date`, `milestones[].date` — already the case). Allocation *rate* is authored as ISO weekly buckets (ADR-004, already the case: `allocations[].weeks`). These remain decoupled via the aggregation build step (ADR-002/ADR-005), exactly as ADR-009 proposed. No schema change — this section only makes the rule explicit in `docs/architecture.md` instead of leaving it implicit.

### 3. Frontmatter vs. body split
Adopt the same underlying rule already used by `mtd-obs` and `obs-teamboard`: structured, typed, machine-validated fields go in frontmatter; narrative and context go in the Markdown body. Pussla already follows this (`project_id`, `status`, dates, `milestones`, `activities`, `hourly_rate` in frontmatter; `## Scope`/`## Risks` in body). Pussla is not Obsidian-rendered, so `obs-teamboard`'s `@[[wikilink]]`-mention-in-body convention for assignees is **not** adopted — a typed `team_aliases` array in frontmatter remains the better fit for a Rust-validated schema, since Pussla has no click-to-edit note UI to make body-mentions ergonomic. The rule is: reuse the *underlying* principle, not the Obsidian-specific mechanics that only pay off inside an editor.

### 4. Customer becomes its own note type
`planning/customers/<slug>.md`, following the same shape as the role catalog (ADR-006, `planning/roles/<role_id>.md`): frontmatter for machine-readable fields (`customer_id`, `name`, contacts), Markdown body for freeform notes. This follows the established pattern of generalizing shared infrastructure immediately rather than leaving customer as a bare frontmatter string on the project that would need retrofitting later.

### 5. Comments
If/when project notes need a discussion trail, reuse `obs-teamboard`'s format verbatim: a flat chronological `## Comments` section, one bullet per comment, each attributed with a mention and a date. Not implemented now — recorded here so a future implementer doesn't invent a fourth comment shape across Mathias's tools.

### 6. Discovery stays scoped
`mtd-obs`'s vault-wide `#project`-tag scanning is an Obsidian-specific convergence idea, not adopted here. Pussla's data root is already physically scoped (`planning/` vs `identity/` — see ADR-008), so project/person/customer discovery stays "everything under the configured planning root," never a vault-wide tag scan. This preserves the identity-shield pattern's "protection by absence of a path" property (ADR-008) — a vault-wide scan would risk walking into identity-vault territory if the two were ever colocated.

## Rationale
1. **Minimal migration cost.** Points 1–3 mostly describe the current data model; this ADR mostly documents and confirms rather than changes.
2. **Consistency across Mathias's tools.** Points 3 and 5 deliberately reuse conventions already validated in `mtd-obs`/`obs-teamboard` rather than inventing parallel ones, while point 3 explicitly declines to import a convention (body-mentions) that only makes sense inside an Obsidian editor.
3. **PII boundary stays simple.** Point 6 keeps Pussla's discovery model aligned with ADR-008's identity-vault separation instead of diffusing it vault-wide.

## Consequences
- `docs/architecture.md` gains an explicit statement of the span-vs-rate rule and the frontmatter-vs-body rule (previously implicit).
- A new `planning/customers/` folder and `Customer` model are added to the schema (`core/`, see ADR-011) and to `reqs/`.
- ADR-009 is superseded: its "Hybrid" alternative is the one actually adopted, and none of its open questions remain open.
- No migration of existing `tst-data/` is required for points 1–3; only point 4 (customers) adds new data.

## References
- [ADR-001 — Person-centric vs project-centric storage](adr-001-person-centric-vs-project-centric.md)
- [ADR-004 — Time granularity](adr-004-time-granularity.md)
- [ADR-006 — Staffing, role and skill catalog](adr-006-staffing-role-skill-catalog.md)
- [ADR-008 — Identity-vault separation](adr-008-identity-vault-separation.md)
- [ADR-009 — PM-first planning direction (superseded by this ADR)](adr-009-pm-first-planning-direction.md)
- `mtd-obs`/`obs-teamboard` (vault) — frontmatter/body split, comment format conventions
