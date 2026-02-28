Foundation Requirements
=======================

This page contains cross-cutting requirements that define core architecture,
privacy boundaries, validation expectations, and planning-as-code data handling.

.. req:: Git-based planning
   :id: REQ_PUSSLA_001
   :status: open
   :tags: core, workflow

   The system shall use Git for all resource planning to ensure traceability, version control, and pull-request-based review of changes.

.. req:: Two-layer architecture
   :id: REQ_PUSSLA_002
   :status: open
   :tags: security, architecture

   The system must separate PII into an Identity Layer and anonymized data into an Allocation Layer to follow Privacy by Design.

.. req:: AI-safe allocation
   :id: REQ_PUSSLA_003
   :status: open
   :tags: ai, privacy

   The Allocation Layer may only use anonymous aliases. This enables third-party AI analysis without exposing personal names.

.. req:: Workload validation
   :id: REQ_PUSSLA_004
   :status: open
   :tags: validation

   The system shall use a linter to warn when an individual's total planned hours exceed weekly capacity hours for any time period.
   The system shall also expose derived over-allocation percentage (greater than 100%) in dashboards and reports.
   Over-allocation warnings shall not block saving planning data.

.. req:: PII leak protection
   :id: REQ_PUSSLA_005
   :status: open
   :tags: security, ci-cd

   The CI/CD pipeline must automatically verify that no names or personal data are written in public planning files.

.. req:: Context via Markdown
   :id: REQ_PUSSLA_006
   :status: open
   :tags: ux

   Each resource shall have an associated Markdown file where qualitative information about goals and development can be documented, not only numeric data.

.. req:: Project metadata in frontmatter
   :id: REQ_PUSSLA_008
   :status: open
   :tags: architecture, data-model

   Each project shall have its own Markdown file with YAML frontmatter (for example: owner_alias, start_week, end_week, status) and a free-form Markdown body for scope and risks.

.. req:: Shareable AI folder without identity data
   :id: REQ_PUSSLA_009
   :status: open
   :tags: ai, privacy, structure

   Test and demo data shall be shareable via a single folder, `tst-data/planning/`, containing `allocations/` and `projects/`, while `tst-data/identity/` remains separate.
   Legacy path `tst-data/planing/` may be supported for backward compatibility.
