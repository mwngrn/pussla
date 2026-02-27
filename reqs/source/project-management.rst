Project Management Requirements
===============================

This page defines formal requirements for project management and project planning extensions.

.. req:: Capacity is stored in hours per person-week
   :id: REQ_PUSSLA_017
   :status: open
   :tags: project-management, data-model, planning

   The system shall store weekly person capacity in hours for each person and week.
   Default weekly capacity shall be 40 hours unless explicitly configured.
   Capacity hours shall be treated as canonical input data.

.. req:: Allocation percent is derived from planned hours
   :id: REQ_PUSSLA_018
   :status: open
   :tags: project-management, planning, calculations

   The system shall calculate allocation percentage from planned hours and capacity hours using:
   ``allocation_percent = planned_hours / capacity_hours * 100``.
   The derived percentage shall be available in dashboards and APIs.
   Planned hours and capacity hours shall be treated as canonical data, while percentage shall be a derived value.

.. req:: Project planning supports hour-based assignments
   :id: REQ_PUSSLA_019
   :status: open
   :tags: project-management, planning, frontend

   It shall be possible to plan project work in hours per person and week.
   The UI shall support creating and editing multiple project assignments in the same week.
   Hour-based planning shall persist as source data in storage and APIs.

.. req:: Project cost calculation from rates and planned hours
   :id: REQ_PUSSLA_020
   :status: open
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
   :status: open
   :tags: project-management, migration, compatibility

   The system shall provide a migration path from existing percentage-based allocation data.
   Existing data shall remain readable during migration, and converted data shall preserve planning intent.

.. req:: Distinguish committed assignments from tentative plans
   :id: REQ_PUSSLA_023
   :status: open
   :tags: project-management, workflow, planning

   The system shall support assignment states for committed and tentative project plans.
   Users shall be able to filter tentative assignments out from planning views and reports.

.. req:: Project planning view with grouped time headers
   :id: REQ_PUSSLA_024
   :status: open
   :tags: project-management, frontend, timeline

   The system shall provide a project planning view where time is displayed in grouped headers.
   The header model shall align with dashboard grouped header behavior (Year, Month, Week), consistent with ``REQ_PUSSLA_013``.

.. req:: Project view shows person-week planned hours
   :id: REQ_PUSSLA_025
   :status: open
   :tags: project-management, frontend, planning

   The project planning view shall show allocated people as rows and weeks as columns.
   Each person-week cell shall display planned hours for that project.

.. req:: Project view weekly footer sums
   :id: REQ_PUSSLA_026
   :status: open
   :tags: project-management, frontend, reporting

   The project planning view shall include a footer row with total planned hours per displayed week column.
   Footer totals shall update when visible rows or periods change due to filters.

.. req:: Project summary panel
   :id: REQ_PUSSLA_027
   :status: open
   :tags: project-management, frontend, reporting

   The project planning view shall include a summary panel for the selected project with:
   number of allocated people, start week/date, end week/date, and total planned hours.

.. req:: Date-based milestone management
   :id: REQ_PUSSLA_028
   :status: open
   :tags: project-management, milestones, frontend

   The system shall support project milestones stored as date-based entries.
   Users shall be able to add, edit, and remove milestones.
   Milestones shall be visualized in project timeline and overview views.

.. req:: Milestone validation and ordering
   :id: REQ_PUSSLA_029
   :status: open
   :tags: project-management, milestones, validation

   The system shall validate milestone dates and prevent invalid date values.
   Milestones shall be displayed in chronological order.

.. req:: Project timeline and milestone consistency
   :id: REQ_PUSSLA_030
   :status: open
   :tags: project-management, milestones, consistency

   Milestones shown in a project detail view and in timeline overview shall be based on the same underlying milestone data.
   Updates to milestones shall be reflected across both views after save.

.. req:: Project date range derivation and override
   :id: REQ_PUSSLA_031
   :status: open
   :tags: project-management, data-model, timeline

   The system shall support project start and end boundaries derived from planned assignments and milestones.
   The system may support explicit manual start and end overrides in project metadata.

.. req:: Project hours-to-cost rollup in view
   :id: REQ_PUSSLA_032
   :status: open
   :tags: project-management, finance, frontend

   The project planning view shall provide cost rollups derived from planned hours and configured rates.
   The view shall support at least weekly and total cost summaries.

.. req:: Structured title-role and skill profile for people
   :id: REQ_PUSSLA_033
   :status: open
   :tags: staffing, data-model, skills

   The system shall store a structured people profile with title/role and skills for staffing queries.
   Skill entries shall support normalized matching terms (for example ``c++``, ``python``, ``rust``, ``zephyr``) and may include proficiency metadata.

.. req:: Staffing request with role-skill-time constraints
   :id: REQ_PUSSLA_034
   :status: open
   :tags: staffing, matching, planning

   The system shall support staffing requests containing required role/title, required skills, start date, and duration.
   The system shall convert date-based duration requests (for example ``4 months starting in August``) into a planning period suitable for week-based matching.

.. req:: Availability validation over requested period
   :id: REQ_PUSSLA_035
   :status: open
   :tags: staffing, matching, validation

   Candidate matching shall validate availability across the full requested period, not only the first week.
   Availability shall consider existing committed assignments, configured capacity hours, and capacity exceptions.

.. req:: Candidate ranking and explainability
   :id: REQ_PUSSLA_036
   :status: open
   :tags: staffing, matching, explainability

   The system shall return a ranked list of suitable candidates for a staffing request.
   Each recommendation shall include an explanation of match factors such as role fit, skill fit, and availability fit.

.. req:: External agent staffing request workflow
   :id: REQ_PUSSLA_037
   :status: open
   :tags: staffing, workflow, integration

   The system shall support staffing requests expressed in structured data, suitable for use by external tools and agents (for example Codex or Claude Code) operating on the planning folder.
   Example intent: ``book a suitable available Embedded SW developer with C++ and Zephyr for 4 months starting in August for the Deathstar project``.
   The workflow shall convert such intents into a structured request and require explicit user confirmation before booking.

.. req:: Booking outcome and assignment state
   :id: REQ_PUSSLA_038
   :status: open
   :tags: staffing, workflow, planning

   Booking from staffing recommendations shall support creating tentative or committed assignments.
   The outcome shall be persisted in planning data with traceability to the originating staffing request.

.. req:: Staffing conflict handling and fallback recommendations
   :id: REQ_PUSSLA_039
   :status: open
   :tags: staffing, validation, workflow

   If no candidate fully satisfies the staffing constraints, the system shall provide fallback recommendations with explicit constraint gaps.
   If a booking attempt causes conflicts, the system shall present the conflicts and allow cancel or save with explicit acknowledgement according to policy.
