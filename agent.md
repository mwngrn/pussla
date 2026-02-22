# Agent Context: Project Pussla

This document defines the working context for the coding agent assisting with development and maintenance of **Pussla**, a Planning-as-Code system.

## Way of Working (WoW)
- Canonical workflow document: `docs/way-of-working.md`.
- Requirements are added and tracked with Sphinx-needs in `reqs/`.
- Stories, to-do work, and implementation tracking are done in Backlog.md under `backlog/`.
- Documentation and design decisions are written and maintained in `docs/`.
- `README.md` is the high-level overview and points to deeper documentation.

## Project Core Architecture
- Build a resource planning tool for around 100 people.
- Philosophy: Planning-as-Code (Git-based, human-readable).
- Data format: Markdown with YAML frontmatter for identity, YAML for allocations.
- Time granularity: discrete ISO weeks (weekly buckets).
- Privacy model: Privacy by Design with strict PII separation.

## Directory Structure
- `/identity/`: Protected. `{alias}.md` mapping real names to aliases.
- `/allocations/`: Public. `{alias}.yaml` with weekly load data.
- `/projects/`: Public. `{project}.md` with YAML frontmatter and markdown context.
- `/src/`: Python validation, aggregation, and utility tooling.
- `/docs/`: System documentation.
- `/docs/design/`: Architecture Decision Records (ADRs).
- `/tests/`: Python unit tests for validation and aggregation.
- `/reqs/`: Requirements in Sphinx-Needs format.

## Sample Data Layout
- Shareable planning data: `tst-data/planing/`
- Public planning data: `tst-data/planing/allocations/` and `tst-data/planing/projects/`
- Private identity mapping: `tst-data/identity/`

## Privacy and Alias Rules
- Never use real names in `/allocations/`.
- Never use real names in `/projects/`.
- Use aliases for all public files (for example: `FishCatcher`, `mrBrown`).
- If asked to analyze the team, use `load` and `alias` fields only, not `real_name`.

## Technical Specifications
- Use ISO-8601 week numbers.
- `load` must be an integer from 0 to 100.
- Run Python-based schema validation for YAML integrity before commit.

## Agent Task Expectations
1. Generate realistic mock YAML data for aliases.
2. Generate project Markdown files with YAML frontmatter and contextual notes.
3. Write Python parsers that aggregate weekly data into monthly and quarterly summaries.
4. Validate and flag over-allocation (sum of load > 100 for any week).
5. Draft Markdown project descriptions and consultant notes.

## Communication Style
- Keep responses concise and professional.
- Maintain a pragmatic, supportive style.
