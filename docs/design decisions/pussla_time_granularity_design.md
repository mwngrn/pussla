# Design Choice: Discrete Weekly Buckets vs. Continuous Date Ranges

## Status
**Accepted**

## Context
Resource planning for a 100-person consulting firm requires high-frequency updates. Assignments shift, vacations are booked, and project priorities change weekly. We need a data format that remains readable in Git and easy to aggregate into monthly and quarterly views.

## Decision
We will use **discrete weekly buckets** (based on ISO week numbers) as the primary unit of time for allocations, rather than continuous "Start Date to End Date" ranges.

---

## Rationale

### 1. Superior Git Diff Readability
In a continuous date-range system, changing the load for a single week (e.g., during a holiday) requires splitting one database entry into three. This creates a messy "delete and replace" diff in Git. With weekly buckets, changing one week only affects the specific line for that week, making the audit trail crystal clear.

### 2. Computational Simplicity (The "No-Overlap" Rule)
Date ranges require complex logic to calculate how many "billable days" fall into a specific month, especially when a range spans across month boundaries. By using weeks, the math for a monthly or quarterly rollup becomes simple addition:
$$Total Load = \sum Week_{load}$$
This reduces the risk of calculation errors in the dashboard and the AI analysis engine.

### 3. Direct Mapping to Heatmap Visualization
The user requirement is to view the organization as a "Week-by-Week" grid. By storing the data in the same "shape" as it is displayed, we eliminate the need for an expensive transformation layer in the frontend. One YAML line = one Heatmap cell.

### 4. The "Consultancy Currency"
In professional services, the **Week** is the standard unit of measurement for capacity, reporting, and billing. Aligning the planning tool with the mental model of the consultants reduces friction.

---

## Implementation Example

Instead of a single "Start: Jan 1, End: June 30" entry, we group weeks to keep the YAML concise while maintaining discrete control:

```yaml
# Example: Allocation for FishCatcher
- project: "Project-Neon"
  weeks: [10, 11, 12, 13]  # March weeks
  load: 80
- project: "Project-Neon"
  weeks: [14]             # Specific adjustment for Easter week
  load: 20
```

---

## Consequences

* **Year-End Boundaries:** ISO Week 1 often spans two years. The parser must be aware of year-week mapping (e.g., `2026-W52`) to ensure data doesn't "leak" into the wrong year.
* **Month Rollups:** Since weeks rarely align perfectly with month starts/ends, the system will attribute a week's load to the month where the majority of its workdays reside, or split it based on a 5-day work week (Monday–Friday) for high-precision reporting.
* **Verbosity:** Files might become longer than a simple date-range list, but this is a worthwhile trade-off for the increased control and auditability.
