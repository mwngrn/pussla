# Way of Working (WoW)

This document defines how work is captured, tracked, and documented in Pussla for both humans and AI agents.

## Source of Truth by Folder

### `reqs/` - Requirements
- Requirements are authored and tracked using Sphinx-needs.
- Requirement IDs and lifecycle status are maintained in requirement files under `reqs/`.
- If behavior is expected from the system, it should be represented as a requirement.

### `backlog/` - Stories and To-Do Work
- Stories, implementation tasks, and ongoing to-do items are tracked in Backlog.md.
- Task creation, status updates, and completion follow the Backlog MCP workflow.
- Backlog tracks delivery commitments and execution state.

### `docs/` - Documentation and Design Decisions
- Architecture and design decisions are documented in `docs/`.
- Technical documentation, rationale, and process details are maintained here.
- Use this folder for durable knowledge that explains why and how things are built.

### `README.md` - Project Entry Point
- `README.md` provides high-level project information.
- It links to deeper documentation in `docs/` and to key working conventions.
- Keep the README concise and navigational, not exhaustive.

## Workflow Summary

1. Capture or update requirements in `reqs/` (Sphinx-needs).
2. Plan and track implementation in `backlog/` (Backlog.md).
3. Record design and documentation updates in `docs/`.
4. Keep `README.md` aligned as the top-level overview and index.

## AI Agent Guidance

- Follow this WoW before creating or updating work artifacts.
- Do not mix concerns:
  - Do not track stories in `reqs/`.
  - Do not store requirements only in backlog tasks.
  - Do not hide design decisions in PR discussion only.
- When in doubt, link to the relevant source-of-truth location instead of duplicating content.
