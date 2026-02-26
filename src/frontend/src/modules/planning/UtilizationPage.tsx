import { useMemo, useState, useCallback, useEffect, useRef } from "react";
import { useQuery } from "@tanstack/react-query";
import { fetchDashboardData, DashboardUser, WeeklyStats } from "@/api/dashboard";
import { useSettingsStore } from "@/store/settings.store";
import { ChevronLeft, ChevronRight, Search, Download } from "lucide-react";
import { EditWeekDialog } from "./EditWeekDialog";
import {
  formatWeekLabel,
  formatWeekDate,
  formatWeekFull,
  fteCellClass,
  buildCalendarGroups,
  quarterLabel,
  getFirstWeekOfQuarter,
  getCurrentISOWeek,
  calculateWeekAverages,
} from "./utils";

type SortKey = "surname" | "firstname" | "avg4";

export function UtilizationPage() {
  const { includePii, underAllocationThreshold: threshold } = useSettingsStore();
  const [quarterOffset, setQuarterOffset] = useState(0);
  const currentWeek = getCurrentISOWeek();
  const [sortKey, setSortKey] = useState<SortKey>("surname");
  const [sortDir, setSortDir] = useState<"asc" | "desc">("asc");
  const [search, setSearch] = useState("");
  const containerRef = useRef<HTMLDivElement>(null);
  const initialScrolled = useRef(false);

  const [editingUser, setEditingUser] = useState<DashboardUser | null>(null);
  const [editingSlot, setEditingSlot] = useState<WeeklyStats | null>(null);
  const [editOpen, setEditOpen] = useState(false);

  const { data, isLoading, error } = useQuery({
    queryKey: ["dashboard", includePii],
    queryFn: () => fetchDashboardData(includePii),
  });

  const weeks = data?.weeks ?? [];
  const averages = useMemo(
    () => calculateWeekAverages(weeks, data?.users ?? []),
    [weeks, data?.users]
  );

  const scrollToWeek = useCallback(
    (targetWeek: string, behavior: ScrollBehavior = "smooth") => {
      const container = containerRef.current;
      if (!container || weeks.length === 0) return;
      const week = weeks.find((w) => w >= targetWeek) ?? weeks[0];
      const th = container.querySelector<HTMLElement>(`[data-week="${week}"]`);
      if (!th) return;
      const containerLeft = container.getBoundingClientRect().left;
      const thLeft = th.getBoundingClientRect().left;
      const newScrollLeft =
        container.scrollLeft + (thLeft - containerLeft) - 216;
      container.scrollTo({ left: Math.max(0, newScrollLeft), behavior });
    },
    [weeks]
  );

  useEffect(() => {
    if (weeks.length > 0 && !initialScrolled.current) {
      scrollToWeek(currentWeek, "auto");
      initialScrolled.current = true;
    }
  }, [weeks, currentWeek, scrollToWeek]);

  const handlePrevQuarter = () => {
    const next = quarterOffset - 1;
    setQuarterOffset(next);
    scrollToWeek(getFirstWeekOfQuarter(next));
  };

  const handleNextQuarter = () => {
    const next = quarterOffset + 1;
    setQuarterOffset(next);
    scrollToWeek(getFirstWeekOfQuarter(next));
  };

  const handleToday = () => {
    setQuarterOffset(0);
    scrollToWeek(currentWeek);
  };

  // avg of next 4 weeks per user
  const next4Weeks = useMemo(
    () => weeks.filter((w) => w >= currentWeek).slice(0, 4),
    [weeks, currentWeek]
  );

  const avg4Map = useMemo(() => {
    const map: Record<string, number | null> = {};
    for (const user of data?.users ?? []) {
      map[user.alias] =
        next4Weeks.length > 0
          ? Math.round(
              next4Weeks.reduce((sum, w) => {
                const slot = user.weekly_stats.find((s) => s.week === w);
                return sum + (slot?.total_load ?? 0);
              }, 0) / next4Weeks.length
            )
          : null;
    }
    return map;
  }, [data?.users, next4Weeks]);

  const rows = useMemo(() => {
    const arr = [...(data?.users ?? [])];
    arr.sort((a, b) => {
      let cmp = 0;
      if (sortKey === "surname") {
        cmp = a.display_name
          .split(" ")
          .slice(-1)[0]
          .localeCompare(b.display_name.split(" ").slice(-1)[0]);
      } else if (sortKey === "firstname") {
        cmp = a.display_name
          .split(" ")[0]
          .localeCompare(b.display_name.split(" ")[0]);
      } else {
        cmp = (avg4Map[a.alias] ?? -1) - (avg4Map[b.alias] ?? -1);
      }
      return sortDir === "asc" ? cmp : -cmp;
    });

    if (!search.trim()) return arr;
    const q = search.toLowerCase();
    return arr.filter(
      (r) =>
        r.display_name.toLowerCase().includes(q) ||
        r.alias.toLowerCase().includes(q)
    );
  }, [data?.users, sortKey, sortDir, avg4Map, search]);

  const toggleAvgSort = () => {
    if (sortKey === "avg4") setSortDir((d) => (d === "asc" ? "desc" : "asc"));
    else {
      setSortKey("avg4");
      setSortDir("desc");
    }
  };

  const setNameSort = (key: "surname" | "firstname") => {
    if (sortKey === key) setSortDir((d) => (d === "asc" ? "desc" : "asc"));
    else {
      setSortKey(key);
      setSortDir("asc");
    }
  };

  const openEdit = (user: DashboardUser, slot: WeeklyStats) => {
    setEditingUser(user);
    setEditingSlot(slot);
    setEditOpen(true);
  };

  const handleExportCSV = () => {
    if (!data) return;
    const lines: string[] = [];
    lines.push(["Person", ...weeks].join(","));
    for (const row of rows) {
      lines.push(
        [
          `"${row.display_name}"`,
          ...weeks.map((w) => {
            const slot = row.weekly_stats.find((s) => s.week === w);
            return String(slot?.total_load ?? 0);
          }),
        ].join(",")
      );
    }
    lines.push(
      ["Average", ...weeks.map((w) => String(averages[w] ?? 0))].join(",")
    );

    const blob = new Blob([lines.join("\n")], {
      type: "text/csv;charset=utf-8;",
    });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `utilization-${quarterLabel(quarterOffset).replace(" ", "-")}.csv`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const { monthGroups } = buildCalendarGroups(weeks);

  return (
    <div className="p-4 md:p-6 flex flex-col gap-4 h-full">
      {/* Header */}
      <div className="flex items-center justify-between flex-wrap gap-3 shrink-0">
        <div className="flex items-center gap-3">
          <h2 className="text-xl font-semibold text-gray-900">Utilization</h2>
          <div className="flex rounded border overflow-hidden text-xs">
            <button
              onClick={() => setNameSort("surname")}
              className={`px-2.5 py-1 font-medium transition-colors ${
                sortKey === "surname"
                  ? "bg-gray-100 text-gray-800"
                  : "text-gray-500 hover:bg-gray-50"
              }`}
            >
              Surname{sortKey === "surname" ? (sortDir === "asc" ? " ↑" : " ↓") : ""}
            </button>
            <button
              onClick={() => setNameSort("firstname")}
              className={`px-2.5 py-1 font-medium transition-colors border-l ${
                sortKey === "firstname"
                  ? "bg-gray-100 text-gray-800"
                  : "text-gray-500 hover:bg-gray-50"
              }`}
            >
              First name{sortKey === "firstname" ? (sortDir === "asc" ? " ↑" : " ↓") : ""}
            </button>
          </div>
        </div>

        <div className="flex items-center gap-3 flex-wrap">
          <div className="flex items-center gap-1">
            <button
              onClick={handlePrevQuarter}
              className="p-1 rounded hover:bg-gray-100 text-gray-500 transition-colors"
              title="Previous quarter"
            >
              <ChevronLeft className="h-4 w-4" />
            </button>
            <span className="text-sm font-medium text-gray-700 min-w-[72px] text-center">
              {quarterLabel(quarterOffset)}
            </span>
            <button
              onClick={handleNextQuarter}
              className="p-1 rounded hover:bg-gray-100 text-gray-500 transition-colors"
              title="Next quarter"
            >
              <ChevronRight className="h-4 w-4" />
            </button>
            {quarterOffset !== 0 && (
              <button
                onClick={handleToday}
                className="ml-1 px-2 py-0.5 text-xs rounded border text-gray-500 border-gray-300 hover:bg-gray-50 transition-colors"
              >
                Today
              </button>
            )}
          </div>
          <div className="relative">
            <Search className="absolute left-2 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-gray-400 pointer-events-none" />
            <input
              type="text"
              placeholder="Filter people…"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              className="pl-7 pr-3 py-1 text-xs rounded border border-gray-300 focus:outline-none focus:ring-1 focus:ring-blue-500 w-36"
            />
          </div>
          <button
            onClick={handleExportCSV}
            disabled={!data}
            className="flex items-center gap-1.5 px-2.5 py-1 text-xs rounded border border-gray-300 text-gray-600 hover:bg-gray-50 disabled:opacity-40 transition-colors"
            title="Export as CSV"
          >
            <Download className="h-3.5 w-3.5" />
            CSV
          </button>
        </div>
      </div>

      {/* Legend */}
      <div className="flex items-center gap-3 text-xs flex-wrap shrink-0">
        {[
          { label: "0%", cls: "bg-gray-50 text-gray-400" },
          {
            label: `< ${Math.round(threshold * 0.9)}%`,
            cls: "bg-red-100 text-red-800",
          },
          {
            label: `${Math.round(threshold * 0.9)}–${threshold - 1}%`,
            cls: "bg-yellow-100 text-yellow-800",
          },
          { label: `${threshold}–100%`, cls: "bg-green-100 text-green-800" },
          { label: "> 100%", cls: "bg-blue-100 text-blue-800" },
        ].map(({ label, cls }) => (
          <span
            key={label}
            className={`inline-flex items-center rounded px-2 py-0.5 font-medium ${cls}`}
          >
            {label}
          </span>
        ))}
        <span className="text-gray-400 ml-2">Click a cell to edit</span>
      </div>

      {isLoading && <p className="text-sm text-gray-500">Loading…</p>}
      {error && (
        <p className="text-sm text-red-600">
          {error instanceof Error ? error.message : "Failed to load"}
        </p>
      )}

      {!isLoading && !error && (
        <div
          ref={containerRef}
          className="flex-1 min-h-0 overflow-auto rounded-lg border bg-white"
        >
          <table className="border-separate border-spacing-0 text-sm">
            <thead>
              {/* Month header row */}
              <tr>
                <th
                  rowSpan={2}
                  className="sticky left-0 z-30 bg-gray-50 border-b border-r px-3 py-2 text-left font-medium text-gray-600 min-w-[160px] whitespace-nowrap"
                >
                  Person{" "}
                  {search && (
                    <span className="text-gray-400 font-normal">
                      ({rows.length})
                    </span>
                  )}
                </th>
                <th
                  rowSpan={2}
                  onClick={toggleAvgSort}
                  className={`sticky left-[160px] z-30 border-b border-r px-3 py-2 text-center font-medium min-w-[56px] whitespace-nowrap cursor-pointer select-none hover:bg-gray-100 ${
                    sortKey === "avg4"
                      ? "bg-blue-50 text-blue-700"
                      : "bg-gray-50 text-gray-600"
                  }`}
                  title="Average utilization over next 4 weeks — click to sort"
                >
                  Avg 4W
                  {sortKey === "avg4" ? (sortDir === "asc" ? " ↑" : " ↓") : ""}
                </th>
                {monthGroups.map((mg) => (
                  <th
                    key={mg.key}
                    colSpan={mg.span}
                    className="border-b border-l px-2 py-1 text-center text-xs font-semibold text-gray-500 bg-gray-50 whitespace-nowrap"
                  >
                    {mg.label}
                  </th>
                ))}
              </tr>
              {/* Week header row */}
              <tr>
                {weeks.map((week) => (
                  <th
                    key={week}
                    data-week={week}
                    className={`border-b border-l px-1 py-1.5 text-center font-medium min-w-[52px] whitespace-nowrap ${
                      week === currentWeek
                        ? "bg-blue-100 text-blue-700"
                        : "bg-gray-50 text-gray-500"
                    }`}
                    title={formatWeekFull(week)}
                  >
                    <span className="block text-[10px] font-normal opacity-70">
                      {formatWeekDate(week)}
                    </span>
                    {formatWeekLabel(week)}
                  </th>
                ))}
              </tr>
            </thead>
            <tbody>
              {rows.length === 0 && (
                <tr>
                  <td
                    colSpan={weeks.length + 2}
                    className="px-4 py-8 text-center text-gray-500"
                  >
                    {search
                      ? `No people matching "${search}".`
                      : "No data found."}
                  </td>
                </tr>
              )}
              {rows.map((user, ri) => {
                const rowBg = ri % 2 === 1 ? "bg-gray-50" : "bg-white";
                const avg4 = avg4Map[user.alias];
                return (
                  <tr key={user.alias} className={rowBg}>
                    <td
                      className={`sticky left-0 z-10 ${rowBg} border-b border-r px-3 py-1.5 whitespace-nowrap`}
                    >
                      <div className="font-medium text-gray-800 text-xs">
                        {user.display_name}
                      </div>
                      {user.alias !== user.display_name && (
                        <div className="text-[10px] text-gray-400">
                          {user.alias} · {user.role}
                        </div>
                      )}
                      {user.alias === user.display_name && (
                        <div className="text-[10px] text-gray-400">
                          {user.role}
                        </div>
                      )}
                    </td>
                    <td
                      className={`sticky left-[160px] z-10 ${rowBg} border-b border-r px-2 py-1.5 text-center text-xs font-medium ${
                        avg4 !== null
                          ? fteCellClass(avg4, threshold)
                          : "text-gray-400"
                      }`}
                    >
                      {avg4 !== null ? `${avg4}%` : ""}
                    </td>
                    {weeks.map((week) => {
                      const slot = user.weekly_stats.find(
                        (s) => s.week === week
                      );
                      const fte = slot?.total_load ?? 0;
                      return (
                        <td
                          key={week}
                          className={`border-b border-l px-1 py-1 text-center text-xs cursor-pointer transition-opacity hover:opacity-80 ${fteCellClass(
                            fte,
                            threshold
                          )} ${
                            week === currentWeek
                              ? "ring-1 ring-inset ring-blue-400"
                              : ""
                          }`}
                          onClick={() =>
                            slot
                              ? openEdit(user, slot)
                              : openEdit(user, {
                                  week,
                                  total_load: 0,
                                  projects: [],
                                })
                          }
                          title={`${user.display_name} · ${formatWeekLabel(week)} · ${fte}%\nClick to edit`}
                        >
                          {fte > 0 ? `${fte}%` : ""}
                        </td>
                      );
                    })}
                  </tr>
                );
              })}

              {/* Average row */}
              {(data?.users?.length ?? 0) > 0 && (
                <tr className="border-t-2 border-gray-300 bg-gray-50 font-medium">
                  <td className="sticky left-0 z-10 bg-gray-50 border-r px-3 py-1.5 text-xs text-gray-600 whitespace-nowrap">
                    Average
                  </td>
                  <td className="sticky left-[160px] z-10 bg-gray-50 border-r px-2 py-1.5 text-center text-xs text-gray-400" />
                  {weeks.map((week) => {
                    const avg = averages[week] ?? 0;
                    return (
                      <td
                        key={week}
                        className={`border-l px-2 py-1.5 text-center text-xs ${fteCellClass(
                          Math.round(avg),
                          threshold
                        )} ${
                          week === currentWeek
                            ? "ring-1 ring-inset ring-blue-400"
                            : ""
                        }`}
                      >
                        {avg > 0 ? `${avg}%` : ""}
                      </td>
                    );
                  })}
                </tr>
              )}
            </tbody>
          </table>
        </div>
      )}

      <EditWeekDialog
        open={editOpen}
        onOpenChange={(open) => {
          setEditOpen(open);
          if (!open) {
            setEditingUser(null);
            setEditingSlot(null);
          }
        }}
        user={editingUser}
        slot={editingSlot}
        dashboardData={data ?? null}
      />
    </div>
  );
}
