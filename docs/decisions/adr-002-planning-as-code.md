# Design Choice: Markdown with YAML Frontmatter for "Planning-as-Code"

## Status
**Accepted**

## Context
For an organization of approximately 100 people, resource planning requires a balance between **structured data** (for automated capacity calculations) and **qualitative context** (notes on consultant preferences, project nuances, or long-term goals). Existing tools often fail because they decouple the "data" from the "story" behind the allocation.

## Decision
We will utilize **Markdown files with YAML Frontmatter** as the primary data storage format, persisted in a Git repository.

* **Metadata (YAML):** Contains structured fields such as `user_id`, `project_assignments`, `capacity_percentage`, and `timeframes`.
* **Body (Markdown):** Contains free-form text for internal documentation, performance notes, or project-specific context.

---

## Rationale

### 1. Human Ergonomics vs. Machine Readability
While JSON is a standard for data exchange, its syntax is fragile and lacks support for comments. **YAML** provides a clean, indentation-based hierarchy that is easily readable by non-developers while remaining trivial for our parser to convert into a structured database for the GUI.

### 2. Superior Version Control (Git-Diffs)
One of the primary goals is **auditability**. YAML is line-based; when an allocation changes from 50% to 80%, the Git diff is a single-line change. This allows for a **Pull Request workflow** where changes to the company's planning can be peer-reviewed before being "merged" into the official schedule.

### 3. Contextual Enrichment (The Markdown Body)
Planning is rarely just about numbers. By using Markdown, each consultant or project has a "living document." This prevents the "spreadsheet vacuum" where numbers lose their real-world meaning.

### 4. Ecosystem & Tooling
By adopting this format, we tap into a massive ecosystem of existing tools like VS Code, Obsidian, and GitHub’s web interface.

### 5. Reduced Merge Conflicts
By splitting data into individual files (e.g., `people/johndoe.md`), we drastically reduce the likelihood of multiple users editing the same line simultaneously.

---

## Consequences
* **Schema Validation:** Since text files are "untyped," we must implement a **CI Linting step**.
* **Parser Latency:** We introduce a "build step" to aggregate these files into a single state for the GUI.
