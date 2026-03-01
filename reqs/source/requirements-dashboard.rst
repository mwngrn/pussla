Dashboard Requirements
======================

This page contains dashboard-specific requirements for timeline views,
visualization, filtering, formatting, and edit behavior.

.. req:: Dashboard visualization
   :id: REQ_PUSSLA_007
   :status: open
   :tags: frontend

   The system shall be able to generate a visual heatmap and quarterly forecasts based on the underlying text files.

.. req:: Dashboard filter for past periods
   :id: REQ_PUSSLA_012
   :status: open
   :tags: dashboard, ux, filtering

   The dashboard shall allow users to hide or show past weeks and past months, so users can focus on the current week and future periods when planning.

.. req:: Dashboard grouped calendar headers
   :id: REQ_PUSSLA_013
   :status: open
   :tags: dashboard, ux, frontend

   The dashboard shall show grouped headers with Year at the top, Month (without year) in the middle, and Week at the bottom.
   Month headers shall also show allocation percentage for each month group.

.. req:: Dashboard weekly footer summary
   :id: REQ_PUSSLA_014
   :status: open
   :tags: dashboard, frontend, reporting

   The dashboard shall show a footer row with allocation percentage per week column.
   Footer values shall update when visible users/weeks change due to filters.

.. req:: Dashboard week label format
   :id: REQ_PUSSLA_015
   :status: open
   :tags: dashboard, frontend, formatting

   The dashboard shall display week labels in `Www` format (for example `W01`) while keeping canonical storage/API keys in `YYYY-Www` format.

.. req:: Dashboard editable weekly planning
   :id: REQ_PUSSLA_016
   :status: open
   :tags: dashboard, editing, api, data-write

   The dashboard shall allow editing weekly planning for a selected person-week with support for multiple projects in the same week.
   The dashboard shall support editing planned hours and show derived allocation percentages.
   Saving shall require explicit user action in the GUI (Save), support cancel/discard, and persist changes to planning people profile files (Markdown with YAML frontmatter).
   Overbooking (sum of planned hours greater than capacity hours, equivalent to derived percentage greater than 100%) is allowed and shall not block save.
