# 🧩 Pussla: Next Steps

*Bitarna faller på plats!* Here are the prioritized next steps to continue evolving our Planning-as-Code system.

## 🛠 Automation & CI/CD
- [ ] **Automated Dashboard:** Set up a GitHub Action to run `src/aggregate_planning_data.py` on every push to `main` and update a `DASHBOARD.md` file.
- [ ] **Expanded CI Validation:** Update `.github/workflows/validate.yml` to include the new cross-reference checks and the unit test suite (`make test`).

## 📊 Analytics & Reporting
- [ ] **Capacity vs. Allocation:** Compare actual staffing in `allocations/` against `capacity_target` in `projects/` to identify under-resourced projects.
- [ ] **Visual Heatmaps:** Generate SVG or HTML-based heatmaps with color-coding for utilization levels (e.g., Green for 80-100%, Red for >100%).
- [ ] **Quarterly Forecasting:** Add a script to aggregate data by quarter (Q1, Q2, etc.) for high-level management reviews.

## 👥 People & Skills
- [ ] **Skill Mapping:** Add a `skills` or `role` field to the `/identity/` layer to allow for resource gap analysis.
- [ ] **Availability Tracking:** Add support for tracking planned leave/vacation more explicitly to improve forecast accuracy.

## 📂 Data Integrity
- [ ] **Schema Enforcement:** Use JSON Schema or Pydantic to formalize the YAML structures and provide better error messages during validation.
- [ ] **Alias Uniqueness:** Add a check to ensure no two files in `identity/` or `allocations/` share the same alias (case-insensitive).

---
**Nu kör vi!**
