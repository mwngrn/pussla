import { useEffect, useMemo, useState } from "react";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import {
  DashboardUser,
  ProjectSlot,
  buildProjectContextMap,
  extractProjects,
  fetchDashboardData,
  updateAllocation,
  updateProject,
} from "@/api/dashboard";
import { useSettingsStore } from "@/store/settings.store";
import {
  buildCalendarGroups,
  formatWeekLabel,
  isoWeekStartDate,
  parseIsoWeek,
} from "@/modules/planning/utils";
import { Search, Pencil, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import {
  DEFAULT_WEEKLY_CAPACITY_HOURS,
  hoursToLoadPercent,
  loadPercentToHours,
  mapMilestonesToWeeks,
} from "./representation_policy.js";

interface EditState {
  user: DashboardUser;
  week: string;
  hours: string;
  state: "committed" | "tentative";
}

interface MilestoneDraft {
  id: string;
  title: string;
  date: string;
}

interface MilestoneEditState {
  id: string;
  week: string;
  title: string;
  date: string;
  isNew: boolean;
}

function toDateISOFromWeek(week: string): string {
  const parsed = parseIsoWeek(week);
  if (!parsed) return week;
  const date = isoWeekStartDate(parsed.year, parsed.week);
  return date.toISOString().slice(0, 10);
}

function getProjectAssignments(
  user: DashboardUser,
  week: string,
  project: string
): ProjectSlot[] {
  const slot = user.weekly_stats.find((s) => s.week === week);
  if (!slot) return [];
  return slot.projects.filter((p) => p.project === project);
}

function getProjectHours(
  user: DashboardUser,
  week: string,
  project: string,
  includeTentative: boolean
): number {
  const assignments = getProjectAssignments(user, week, project);
  let total = 0;
  for (const entry of assignments) {
    const state = entry.state ?? "committed";
    if (!includeTentative && state === "tentative") continue;
    total +=
      typeof entry.planned_hours === "number"
        ? entry.planned_hours
        : loadPercentToHours(entry.load ?? 0, DEFAULT_WEEKLY_CAPACITY_HOURS);
  }
  return Math.round(total * 10) / 10;
}

function getProjectState(
  user: DashboardUser,
  week: string,
  project: string
): "committed" | "tentative" {
  const entries = getProjectAssignments(user, week, project);
  if (entries.some((e) => (e.state ?? "committed") === "committed")) {
    return "committed";
  }
  return "tentative";
}

function buildUpdatedAllocations(
  user: DashboardUser,
  week: string,
  project: string,
  newHours: number,
  state: "committed" | "tentative"
): Array<{
  project: string;
  load?: number;
  planned_hours?: number;
  capacity_hours?: number;
  state?: "committed" | "tentative";
}> {
  const slot = user.weekly_stats.find((s) => s.week === week);
  const existing = slot?.projects ?? [];
  const next = existing
    .filter((p) => p.project !== project)
    .map((p) => ({
      project: p.project,
      load: p.load,
      planned_hours: p.planned_hours,
      capacity_hours: p.capacity_hours ?? DEFAULT_WEEKLY_CAPACITY_HOURS,
      state: (p.state ?? "committed") as "committed" | "tentative",
    }));

  if (newHours > 0) {
    next.push({
      project,
      load: hoursToLoadPercent(newHours, DEFAULT_WEEKLY_CAPACITY_HOURS),
      planned_hours: Math.round(newHours * 10) / 10,
      capacity_hours: DEFAULT_WEEKLY_CAPACITY_HOURS,
      state,
    });
  }

  return next;
}

export function ProjectManagementPage() {
  const { includePii } = useSettingsStore();
  const qc = useQueryClient();
  const [search, setSearch] = useState("");
  const [selectedProject, setSelectedProject] = useState("");
  const [edit, setEdit] = useState<EditState | null>(null);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [includeTentative, setIncludeTentative] = useState(true);
  const [settingsOpen, setSettingsOpen] = useState(false);

  const [hourlyRateDraft, setHourlyRateDraft] = useState("");
  const [startOverrideDraft, setStartOverrideDraft] = useState("");
  const [endOverrideDraft, setEndOverrideDraft] = useState("");
  const [milestones, setMilestones] = useState<MilestoneDraft[]>([]);
  const [milestoneEdit, setMilestoneEdit] = useState<MilestoneEditState | null>(null);

  const { data, isLoading, error: loadError } = useQuery({
    queryKey: ["dashboard", includePii],
    queryFn: () => fetchDashboardData(includePii),
  });

  const projectOptions = useMemo(() => (data ? extractProjects(data) : []), [data]);

  const projectContextMap = useMemo(
    () => (data ? buildProjectContextMap(data) : {}),
    [data]
  );

  const projectContext = selectedProject ? projectContextMap[selectedProject] : undefined;

  useEffect(() => {
    if (!projectOptions.length) {
      if (selectedProject) setSelectedProject("");
      return;
    }
    if (!selectedProject || !projectOptions.includes(selectedProject)) {
      setSelectedProject(projectOptions[0]);
    }
  }, [projectOptions, selectedProject]);

  useEffect(() => {
    const rate = projectContext?.hourly_rate;
    setHourlyRateDraft(typeof rate === "number" ? String(rate) : "");
    setStartOverrideDraft(projectContext?.start_week_override ?? "");
    setEndOverrideDraft(projectContext?.end_week_override ?? "");
    const sourceMilestones = projectContext?.milestones ?? [];
    setMilestones(
      sourceMilestones.map((m, idx) => ({
        id: m.id ?? `ms-${idx + 1}`,
        title: m.title,
        date: m.date,
      }))
    );
  }, [projectContext?.hourly_rate, projectContext?.start_week_override, projectContext?.end_week_override, projectContext?.milestones]);

  const weeks = useMemo(() => {
    if (!data) return [];
    return data.weeks.slice(0, 26);
  }, [data]);

  const { yearGroups, monthGroups } = useMemo(() => buildCalendarGroups(weeks), [weeks]);

  const filteredUsers = useMemo(() => {
    if (!data || !selectedProject) return [];
    const q = search.trim().toLowerCase();

    return data.users.filter((u) => {
      const hasProject = weeks.some(
        (w) => getProjectHours(u, w, selectedProject, includeTentative) > 0
      );
      if (!hasProject) return false;
      if (!q) return true;
      return (
        u.display_name.toLowerCase().includes(q) ||
        u.alias.toLowerCase().includes(q)
      );
    });
  }, [data, selectedProject, search, includeTentative, weeks]);

  const weeklyTotals = useMemo(() => {
    if (!selectedProject) return {};
    const totals: Record<string, number> = {};
    for (const week of weeks) totals[week] = 0;
    for (const user of filteredUsers) {
      for (const week of weeks) {
        totals[week] += getProjectHours(user, week, selectedProject, includeTentative);
      }
    }
    return totals;
  }, [filteredUsers, selectedProject, includeTentative, weeks]);

  const derivedRange = useMemo(() => {
    const activeWeeks = weeks.filter((w) => (weeklyTotals[w] ?? 0) > 0);
    return {
      start: activeWeeks[0] ?? null,
      end: activeWeeks[activeWeeks.length - 1] ?? null,
    };
  }, [weeks, weeklyTotals]);

  const resolvedStart =
    projectContext?.start_week_override ||
    projectContext?.resolved_start_week ||
    projectContext?.start_week ||
    derivedRange.start;
  const resolvedEnd =
    projectContext?.end_week_override ||
    projectContext?.resolved_end_week ||
    projectContext?.end_week ||
    derivedRange.end;

  const hourlyRate = typeof projectContext?.hourly_rate === "number" ? projectContext.hourly_rate : 0;

  const weeklyCosts = useMemo(() => {
    const costs: Record<string, number> = {};
    for (const week of weeks) {
      costs[week] = Math.round(((weeklyTotals[week] ?? 0) * hourlyRate) * 100) / 100;
    }
    return costs;
  }, [weeks, weeklyTotals, hourlyRate]);

  const totalAcrossWeeks = Object.values(weeklyTotals).reduce((sum, value) => sum + value, 0);
  const totalCostAcrossWeeks = Object.values(weeklyCosts).reduce((sum, value) => sum + value, 0);

  const milestonesByWeek = useMemo(
    () => mapMilestonesToWeeks(milestones, weeks),
    [milestones, weeks]
  );

  const openEditor = (user: DashboardUser, week: string) => {
    const currentHours = getProjectHours(user, week, selectedProject, true);
    const currentState = getProjectState(user, week, selectedProject);
    setError(null);
    setEdit({ user, week, hours: String(currentHours), state: currentState });
  };

  const handleSaveAllocation = async () => {
    if (!edit || !selectedProject) return;
    const nextHours = Number(edit.hours);
    if (!Number.isFinite(nextHours) || nextHours < 0) {
      setError("Hours must be a non-negative number.");
      return;
    }

    setSaving(true);
    setError(null);
    try {
      const allocations = buildUpdatedAllocations(
        edit.user,
        edit.week,
        selectedProject,
        nextHours,
        edit.state
      );
      await updateAllocation({
        alias: edit.user.alias,
        week: edit.week,
        allocations,
      });
      qc.invalidateQueries({ queryKey: ["dashboard"] });
      setEdit(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to save allocation");
    } finally {
      setSaving(false);
    }
  };

  const handleSaveProjectMeta = async () => {
    if (!selectedProject) return;
    setSaving(true);
    setError(null);
    try {
      await updateProject({
        project: selectedProject,
        updates: {
          hourly_rate: hourlyRateDraft.trim() ? Number(hourlyRateDraft) : null,
          start_week_override: startOverrideDraft.trim() || null,
          end_week_override: endOverrideDraft.trim() || null,
        },
      });
      qc.invalidateQueries({ queryKey: ["dashboard"] });
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to save project settings");
    } finally {
      setSaving(false);
    }
  };

  const persistMilestones = async (nextMilestones: MilestoneDraft[]) => {
    if (!selectedProject) return;
    setSaving(true);
    setError(null);
    try {
      const sortedMilestones = [...nextMilestones].sort(
        (a, b) => a.date.localeCompare(b.date) || a.title.localeCompare(b.title)
      );
      await updateProject({
        project: selectedProject,
        updates: { milestones: sortedMilestones },
      });
      setMilestones(sortedMilestones);
      qc.invalidateQueries({ queryKey: ["dashboard"] });
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to save milestones");
    } finally {
      setSaving(false);
    }
  };

  const openNewMilestoneForWeek = (week: string) => {
    setMilestoneEdit({
      id: `ms-${Date.now()}`,
      week,
      title: "",
      date: toDateISOFromWeek(week),
      isNew: true,
    });
  };

  const openExistingMilestone = (milestone: MilestoneDraft, week: string) => {
    setMilestoneEdit({
      id: milestone.id,
      week,
      title: milestone.title,
      date: milestone.date,
      isNew: false,
    });
  };

  const handleSaveMilestoneEdit = async () => {
    if (!milestoneEdit) return;
    const title = milestoneEdit.title.trim();
    if (!title) {
      setError("Milestone title is required.");
      return;
    }
    if (!/^\d{4}-\d{2}-\d{2}$/.test(milestoneEdit.date)) {
      setError("Milestone date must be YYYY-MM-DD.");
      return;
    }
    const next = milestoneEdit.isNew
      ? [...milestones, { id: milestoneEdit.id, title, date: milestoneEdit.date }]
      : milestones.map((m) =>
          m.id === milestoneEdit.id ? { ...m, title, date: milestoneEdit.date } : m
        );
    await persistMilestones(next);
    setMilestoneEdit(null);
  };

  const handleDeleteMilestoneEdit = async () => {
    if (!milestoneEdit || milestoneEdit.isNew) {
      setMilestoneEdit(null);
      return;
    }
    const next = milestones.filter((m) => m.id !== milestoneEdit.id);
    await persistMilestones(next);
    setMilestoneEdit(null);
  };

  return (
    <div className="p-6 space-y-4">
      <div className="flex flex-col gap-4 lg:flex-row lg:items-end lg:justify-between">
        <div>
          <h2 className="text-xl font-semibold text-gray-900">Project Management</h2>
          <p className="text-sm text-gray-500">Projektvy i timmar, utilization-vyer i procent.</p>
        </div>

        <div className="flex flex-col gap-2 sm:flex-row sm:items-center">
          <label className="text-xs font-medium text-gray-600 uppercase tracking-wide">Active project</label>
          <select
            value={selectedProject}
            onChange={(e) => setSelectedProject(e.target.value)}
            className="h-9 min-w-64 rounded border border-gray-300 px-3 text-sm focus:outline-none focus:ring-1 focus:ring-blue-500"
          >
            {!projectOptions.length && <option value="">No projects</option>}
            {projectOptions.map((name) => (
              <option key={name} value={name}>
                {name}
              </option>
            ))}
          </select>

          <label className="flex items-center gap-1.5 text-xs text-gray-600">
            <input
              type="checkbox"
              checked={includeTentative}
              onChange={(e) => setIncludeTentative(e.target.checked)}
            />
            Include tentative
          </label>

          <Button
            size="sm"
            variant="outline"
            onClick={() => setSettingsOpen(true)}
          >
            Project settings
          </Button>

          <div className="relative">
            <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-gray-400 pointer-events-none" />
            <input
              type="text"
              placeholder="Search people…"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              className="pl-8 pr-3 py-1.5 h-9 text-sm rounded border border-gray-300 focus:outline-none focus:ring-1 focus:ring-blue-500 w-52"
            />
          </div>
        </div>
      </div>

      {isLoading && <p className="text-sm text-gray-500">Loading…</p>}
      {loadError && (
        <p className="text-sm text-red-600">
          {loadError instanceof Error ? loadError.message : "Failed to load"}
        </p>
      )}

      {!isLoading && !loadError && selectedProject && (
        <>
          <div className="grid gap-3 lg:grid-cols-4">
            <div className="rounded border bg-white p-3 text-sm">
              <div className="text-gray-500 text-xs">Allocated people</div>
              <div className="text-lg font-semibold text-gray-900">{filteredUsers.length}</div>
            </div>
            <div className="rounded border bg-white p-3 text-sm">
              <div className="text-gray-500 text-xs">Date range</div>
              <div className="text-sm font-medium text-gray-900">
                {resolvedStart ?? "—"} to {resolvedEnd ?? "—"}
              </div>
            </div>
            <div className="rounded border bg-white p-3 text-sm">
              <div className="text-gray-500 text-xs">Total planned hours</div>
              <div className="text-lg font-semibold text-gray-900">{Math.round(totalAcrossWeeks * 10) / 10}h</div>
            </div>
            <div className="rounded border bg-white p-3 text-sm">
              <div className="text-gray-500 text-xs">Total cost</div>
              <div className="text-lg font-semibold text-gray-900">{Math.round(totalCostAcrossWeeks)} </div>
              <div className="text-[11px] text-gray-500">@ {hourlyRate || 0}/h</div>
            </div>
          </div>

          {error && <p className="text-sm text-red-600 bg-red-50 rounded px-3 py-2">{error}</p>}

          <div className="rounded-lg border bg-white overflow-auto">
            <table className="w-full text-xs min-w-max border-separate border-spacing-0">
              <thead>
                <tr>
                  <th
                    className="text-left sticky left-0 z-20 bg-gray-50 border-b border-r px-3 py-2 font-medium text-gray-600"
                    rowSpan={3}
                  >
                    Person
                  </th>
                  {yearGroups.map((yg) => (
                    <th
                      key={`y-${yg.label}`}
                      colSpan={yg.span}
                      className="border-b border-l px-2 py-1 text-center text-xs font-semibold text-gray-500 bg-gray-50"
                    >
                      {yg.label}
                    </th>
                  ))}
                </tr>
                <tr>
                  {monthGroups.map((mg) => (
                    <th
                      key={`m-${mg.key}`}
                      colSpan={mg.span}
                      className="border-b border-l px-2 py-1 text-center text-xs font-semibold text-gray-500 bg-gray-50"
                    >
                      {mg.label}
                    </th>
                  ))}
                </tr>
                <tr className="border-b bg-gray-50">
                  {weeks.map((week) => (
                    <th
                      key={week}
                      className="text-center px-2 py-2 font-medium text-gray-600 whitespace-nowrap border-l border-b"
                    >
                      {formatWeekLabel(week)}
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody>
                <tr className="border-b bg-amber-50/40">
                  <td className="sticky left-0 z-10 bg-amber-50 px-3 py-2 whitespace-nowrap border-r">
                    <div className="font-medium text-amber-900">Milestones</div>
                    <div className="text-[10px] text-amber-700">Timeline markers</div>
                  </td>
                  {weeks.map((week) => {
                    const items = milestonesByWeek[week] ?? [];
                    return (
                      <td key={`ms-${week}`} className="px-1 py-1 border-l align-top">
                        {items.length === 0 ? (
                          <button
                            className="w-full h-8 rounded border border-dashed border-amber-200 text-[10px] text-amber-500 hover:bg-amber-100/60 transition-colors"
                            onClick={() => openNewMilestoneForWeek(week)}
                            title={`Add milestone in ${formatWeekLabel(week)}`}
                          >
                            Add
                          </button>
                        ) : (
                          <div className="space-y-1">
                            {items.map((ms) => (
                              <button
                                key={ms.id}
                                className="w-full text-left rounded bg-amber-100 text-amber-900 px-1.5 py-0.5 text-[10px] leading-tight hover:bg-amber-200 transition-colors"
                                title={`${ms.date} · ${ms.title}`}
                                onClick={() => openExistingMilestone(ms, week)}
                              >
                                {ms.title}
                              </button>
                            ))}
                          </div>
                        )}
                      </td>
                    );
                  })}
                </tr>
                {!filteredUsers.length && (
                  <tr>
                    <td
                      colSpan={weeks.length + 1}
                      className="px-3 py-8 text-center text-gray-500"
                    >
                      No people found for selected project.
                    </td>
                  </tr>
                )}
                {filteredUsers.map((user) => (
                  <tr key={user.alias} className="border-b last:border-0">
                    <td className="sticky left-0 z-10 bg-white px-3 py-2 whitespace-nowrap border-r">
                      <div className="font-medium text-gray-900">{user.display_name}</div>
                      {user.alias !== user.display_name && (
                        <div className="text-[10px] text-gray-400">{user.alias}</div>
                      )}
                    </td>
                    {weeks.map((week) => {
                      const hours = getProjectHours(user, week, selectedProject, includeTentative);
                      const state = getProjectState(user, week, selectedProject);
                      return (
                        <td key={`${user.alias}-${week}`} className="px-1 py-1 border-l">
                          <button
                            className={`w-full h-8 rounded border text-[11px] transition-colors ${
                              hours > 0
                                ? state === "tentative"
                                  ? "border-amber-200 bg-amber-50 text-amber-700 hover:bg-amber-100"
                                  : "border-blue-200 bg-blue-50 text-blue-700 hover:bg-blue-100"
                                : "border-gray-200 text-gray-400 hover:bg-gray-50"
                            }`}
                            onClick={() => openEditor(user, week)}
                            title="Edit project hours/state for this person/week"
                          >
                            <span className="inline-flex items-center gap-1">
                              {hours > 0 ? `${hours}h` : "—"}
                              <Pencil className="h-3 w-3 opacity-60" />
                            </span>
                          </button>
                        </td>
                      );
                    })}
                  </tr>
                ))}
              </tbody>
              {filteredUsers.length > 0 && (
                <tfoot>
                  <tr className="border-t bg-gray-50">
                    <td className="sticky left-0 z-10 bg-gray-50 px-3 py-2 font-medium text-gray-700 border-r">
                      Weekly total (hours / cost)
                    </td>
                    {weeks.map((week) => (
                      <td
                        key={`total-${week}`}
                        className="text-center px-2 py-2 font-medium text-gray-700 border-l"
                      >
                        <div>{Math.round((weeklyTotals[week] ?? 0) * 10) / 10}h</div>
                        <div className="text-[10px] text-gray-500">{Math.round(weeklyCosts[week] ?? 0)}</div>
                      </td>
                    ))}
                  </tr>
                </tfoot>
              )}
            </table>
          </div>
        </>
      )}

      <Dialog open={!!edit} onOpenChange={(open) => !open && setEdit(null)}>
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Edit project allocation</DialogTitle>
          </DialogHeader>
          {edit && (
            <div className="space-y-4">
              <div className="text-sm text-gray-600">
                <span className="font-medium text-gray-900">{selectedProject}</span>
                {" · "}
                <span>{edit.user.display_name}</span>
                {" · "}
                <span>{formatWeekLabel(edit.week)}</span>
                {" · "}
                <span>{toDateISOFromWeek(edit.week)}</span>
              </div>

              <label className="text-xs uppercase tracking-wide text-gray-500 font-medium block">
                Hours
                <Input
                  type="number"
                  min={0}
                  step={0.5}
                  value={edit.hours}
                  onChange={(e) =>
                    setEdit((prev) =>
                      prev ? { ...prev, hours: e.target.value } : prev
                    )
                  }
                />
              </label>

              <label className="text-xs uppercase tracking-wide text-gray-500 font-medium block">
                State
                <select
                  value={edit.state}
                  onChange={(e) =>
                    setEdit((prev) =>
                      prev
                        ? {
                            ...prev,
                            state: e.target.value as "committed" | "tentative",
                          }
                        : prev
                    )
                  }
                  className="h-9 w-full rounded border border-gray-300 px-2 text-sm focus:outline-none focus:ring-1 focus:ring-blue-500"
                >
                  <option value="committed">Committed</option>
                  <option value="tentative">Tentative</option>
                </select>
              </label>

              {error && (
                <p className="text-sm text-red-600 bg-red-50 rounded px-3 py-2">
                  {error}
                </p>
              )}

              <div className="flex justify-end gap-2 pt-2 border-t">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => setEdit(null)}
                  disabled={saving}
                >
                  Cancel
                </Button>
                <Button size="sm" onClick={handleSaveAllocation} disabled={saving}>
                  {saving ? "Saving…" : "Save"}
                </Button>
              </div>
            </div>
          )}
        </DialogContent>
      </Dialog>

      <Dialog open={settingsOpen} onOpenChange={setSettingsOpen}>
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Project settings</DialogTitle>
          </DialogHeader>
          <div className="space-y-3">
            <label className="text-xs text-gray-600 block">
              Hourly rate
              <Input
                value={hourlyRateDraft}
                onChange={(e) => setHourlyRateDraft(e.target.value)}
                type="number"
                min={0}
                step={1}
              />
            </label>
            <label className="text-xs text-gray-600 block">
              Start override (YYYY-Www)
              <Input
                value={startOverrideDraft}
                onChange={(e) => setStartOverrideDraft(e.target.value)}
                placeholder="2026-W09"
              />
            </label>
            <label className="text-xs text-gray-600 block">
              End override (YYYY-Www)
              <Input
                value={endOverrideDraft}
                onChange={(e) => setEndOverrideDraft(e.target.value)}
                placeholder="2026-W20"
              />
            </label>
            <div className="flex justify-end gap-2 pt-2 border-t">
              <Button
                size="sm"
                variant="outline"
                onClick={() => setSettingsOpen(false)}
                disabled={saving}
              >
                Cancel
              </Button>
              <Button
                size="sm"
                onClick={async () => {
                  await handleSaveProjectMeta();
                  setSettingsOpen(false);
                }}
                disabled={saving}
              >
                {saving ? "Saving…" : "Save settings"}
              </Button>
            </div>
          </div>
        </DialogContent>
      </Dialog>

      <Dialog open={!!milestoneEdit} onOpenChange={(open) => !open && setMilestoneEdit(null)}>
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>
              {milestoneEdit?.isNew ? "Add milestone" : "Edit milestone"}
            </DialogTitle>
          </DialogHeader>
          {milestoneEdit && (
            <div className="space-y-4">
              <div className="text-sm text-gray-600">
                <span className="font-medium text-gray-900">{selectedProject}</span>
                {" · "}
                <span>{formatWeekLabel(milestoneEdit.week)}</span>
              </div>
              <label className="text-xs uppercase tracking-wide text-gray-500 font-medium block">
                Title
                <Input
                  value={milestoneEdit.title}
                  onChange={(e) =>
                    setMilestoneEdit((prev) =>
                      prev ? { ...prev, title: e.target.value } : prev
                    )
                  }
                />
              </label>
              <label className="text-xs uppercase tracking-wide text-gray-500 font-medium block">
                Date
                <Input
                  type="date"
                  value={milestoneEdit.date}
                  onChange={(e) =>
                    setMilestoneEdit((prev) =>
                      prev ? { ...prev, date: e.target.value } : prev
                    )
                  }
                />
              </label>
              <div className="flex justify-between pt-2 border-t">
                <div>
                  {!milestoneEdit.isNew && (
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={handleDeleteMilestoneEdit}
                      disabled={saving}
                    >
                      <Trash2 className="h-3 w-3 mr-1" />
                      Delete
                    </Button>
                  )}
                </div>
                <div className="flex gap-2">
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => setMilestoneEdit(null)}
                    disabled={saving}
                  >
                    Cancel
                  </Button>
                  <Button size="sm" onClick={handleSaveMilestoneEdit} disabled={saving}>
                    {saving ? "Saving…" : "Save"}
                  </Button>
                </div>
              </div>
            </div>
          )}
        </DialogContent>
      </Dialog>
    </div>
  );
}
