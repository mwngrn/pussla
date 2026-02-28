Staffing And Matching Requirements
=================================

This page contains staffing requirements for role/skill modeling, matching,
availability checks, recommendation quality, and booking workflows.

.. req:: Possible assignment and leads
   :id: REQ_PUSSLA_010
   :status: open
   :tags: structure

   It shall be possible to track future leads and possible assignments for a person in order to track what projects he/she might be going into.
   These shall be clearly separately identifiable from decided assignments and be possible to filter out.

.. req:: Role and skill based staffing lookup
   :id: REQ_PUSSLA_011
   :status: open
   :tags: staffing, data-model, matching

   The system shall track each person's title/role and skills, including skill keywords used for matching (for example C++, Python, Rust, Zephyr).
   It shall be possible to identify available people for a project based on required title/role and required skills.

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
