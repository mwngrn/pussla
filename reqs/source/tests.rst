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

.. test:: Dashboard editable weekly allocations writeback
   :id: TEST_PUSSLA_003
   :status: passed
   :tags: dashboard, editing, api, unit
   :links: REQ_PUSSLA_016

   Covered by ``tests/test_dashboard_editing.py``.
   Verifies multi-project week updates, YAML persistence behavior, and payload validation.

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


Test-to-Requirement Matrix
==========================

.. needtable:: Tests and linked requirements
   :types: test
   :columns: id;title;status;links
   :sort: id
