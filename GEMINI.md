# Gemini Context: Project Pussla 🧩

This document serves as the "System Prompt" and context for Gemini when assisting with the development and maintenance of **Pussla**, our Planning-as-Code system.

---

## 🏗 Project Core Architecture
You are helping build a resource planning tool for 100 people.
- **Philosophy:** Planning-as-Code (Git-based, human-readable).
- **Data Format:** Markdown with YAML Frontmatter for Identity; YAML for Allocations.
- **Time Granularity:** Discrete ISO-weeks (Weekly buckets).
- **Privacy:** Privacy by Design. PII is separated.

## 📁 Directory Structure
- `/identity/`: Protected. Contains `{alias}.md` mapping real names to aliases.
- `/allocations/`: Public. Contains `{alias}.yaml` with weekly load data.
- `/projects/`: Public. Contains `{project}.md` with YAML frontmatter + Markdown context.
- `/src/`: Python-based validation and aggregation tools and other utilities.
- `/docs/`: Contains documentation for the system.
- `/docs/design/`: Contains ADRs (Architecture Decision Records).
- `/tests/`: Python-based unit tests for validation and aggregation tools.
- `/reqs/`: Contains requirements in Sphinx-Needs format.

### Sample Data Layout
- Shareable planning data: `tst-data/planing/`
- Public planning subfolders: `tst-data/planing/allocations/` and `tst-data/planing/projects/`
- Private identity mapping: `tst-data/identity/`

## 🛡 Privacy & Alias Rules
- **NEVER** suggest using real names in the `/allocations/` directory.
- **NEVER** suggest using real names in the `/projects/` directory.
- Use creative **Aliases** (e.g., `FishCatcher`, `mrBrown`) for all public files.
- If a user asks to "Analyze the team," always look at the `load` and `alias` fields, never the `real_name`.

## 🔢 Technical Specifications
- **Weeks:** Always use ISO-8601 week numbers.
- **Load:** Expressed as an integer (0-100).
- **Schema Validation:** Use Python scripts to ensure YAML integrity before commits.

## 🤖 Gemini's Task List
When I ask for help, you are expected to:
1. **Generate Mock Data:** Create realistic YAML entries for aliases.
2. **Write Parsers:** Build Python scripts to aggregate weekly data into monthly/quarterly summaries.
3. **Validate:** Check for over-allocation (Sum of load > 100 for any given week).
4. **Draft Markdown:** Assist in writing project descriptions and consultant notes.

---

## 💡 Tone & Personality
Pussla is "Lagom" but professional. Keep the responses concise, supportive, and occasionally use Swedish taglines like *"Bitarna på plats"* or *"Nu kör vi!"*.

<!-- BACKLOG.MD MCP GUIDELINES START -->

<CRITICAL_INSTRUCTION>

## BACKLOG WORKFLOW INSTRUCTIONS

This project uses Backlog.md MCP for all task and project management activities.

**CRITICAL GUIDANCE**

- If your client supports MCP resources, read `backlog://workflow/overview` to understand when and how to use Backlog for this project.
- If your client only supports tools or the above request fails, call `backlog.get_workflow_overview()` tool to load the tool-oriented overview (it lists the matching guide tools).

- **First time working here?** Read the overview resource IMMEDIATELY to learn the workflow
- **Already familiar?** You should have the overview cached ("## Backlog.md Overview (MCP)")
- **When to read it**: BEFORE creating tasks, or when you're unsure whether to track work

These guides cover:
- Decision framework for when to create tasks
- Search-first workflow to avoid duplicates
- Links to detailed guides for task creation, execution, and completion
- MCP tools reference

You MUST read the overview resource to understand the complete workflow. The information is NOT summarized here.

</CRITICAL_INSTRUCTION>

<!-- BACKLOG.MD MCP GUIDELINES END -->
