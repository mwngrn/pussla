Project Management Requirements
===============================

This page defines formal requirements for project management and project planning extensions.

.. req:: Capacity is stored in hours per person-week
   :id: REQ_PUSSLA_017
   :status: implemented
   :tags: project-management, data-model, planning

   The system shall store weekly person capacity in hours for each person and week.
   Default weekly capacity shall be 40 hours unless explicitly configured.
   Capacity hours shall be treated as canonical input data.

.. req:: Allocation percent is derived from planned hours
   :id: REQ_PUSSLA_018
   :status: implemented
   :tags: project-management, planning, calculations

   The system shall calculate allocation percentage from planned hours and capacity hours using:
   ``allocation_percent = planned_hours / capacity_hours * 100``.
   The derived percentage shall be available in dashboards and APIs.
   Planned hours and capacity hours shall be treated as canonical data, while percentage shall be a derived value.

.. req:: Representation policy for percent vs hours in UI
   :id: REQ_PUSSLA_040
   :status: implemented
   :tags: project-management, dashboard, frontend, ux

   Utilization-oriented views (for example dashboard/person-centric utilization summaries) shall display allocation as percentages by default.
   Time-entry and project-centric planning views shall display and edit planned work in hours.
   Storage and API write paths shall persist planned hours and capacity hours as canonical values, while percentage remains a derived presentation value.

.. req:: Project planning supports hour-based assignments
   :id: REQ_PUSSLA_019
   :status: implemented
   :tags: project-management, planning, frontend

   It shall be possible to plan project work in hours per person and week.
   The UI shall support creating and editing multiple project assignments in the same week.
   Hour-based planning shall persist as source data in storage and APIs.

.. req:: Project cost calculation from rates and planned hours
   :id: REQ_PUSSLA_020
   :status: implemented
   :tags: project-management, finance, calculations

   The system shall support cost calculation for project plans using planned hours and hourly rates.
   The system shall provide weekly and total cost summaries per project.

.. req:: Configurable person capacity and calendar exceptions
   :id: REQ_PUSSLA_021
   :status: open
   :tags: project-management, capacity, planning

   The system shall support overriding default weekly capacity per person.
   The system shall support capacity exceptions (for example holidays, leave, and part-time weeks).

.. req:: Backward compatibility for existing percentage-based data
   :id: REQ_PUSSLA_022
   :status: implemented
   :tags: project-management, migration, compatibility

   The system shall provide a migration path from existing percentage-based allocation data.
   Existing data shall remain readable during migration, and converted data shall preserve planning intent.

.. req:: Distinguish committed assignments from tentative plans
   :id: REQ_PUSSLA_023
   :status: implemented
   :tags: project-management, workflow, planning

   The system shall support assignment states for committed and tentative project plans.
   Users shall be able to filter tentative assignments out from planning views and reports.

.. req:: Project planning view with grouped time headers
   :id: REQ_PUSSLA_024
   :status: implemented
   :tags: project-management, frontend, timeline

   The system shall provide a project planning view where time is displayed in grouped headers.
   The header model shall align with dashboard grouped header behavior (Year, Month, Week), consistent with ``REQ_PUSSLA_013``.

.. req:: Project view shows person-week planned hours
   :id: REQ_PUSSLA_025
   :status: implemented
   :tags: project-management, frontend, planning

   The project planning view shall show allocated people as rows and weeks as columns.
   Each person-week cell shall display planned hours for that project.
   The view shall support selecting one active project at a time (for example using a dropdown selector).

.. req:: Project view weekly footer sums
   :id: REQ_PUSSLA_026
   :status: implemented
   :tags: project-management, frontend, reporting

   The project planning view shall include a footer row with total planned hours per displayed week column.
   Footer totals shall update when visible rows or periods change due to filters.

.. req:: Project summary panel
   :id: REQ_PUSSLA_027
   :status: implemented
   :tags: project-management, frontend, reporting

   The project planning view shall include a summary panel for the selected project with:
   number of allocated people, start week/date, end week/date, and total planned hours.

.. req:: Date-based milestone management
   :id: REQ_PUSSLA_028
   :status: implemented
   :tags: project-management, milestones, frontend

   The system shall support project milestones stored as date-based entries.
   Users shall be able to add, edit, and remove milestones.
   Milestones shall be visualized in project timeline and overview views.
   Project planning week views shall support milestone creation from week cells and milestone editing via direct interaction with rendered milestone items.

.. req:: Milestone validation and ordering
   :id: REQ_PUSSLA_029
   :status: implemented
   :tags: project-management, milestones, validation

   The system shall validate milestone dates and prevent invalid date values.
   Milestones shall be displayed in chronological order.

.. req:: Project timeline and milestone consistency
   :id: REQ_PUSSLA_030
   :status: implemented
   :tags: project-management, milestones, consistency

   Milestones shown in a project detail view and in timeline overview shall be based on the same underlying milestone data.
   Updates to milestones shall be reflected across both views after save.

.. req:: Project date range derivation and override
   :id: REQ_PUSSLA_031
   :status: implemented
   :tags: project-management, data-model, timeline

   The system shall support project start and end boundaries derived from planned assignments and milestones.
   The system may support explicit manual start and end overrides in project metadata.
   When override values are present, overrides shall take precedence over derived boundaries in project summaries.

.. req:: Project hours-to-cost rollup in view
   :id: REQ_PUSSLA_032
   :status: implemented
   :tags: project-management, finance, frontend

   The project planning view shall provide cost rollups derived from planned hours and configured rates.
   The view shall support at least weekly and total cost summaries.
