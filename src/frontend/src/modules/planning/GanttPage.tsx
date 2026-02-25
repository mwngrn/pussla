import React, { useMemo, useState, useCallback, useEffect, useRef } from "react";
import { useQuery } from "@tanstack/react-query";
import { fetchDashboardData, DashboardUser, WeeklyStats } from "@/api/dashboard";
import { useSettingsStore } from "@/store/settings.store";
import { ChevronLeft, ChevronRight, Search } from "lucide-react";
import { EditWeekDialog } from "./EditWeekDialog";
import {
  formatWeekLabel,
  formatWeekDate,
  formatWeekFull,
  buildCalendarGroups,
  projectColor,
  quarterLabel,
  getFirstWeekOfQuarter,
  getCurrentISOWeek,
} from "./utils";

export function GanttPage() {
  const { includePii } = useSettingsStore();
  const [quarterOffset, setQuarterOffset] = useState(0);
  const currentWeek = getCurrentISOWeek();
  const [compact, setCompact] = useState(true);
  const [sortBy, setSortBy] = useState<"surname" | "firstname">("surname");
  const [search, setSearch] = useState("");
  const containerRef = useRef<HTMLDivElement>(null);
  const initialScrolled = useRef(false);

  const [tooltip, setTooltip] = useState<{
    text: string;
    x: number;
    y: number;
  } | null>(null);
  const [editingUser, setEditingUser] = useState<DashboardUser | null>(null);
  const [editingSlot, setEditingSlot] = useState<WeeklyStats | null>(null);
  const [editOpen, setEditOpen] = useState(false);

  const { data, isLoading, error } = useQuery({
    queryKey: ["dashboard", includePii],
    queryFn: () => fetchDashboardData(includePii),
  });

  const weeks = data?.weeks ?? [];

  const scrollToWeek = useCallback(
    (targetWeek: string, behavior: ScrollBehavior = "smooth") => {
      const container = containerRef.current;
      if (!container || weeks.length === 0) return;
      const week = weeks.find((w) => w >= targetWeek) ?? weeks[0];
      const th = container.querySelector<HTMLElement>(
        `[data-week="${week}"]`
      );
      if (!th) return;
      const containerLeft = container.getBoundingClientRect().left;
      const thLeft = th.getBoundingClientRect().left;
      const newScrollLeft =
        container.scrollLeft + (thLeft - containerLeft) - 160;
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

  const sortRows = useCallback(
    (arr: DashboardUser[]) => {
      return [...arr].sort((a, b) => {
        const nameA =
          sortBy === "firstname"
            ? a.display_name.split(" ")[0]
            : a.display_name.split(" ").slice(-1)[0];
        const nameB =
          sortBy === "firstname"
            ? b.display_name.split(" ")[0]
            : b.display_name.split(" ").slice(-1)[0];
        return nameA.localeCompare(nameB);
      });
    },
    [sortBy]
  );

  const rows = useMemo(() => {
    const sorted = sortRows(data?.users ?? []);
    if (!search.trim()) return sorted;
    const q = search.toLowerCase();
    return sorted.filter(
      (r) =>
        r.display_name.toLowerCase().includes(q) ||
        r.alias.toLowerCase().includes(q)
    );
  }, [data?.users, sortRows, search]);

  const { monthGroups } = buildCalendarGroups(weeks);

  return (
    <div className="p-4 md:p-6 flex flex-col gap-4 h-full">
      {/* Header */}
      <div className="flex items-center justify-between flex-wrap gap-3 shrink-0">
        <div className="flex items-center gap-3">
          <h2 className="text-xl font-semibold text-gray-900">Gantt</h2>
          <button
            onClick={() => setCompact((c) => !c)}
            className={`px-2.5 py-1 rounded border text-xs font-medium transition-colors ${
              compact
                ? "bg-blue-50 text-blue-700 border-blue-300"
                : "text-gray-600 border-gray-300 hover:bg-gray-50"
            }`}
          >
            {compact ? "Normal view" : "Compact"}
          </button>
          <div className="flex rounded border overflow-hidden text-xs">
            <button
              onClick={() => setSortBy("surname")}
              className={`px-2.5 py-1 font-medium transition-colors ${
                sortBy === "surname"
                  ? "bg-gray-100 text-gray-800"
                  : "text-gray-500 hover:bg-gray-50"
              }`}
            >
              Surname
            </button>
            <button
              onClick={() => setSortBy("firstname")}
              className={`px-2.5 py-1 font-medium transition-colors border-l ${
                sortBy === "firstname"
                  ? "bg-gray-100 text-gray-800"
                  : "text-gray-500 hover:bg-gray-50"
              }`}
            >
              First name
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
        </div>
      </div>

      {/* Legend */}
      <div className="flex items-center gap-3 text-xs shrink-0 text-gray-500">
        <span>Color = project · Size = FTE% · Click a bar to edit</span>
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
              {/* Month header */}
              <tr>
                <th
                  rowSpan={2}
                  className="sticky left-0 z-20 bg-gray-50 border-b border-r px-3 py-2 text-left font-medium text-gray-600 min-w-[160px] whitespace-nowrap"
                >
                  Person{" "}
                  {search && (
                    <span className="text-gray-400 font-normal">
                      ({rows.length})
                    </span>
                  )}
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
              {/* Week header */}
              <tr>
                {weeks.map((week) => (
                  <th
                    key={week}
                    data-week={week}
                    className={`border-b border-l px-1 py-1.5 text-center font-medium whitespace-nowrap ${
                      compact ? "min-w-[44px]" : "min-w-[90px]"
                    } ${
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
                    colSpan={weeks.length + 1}
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
                          {user.alias}
                        </div>
                      )}
                    </td>
                    {weeks.map((week) => {
                      const slot = user.weekly_stats.find(
                        (s) => s.week === week
                      );
                      const isCurrentWeek = week === currentWeek;
                      return (
                        <td
                          key={week}
                          className={`border-b border-l px-1 py-1 relative ${
                            compact
                              ? "align-middle min-w-[44px]"
                              : "align-top min-w-[90px]"
                          } ${isCurrentWeek ? "bg-blue-50/40" : ""}`}
                        >
                          <div
                            className={
                              compact
                                ? "flex flex-wrap gap-0.5 justify-center p-0.5"
                                : ""
                            }
                          >
                            {slot?.projects.map((proj) => {
                              const color = projectColor(proj.project);
                              const tooltipText = `${proj.project}: ${proj.load}%${
                                proj.context?.summary
                                  ? ` — ${proj.context.summary}`
                                  : ""
                              }`;
                              const handleMouseEnter = (
                                e: React.MouseEvent<HTMLButtonElement>
                              ) => {
                                const rect =
                                  e.currentTarget.getBoundingClientRect();
                                setTooltip({
                                  text: tooltipText,
                                  x: rect.left + rect.width / 2,
                                  y: rect.top,
                                });
                              };
                              const handleClick = () => {
                                if (slot) {
                                  setEditingUser(user);
                                  setEditingSlot(slot);
                                  setEditOpen(true);
                                }
                              };
                              return compact ? (
                                <button
                                  key={proj.project}
                                  onClick={handleClick}
                                  title={tooltipText}
                                  onMouseEnter={handleMouseEnter}
                                  onMouseLeave={() => setTooltip(null)}
                                  className="w-2.5 h-2.5 rounded cursor-pointer hover:ring-2 ring-white ring-offset-1 transition-all"
                                  style={{ backgroundColor: color }}
                                />
                              ) : (
                                <button
                                  key={proj.project}
                                  onClick={handleClick}
                                  onMouseEnter={handleMouseEnter}
                                  onMouseLeave={() => setTooltip(null)}
                                  className="w-full text-left rounded px-1 py-0.5 mb-0.5 text-xs text-white truncate cursor-pointer hover:brightness-90 transition-all"
                                  style={{ backgroundColor: color }}
                                >
                                  {proj.project} {proj.load}%
                                </button>
                              );
                            })}
                          </div>
                        </td>
                      );
                    })}
                  </tr>
                );
              })}
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

      {tooltip && (
        <div
          className="fixed z-50 pointer-events-none"
          style={{
            left: tooltip.x,
            top: tooltip.y - 6,
            transform: "translateX(-50%) translateY(-100%)",
          }}
        >
          <div className="bg-gray-900 text-white text-xs px-2 py-1 rounded shadow-lg whitespace-nowrap max-w-xs">
            {tooltip.text}
          </div>
        </div>
      )}
    </div>
  );
}
