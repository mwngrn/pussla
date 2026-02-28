Test Coverage
=============

This page maps unit test cases to requirements using Sphinx-needs ``test`` items.

.. test:: Dashboard week label formatter (Www)
   :id: TEST_PUSSLA_001
   :status: passed
   :tags: dashboard, frontend, formatting, unit
   :links: REQ_PUSSLA_015

   Covered by ``tests/test_week_format.js``.
   Verifies canonical ISO week keys are rendered as ``Www`` labels.

.. test:: Dashboard calendar header grouping and month percentages
   :id: TEST_PUSSLA_002
   :status: passed
   :tags: dashboard, frontend, unit
   :links: REQ_PUSSLA_013, REQ_PUSSLA_014

   Covered by ``tests/test_header_groups.js``.
   Verifies year/month grouping and percentage aggregation logic used by headers/footer.

.. test:: Dashboard editable weekly planning writeback
   :id: TEST_PUSSLA_003
   :status: passed
   :tags: dashboard, editing, api, unit
   :links: REQ_PUSSLA_016

   Covered by ``tests/test_dashboard_editing.py``.
   Verifies multi-project week updates, planned-hours persistence behavior, and payload validation.

.. test:: Validation helpers for planning data format
   :id: TEST_PUSSLA_004
   :status: passed
   :tags: validation, unit
   :links: REQ_PUSSLA_004

   Covered by ``tests/test_validation.py``.
   Verifies validation regex behavior for ISO week format and selected PII detection patterns.

.. test:: Planning aggregation helpers
   :id: TEST_PUSSLA_005
   :status: passed
   :tags: aggregation, unit
   :links: REQ_PUSSLA_007

   Covered by ``tests/test_aggregation.py``.
   Verifies helper behavior used for planning aggregation outputs.

.. test:: Project view grouped header rendering
   :id: TEST_PUSSLA_006
   :status: open
   :tags: project-management, frontend, unit
   :links: REQ_PUSSLA_024

   Planned coverage in ``tests/test_project_view_headers.js``.
   Verifies grouped Year/Month/Week headers in project planning view and consistency with dashboard grouping logic.

.. test:: Project view person-week planned hours cells
   :id: TEST_PUSSLA_007
   :status: open
   :tags: project-management, frontend, unit
   :links: REQ_PUSSLA_025

   Planned coverage in ``tests/test_project_view_hours_grid.js``.
   Verifies person rows, week columns, and hour values rendered per cell for selected project.

.. test:: Project view weekly footer totals
   :id: TEST_PUSSLA_008
   :status: open
   :tags: project-management, frontend, unit
   :links: REQ_PUSSLA_026

   Planned coverage in ``tests/test_project_view_footer_totals.js``.
   Verifies footer totals per week and recomputation when filters change visible rows/periods.

.. test:: Project summary panel aggregates
   :id: TEST_PUSSLA_009
   :status: open
   :tags: project-management, frontend, unit
   :links: REQ_PUSSLA_027

   Planned coverage in ``tests/test_project_summary_panel.js``.
   Verifies project summary metrics: allocated people count, start/end boundaries, and total planned hours.

.. test:: Milestone CRUD and date-based rendering
   :id: TEST_PUSSLA_010
   :status: open
   :tags: project-management, milestones, integration
   :links: REQ_PUSSLA_028

   Planned coverage in ``tests/test_milestone_crud.py``.
   Verifies add/edit/remove flows and timeline rendering for date-based milestones.

.. test:: Milestone date validation and ordering
   :id: TEST_PUSSLA_011
   :status: open
   :tags: project-management, milestones, validation
   :links: REQ_PUSSLA_029

   Planned coverage in ``tests/test_milestone_validation.py``.
   Verifies invalid milestone dates are rejected and valid milestones are presented in chronological order.

.. test:: Milestone consistency across views
   :id: TEST_PUSSLA_012
   :status: open
   :tags: project-management, milestones, integration
   :links: REQ_PUSSLA_030

   Planned coverage in ``tests/test_milestone_view_consistency.py``.
   Verifies milestone updates appear consistently in project detail and timeline overview after save.

.. test:: Project date range derivation and override behavior
   :id: TEST_PUSSLA_013
   :status: open
   :tags: project-management, timeline, unit
   :links: REQ_PUSSLA_031

   Planned coverage in ``tests/test_project_date_range.py``.
   Verifies derived project boundaries from assignments/milestones and explicit metadata override behavior.

.. test:: Hours-to-cost rollup calculations
   :id: TEST_PUSSLA_014
   :status: open
   :tags: project-management, finance, unit
   :links: REQ_PUSSLA_032, REQ_PUSSLA_020

   Planned coverage in ``tests/test_project_cost_rollups.py``.
   Verifies weekly and total cost rollups derived from planned hours and configured rates.

.. test:: Percent-vs-hours representation policy
   :id: TEST_PUSSLA_022
   :status: passed
   :tags: project-management, dashboard, frontend, unit
   :links: REQ_PUSSLA_040, REQ_PUSSLA_018

   Covered by ``tests/test_representation_policy.mjs``.
   Verifies utilization-oriented views present derived percentages, while project-centric and time-entry editing flows use planned hours.

.. test:: Canonical hours write/read compatibility
   :id: TEST_PUSSLA_023
   :status: passed
   :tags: project-management, backend, migration, unit
   :links: REQ_PUSSLA_017, REQ_PUSSLA_018, REQ_PUSSLA_022

   Covered by ``tests/test_dashboard_editing.py`` and ``tests/test_project_management_backend.py``.
   Verifies canonical hours persistence and backward-compatible read behavior for legacy percentage-based entries.

.. test:: Project metadata updates for rate and milestones
   :id: TEST_PUSSLA_024
   :status: passed
   :tags: project-management, milestones, finance, backend, unit
   :links: REQ_PUSSLA_020, REQ_PUSSLA_028, REQ_PUSSLA_029, REQ_PUSSLA_031

   Covered by ``tests/test_project_management_backend.py``.
   Verifies project metadata write path for hourly rate, milestone ordering/validation, and date-range override fields.

.. test:: Assignment state persistence in planning data
   :id: TEST_PUSSLA_025
   :status: passed
   :tags: project-management, workflow, backend, unit
   :links: REQ_PUSSLA_023

   Covered by ``tests/test_project_management_backend.py``.
   Verifies tentative/committed state is retained through read/write flow and exposed in dashboard data.

.. test:: People profile title-role and skill schema
   :id: TEST_PUSSLA_015
   :status: open
   :tags: staffing, data-model, validation
   :links: REQ_PUSSLA_011, REQ_PUSSLA_033

   Planned coverage in ``tests/test_people_skill_profile_schema.py``.
   Verifies title/role and normalized skill terms are parsed and validated for staffing matching.

.. test:: Staffing request period normalization
   :id: TEST_PUSSLA_016
   :status: open
   :tags: staffing, matching, unit
   :links: REQ_PUSSLA_034

   Planned coverage in ``tests/test_staffing_request_period.py``.
   Verifies date-based requests such as ``4 months starting in August`` are normalized to a planning week range.

.. test:: Staffing availability over full requested period
   :id: TEST_PUSSLA_017
   :status: open
   :tags: staffing, matching, validation
   :links: REQ_PUSSLA_035

   Planned coverage in ``tests/test_staffing_availability_window.py``.
   Verifies candidate availability checks include the full requested period and capacity constraints.

.. test:: Candidate ranking and explanation output
   :id: TEST_PUSSLA_018
   :status: open
   :tags: staffing, matching, explainability
   :links: REQ_PUSSLA_036

   Planned coverage in ``tests/test_staffing_candidate_ranking.py``.
   Verifies ranked candidate output and explanation fields for role fit, skill fit, and availability fit.

.. test:: External agent staffing request parse and confirmation gate
   :id: TEST_PUSSLA_019
   :status: open
   :tags: staffing, workflow, integration
   :links: REQ_PUSSLA_037

   Planned coverage in ``tests/test_external_staffing_request.py``.
   Verifies staffing intent is transformed into structured constraints and requires explicit confirmation before booking.

.. test:: Staffing booking assignment states and traceability
   :id: TEST_PUSSLA_020
   :status: open
   :tags: staffing, workflow, integration
   :links: REQ_PUSSLA_038

   Planned coverage in ``tests/test_staffing_booking_flow.py``.
   Verifies booking creates tentative/committed assignments and stores linkage to originating staffing request.

.. test:: Staffing fallback and conflict behavior
   :id: TEST_PUSSLA_021
   :status: open
   :tags: staffing, workflow, validation
   :links: REQ_PUSSLA_039

   Planned coverage in ``tests/test_staffing_conflicts.py``.
   Verifies fallback recommendations when no full match exists and conflict handling behavior during booking.


Test-to-Requirement Matrix
==========================

.. needtable:: Tests and linked requirements
   :types: test
   :columns: id;title;status;links
   :sort: id
