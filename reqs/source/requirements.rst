Requirements
============

This page is the source of truth for formal requirements (Sphinx-needs).

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

   The CI/CD pipeline must automatically verify that no names or personal data are written in public allocation YAML files.

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

.. req:: Dashboard visualization
   :id: REQ_PUSSLA_007
   :status: open
   :tags: frontend

   The system shall be able to generate a visual heatmap and quarterly forecasts based on the underlying text files.

.. req:: Possible assignment and leads
   :id: REQ_PUSSLA_010
   :status: open
   :tags: structure

   It shall be possible to track future leads and possible assignments for a person in order to track what projects he/she might be going into. These shall be clearly separately identifiable from decided assignments and be possible to filter out.

.. req:: Role and skill based staffing lookup
   :id: REQ_PUSSLA_011
   :status: open
   :tags: staffing, data-model, matching

   The system shall track each person's title/role and skills, including skill keywords used for matching (for example C++, Python, Rust, Zephyr).
   It shall be possible to identify available people for a project based on required title/role and required skills.

.. req:: Dashboard filter for past periods
   :id: REQ_PUSSLA_012
   :status: open
   :tags: dashboard, ux, filtering

   The dashboard shall allow users to hide or show past weeks and past months, so users can focus on the current week and future periods when planning.

.. req:: Dashboard grouped calendar headers
   :id: REQ_PUSSLA_013
   :status: implemented
   :tags: dashboard, ux, frontend

   The dashboard shall show grouped headers with Year at the top, Month (without year) in the middle, and Week at the bottom.
   Month headers shall also show allocation percentage for each month group.

.. req:: Dashboard weekly footer summary
   :id: REQ_PUSSLA_014
   :status: implemented
   :tags: dashboard, frontend, reporting

   The dashboard shall show a footer row with allocation percentage per week column.
   Footer values shall update when visible users/weeks change due to filters.

.. req:: Dashboard week label format
   :id: REQ_PUSSLA_015
   :status: implemented
   :tags: dashboard, frontend, formatting

   The dashboard shall display week labels in `Www` format (for example `W01`) while keeping canonical storage/API keys in `YYYY-Www` format.

.. req:: Dashboard editable weekly planning
   :id: REQ_PUSSLA_016
   :status: implemented
   :tags: dashboard, editing, api, data-write

   The dashboard shall allow editing weekly planning for a selected person-week with support for multiple projects in the same week.
   The dashboard shall support editing planned hours and show derived allocation percentages.
   Saving shall require explicit user action in the GUI (Save), support cancel/discard, and persist changes to allocation YAML files.
   Overbooking (sum of planned hours greater than capacity hours, equivalent to derived percentage greater than 100%) is allowed and shall not block save.


Requirement Status Matrix
=========================

.. needtable:: Requirement status overview
   :types: req
   :columns: id;title;status;tags
   :sort: id


Requirement-Test Gap Analysis
=============================

The tables below assume tests link to requirements via ``:links: REQ_...``.

.. needtable:: Implemented requirements without linked tests
   :types: req
   :columns: id;title;status;links_back
   :sort: id
   :filter: status == "implemented" and len(links_back) == 0

.. needtable:: Tests without linked requirements
   :types: test
   :columns: id;title;status;links
   :sort: id
   :filter: len(links) == 0
