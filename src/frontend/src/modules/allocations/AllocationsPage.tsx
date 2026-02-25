import { useMemo, useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { fetchDashboardData, DashboardUser, WeeklyStats } from "@/api/dashboard";
import { useSettingsStore } from "@/store/settings.store";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Search, Pencil } from "lucide-react";
import { EditWeekDialog } from "@/modules/planning/EditWeekDialog";
import {
  getCurrentISOWeek,
  formatWeekLabel,
  fteCellClass,
} from "@/modules/planning/utils";

export function AllocationsPage() {
  const { includePii, underAllocationThreshold: threshold } =
    useSettingsStore();
  const [search, setSearch] = useState("");
  const [expandedAlias, setExpandedAlias] = useState<string | null>(null);
  const [editingUser, setEditingUser] = useState<DashboardUser | null>(null);
  const [editingSlot, setEditingSlot] = useState<WeeklyStats | null>(null);
  const [editOpen, setEditOpen] = useState(false);

  const { data, isLoading, error } = useQuery({
    queryKey: ["dashboard", includePii],
    queryFn: () => fetchDashboardData(includePii),
  });

  const currentWeek = getCurrentISOWeek();

  const users = useMemo(() => {
    const arr = [...(data?.users ?? [])].sort((a, b) =>
      a.display_name.localeCompare(b.display_name)
    );
    if (!search.trim()) return arr;
    const q = search.toLowerCase();
    return arr.filter(
      (u) =>
        u.display_name.toLowerCase().includes(q) ||
        u.alias.toLowerCase().includes(q)
    );
  }, [data?.users, search]);

  const openEdit = (user: DashboardUser, slot: WeeklyStats) => {
    setEditingUser(user);
    setEditingSlot(slot);
    setEditOpen(true);
  };

  // Only show weeks with allocations + current week
  const relevantWeeks = useMemo(() => {
    if (!data) return [];
    return data.weeks.filter((w) => w >= currentWeek).slice(0, 26);
  }, [data, currentWeek]);

  return (
    <div className="p-6">
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center gap-3">
          <h2 className="text-xl font-semibold text-gray-900">Allocations</h2>
          <span className="text-xs text-gray-500 bg-gray-100 rounded px-2 py-0.5">
            Next 26 weeks
          </span>
        </div>
        <div className="relative">
          <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-gray-400 pointer-events-none" />
          <input
            type="text"
            placeholder="Search people…"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="pl-8 pr-3 py-1.5 text-sm rounded border border-gray-300 focus:outline-none focus:ring-1 focus:ring-blue-500 w-48"
          />
        </div>
      </div>

      {isLoading && <p className="text-sm text-gray-500">Loading…</p>}
      {error && (
        <p className="text-sm text-red-600">
          {error instanceof Error ? error.message : "Failed to load"}
        </p>
      )}

      {!isLoading && !error && (
        <div className="space-y-2">
          {users.length === 0 && (
            <p className="text-gray-500 text-sm py-8 text-center">
              {search ? `No people matching "${search}".` : "No data."}
            </p>
          )}
          {users.map((user) => {
            const isExpanded = expandedAlias === user.alias;
            const currentSlot = user.weekly_stats.find(
              (s) => s.week === currentWeek
            );
            const currentLoad = currentSlot?.total_load ?? 0;

            return (
              <div
                key={user.alias}
                className="rounded-lg border bg-white overflow-hidden"
              >
                {/* Person header */}
                <button
                  className="w-full flex items-center justify-between px-4 py-3 hover:bg-gray-50 transition-colors text-left"
                  onClick={() =>
                    setExpandedAlias(isExpanded ? null : user.alias)
                  }
                >
                  <div className="flex items-center gap-3">
                    <div>
                      <div className="font-medium text-gray-900 text-sm">
                        {user.display_name}
                      </div>
                      {user.alias !== user.display_name && (
                        <div className="text-xs text-gray-400">
                          {user.alias} · {user.role}
                        </div>
                      )}
                    </div>
                    {currentLoad > 0 && (
                      <span
                        className={`text-xs font-medium rounded px-2 py-0.5 ${fteCellClass(
                          currentLoad,
                          threshold
                        )}`}
                      >
                        {currentLoad}% this week
                      </span>
                    )}
                  </div>
                  <span className="text-gray-400 text-xs">
                    {isExpanded ? "▲" : "▼"}
                  </span>
                </button>

                {/* Expanded: allocation grid */}
                {isExpanded && (
                  <div className="border-t px-4 py-3">
                    <div className="overflow-x-auto">
                      <table className="text-xs border-separate border-spacing-0">
                        <thead>
                          <tr>
                            <th className="text-left px-2 py-1.5 font-medium text-gray-600 whitespace-nowrap">
                              Week
                            </th>
                            <th className="text-center px-2 py-1.5 font-medium text-gray-600 whitespace-nowrap">
                              Total
                            </th>
                            <th className="text-left px-2 py-1.5 font-medium text-gray-600">
                              Projects
                            </th>
                            <th className="px-2 py-1.5" />
                          </tr>
                        </thead>
                        <tbody>
                          {relevantWeeks.map((week) => {
                            const slot = user.weekly_stats.find(
                              (s) => s.week === week
                            );
                            const load = slot?.total_load ?? 0;
                            const isCurrent = week === currentWeek;
                            return (
                              <tr
                                key={week}
                                className={
                                  isCurrent ? "bg-blue-50/40" : "hover:bg-gray-50"
                                }
                              >
                                <td
                                  className={`px-2 py-1.5 font-medium whitespace-nowrap ${
                                    isCurrent
                                      ? "text-blue-700"
                                      : "text-gray-700"
                                  }`}
                                >
                                  {formatWeekLabel(week)}
                                  {isCurrent && (
                                    <span className="ml-1 text-[10px] text-blue-500">
                                      ←now
                                    </span>
                                  )}
                                </td>
                                <td className="px-2 py-1.5 text-center">
                                  {load > 0 ? (
                                    <span
                                      className={`rounded px-1.5 py-0.5 font-medium ${fteCellClass(
                                        load,
                                        threshold
                                      )}`}
                                    >
                                      {load}%
                                    </span>
                                  ) : (
                                    <span className="text-gray-300">—</span>
                                  )}
                                </td>
                                <td className="px-2 py-1.5">
                                  <div className="flex flex-wrap gap-1">
                                    {slot?.projects.map((p) => (
                                      <Badge key={p.project} variant="secondary">
                                        {p.project}{" "}
                                        <span className="opacity-60">
                                          {p.load}%
                                        </span>
                                      </Badge>
                                    ))}
                                    {(!slot || slot.projects.length === 0) && (
                                      <span className="text-gray-300">—</span>
                                    )}
                                  </div>
                                </td>
                                <td className="px-2 py-1.5 text-right">
                                  <Button
                                    variant="ghost"
                                    size="icon"
                                    className="h-6 w-6"
                                    onClick={() =>
                                      openEdit(
                                        user,
                                        slot ?? {
                                          week,
                                          total_load: 0,
                                          projects: [],
                                        }
                                      )
                                    }
                                    title="Edit allocation"
                                  >
                                    <Pencil className="h-3 w-3" />
                                  </Button>
                                </td>
                              </tr>
                            );
                          })}
                        </tbody>
                      </table>
                    </div>
                  </div>
                )}
              </div>
            );
          })}
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
