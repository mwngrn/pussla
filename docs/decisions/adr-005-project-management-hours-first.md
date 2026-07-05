# Design Choice: Hours-First Planning Model for Project Management

## Status
**Accepted**

## Context
Pussla currently presents utilization as allocation percentages per person-week. This is useful for capacity visualization, but project management also requires hour-based planning and cost calculation. Cost models, budget tracking, and forecast variance are all naturally measured in hours and rates, not percentages.

If planning data is stored only as percentages, cost calculations become indirect and ambiguous because the actual hours depend on person capacity assumptions.

## Decision
The system will use **planned hours** as the canonical planning unit for project management.

- Source of truth for planning entries: `planned_hours` per person, project, and week.
- Source of truth for person capacity: `capacity_hours` per person and week, default `40`.
- Allocation percentage is **derived**, not primary:
  `allocation_percent = planned_hours / capacity_hours * 100`.

The UI and APIs may show both hours and percentages, but persistence shall prioritize hours and capacity.

## Rationale

### 1. Direct support for cost calculations
Project cost is straightforward when hours are primary:
`cost = planned_hours * hourly_rate`.

### 2. Consistent capacity math
Over-allocation and under-allocation are clearer with explicit capacity:
`sum(planned_hours_week) > capacity_hours_week`.

### 3. Better support for exceptions
Part-time work, leave, and holiday weeks are naturally represented by adjusted `capacity_hours` instead of implicit percentage assumptions.

### 4. Cleaner migration path
Existing percentage data can still be supported by conversion:
`planned_hours = allocation_percent / 100 * capacity_hours`.

## Consequences

- Data model changes are required to store weekly capacity and planned hours.
- Existing percentage-based files and APIs need compatibility logic during migration.
- Validation rules must check both hour-based and derived-percent over-allocation.
- Dashboard views should continue to present percent heatmaps, now derived from hours.

## Non-Goals

- This decision does not require removing percentage views from the UI.
- This decision does not force immediate deprecation of all legacy percentage inputs.
