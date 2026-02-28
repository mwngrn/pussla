Requirements
============

This section is the source of truth for formal requirements (Sphinx-needs).
Requirements are organized by domain to keep the specification navigable as it grows.

Requirement Domains
===================

Core and architecture requirements
----------------------------------

Cross-cutting constraints for privacy, data structure, validation, and planning-as-code.

.. toctree::
   :maxdepth: 1

   requirements-foundation

Dashboard requirements
----------------------

Requirements for visualization, timeline behavior, filtering, formatting, and editing flows.

.. toctree::
   :maxdepth: 1

   requirements-dashboard

Staffing and matching requirements
----------------------------------

Requirements for role/skill data, staffing requests, matching logic, and booking workflows.

.. toctree::
   :maxdepth: 1

   requirements-staffing

Project management requirements
-------------------------------

Requirements for hour-based project planning, milestones, and project-level summaries.

.. toctree::
   :maxdepth: 1

   project-management


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
