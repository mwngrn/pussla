# Frontend vs Dashboard Requirement Coverage Diff

Date: 2026-02-26

This compares requirement coverage between:
- `src/frontend` (React solution)
- `src/dashboard` (legacy dashboard implementation)

Note: `src/dashboard/run_dashboard.py` prefers serving `src/frontend/dist` when it exists, so this is a code-level comparison, not necessarily runtime behavior in every environment.

## Summary

`src/dashboard` currently supports more of the formally documented dashboard requirements.

- `src/dashboard` is stronger for `REQ_PUSSLA_013` and `REQ_PUSSLA_014`.
- Both solutions satisfy `REQ_PUSSLA_015` and `REQ_PUSSLA_016`.
- Neither solution currently implements `REQ_PUSSLA_012`.
- `src/frontend` has better quarter-navigation UX related to `REQ_PUSSLA_007`.

## Requirement-by-Requirement Diff

| Requirement | `src/frontend` | `src/dashboard` | Evidence |
|---|---|---|---|
| `REQ_PUSSLA_013` Grouped Year/Month/Week headers + month percentages | Partial (Month + Week only; no Year row; no month percentages) | Full | Frontend: `src/frontend/src/modules/planning/UtilizationPage.tsx` (month/week headers only). Dashboard: `src/dashboard/index.html` (year row, month row with percentages, week row). |
| `REQ_PUSSLA_014` Weekly footer summary updates with filters | Partial (footer exists, but averages computed from all users) | Full (computed from filtered users) | Frontend: footer row in `UtilizationPage.tsx`; averages from `calculateWeekAverages(weeks, data?.users ?? [])`. Dashboard: `users` is filtered by search before `calculateWeekAllocationPercentages(...)`. |
| `REQ_PUSSLA_015` Week label format `Www` | Full | Full | Frontend: `formatWeekLabel` in `src/frontend/src/modules/planning/utils.ts`. Dashboard: `formatWeekLabel` in `src/dashboard/week_format.js`. |
| `REQ_PUSSLA_016` Editable weekly allocations (multi-project, Save/Cancel, overbooking allowed) | Full | Full | Frontend: `EditWeekDialog.tsx` (add/remove rows, Save/Cancel, POST `/api/allocation/update`). Dashboard: modal editor in `index.html` with add/remove rows and save flow via same API. |
| `REQ_PUSSLA_012` Hide/show past weeks and months | Not implemented | Not implemented | No explicit hide/show past-period toggle in either implementation. |
| `REQ_PUSSLA_007` Heatmap + quarterly forecasts | Better support (heatmap + quarter navigation controls) | Partial (heatmap present; no quarter navigation controls) | Frontend: `GanttPage.tsx` has previous/next/today quarter controls. Dashboard: heatmap table only in `index.html`. |

## Key File References

- `src/frontend/src/modules/planning/UtilizationPage.tsx`
- `src/frontend/src/modules/planning/GanttPage.tsx`
- `src/frontend/src/modules/planning/EditWeekDialog.tsx`
- `src/frontend/src/modules/planning/utils.ts`
- `src/dashboard/index.html`
- `src/dashboard/header_groups.js`
- `src/dashboard/week_format.js`
- `src/dashboard/run_dashboard.py`

