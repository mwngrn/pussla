# Planning Direction: Project-Management-First (allocation derived)

## Status
**Proposed** — captures a direction shift for later review, not yet decided. If accepted, supersedes ADR-001 and ADR-004 and amends ADR-005/ADR-006.

## Context
Pussla started **beläggning-first**: the primary artifact was a person-week utilization heatmap (a replacement for the momang/Excel capacity panic). That framing drove the current data model:
- person-centric storage (ADR-001),
- discrete weekly buckets (ADR-004),
- hours-first per person-week (ADR-005),
- alias-based `planning/people|roles` layout (ADR-006).

The intended direction is now **project-management-first**: you plan *projects* (customer, timeline, phases, milestones) and **allocation/utilization becomes a derived view** of assignments — not the primary thing you author.

Main driver: **compatibility with mtd-obs**, so a project is *one shared markdown artifact*. mtd-obs is markdown-first (mtd-obs ADR-001) and renders a single project inline (milestone-chart, mtd-obs ADR-005). If a pussla project *is* an mtd project note, we get one file, two tools — and we avoid two rendering engines with duplicated logic.

## Decision (proposed)
Reframe pussla around project management; beläggning becomes a derived output.

1. **Project & customer are first-class.** A project is an mtd-compatible markdown note that can live *anywhere* in the vault and is identified by a `#project` tag, with frontmatter for `customer`, `start`/`end` (or `duration`), and phases/milestones. Hierarchy: Customer → Project → phases/milestones → assignments.
2. **Assignments yield allocation.** The unit you author is an assignment (person ↔ project over a span, with an effort). Allocation %/hours per week is **derived by the engine**, not authored directly.
3. **Two time concepts, two formats.** Project *span* = dates (mtd-native). Allocation *rate* = normalized to weekly buckets internally, preserving ADR-004's compute/rollup/heatmap benefits. **Author in ranges/durations, compute in weeks** — decoupled via the existing aggregation build step (ADR-002).
4. **Shared project-format spec** across pussla + mtd-obs: mtd ignores pussla-only allocation data; pussla ignores mtd task/render specifics. Without a written spec, "compatible" breaks silently.

## Alternatives considered
- **A — Stay beläggning-first (current ADRs).** Best if the primary user is a self-updating consultant and utilization is the main artifact; keeps person-centric merge isolation. Rejected as the *default* because it blocks mtd convergence.
- **B — PM-first, allocation derived (recommended).** Best if the primary user is a PM/delivery lead; enables the one-project-two-tools convergence.
- **Hybrid — person-centric storage, project-first UX.** Decouple storage from mental model: keep person files (ADR-001/006) but drive the experience from projects. Possible; keeps merge isolation but weakens mtd-compat and still needs the shared format.

## What this supersedes / amends
- **ADR-001 (person-centric):** storage may shift to project/assignment-centric. Trade-off: hot-project files reintroduce write-side merge conflicts *at scale* (100+ people editing weekly). At current scale (personal + possible Acorn collab) this is negligible. Read cost is absorbed by the ADR-002 aggregation build regardless of shape.
- **ADR-004 (weekly buckets):** kept for allocation *rate*; add dates for project *span*. Not a full reversal — a layering.
- **ADR-005 (hours-first):** hours remain the canonical effort unit, now attached to assignments within a project rather than person-week being primary.
- **ADR-006 (roles/skills, `planning/` layout):** the roles/skills catalog stays valid, but "projects anywhere" breaks the fixed `planning/` layout assumption — discovery becomes tag-based (`#project`) scanning.

## Open questions (resolve on review day)
1. **Assignment storage:** on the project note, on the person note, or as its own atomic unit? (Determines which side gets git-write isolation.)
2. **Allocation persistence:** store as date ranges (ergonomic, messy one-week diffs) or as weeks (clean diff, ADR-004)? Recommendation leans: project span = dates, allocation = weeks.
3. **PII surface:** "projects anywhere with `#project`" diffuses the planning layer from a tidy `planning/` folder into scattered notes, so aliases appear vault-wide. How does pussla discover planning notes while preserving the identity-shield separation (aliases only, identity vault out of path)? See ADR-008 and the "identity-shield pattern".
4. **Customer:** its own note type, or just a frontmatter field on projects?
5. **Migration:** from existing person-week percentage/hours data to the assignment model.

## Consequences
- This is a **data-model v2**, not a tweak — do it via explicit superseding ADRs, not silently.
- Requires the shared project-format spec (item 4 of the decision) *before* implementation.
- The identity/PII question (open question 3) must be settled alongside, not after.

## References
- pussla: ADR-001, ADR-002, ADR-004, ADR-005, ADR-006, ADR-008
- mtd-obs: ADR-001 (markdown-first), ADR-004 (tasks vs planning objects), ADR-005 (milestone-chart)
- "identity-shield pattern" (vault) — PII separation this must not break
