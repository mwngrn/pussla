import { useMemo, useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { fetchDashboardData } from "@/api/dashboard";
import { useSettingsStore } from "@/store/settings.store";
import { Badge } from "@/components/ui/badge";
import { Search } from "lucide-react";
import { getCurrentISOWeek, fteCellClass } from "@/modules/planning/utils";

type SortKey = "name" | "alias" | "role" | "load";

export function PeoplePage() {
  const { includePii, underAllocationThreshold: threshold } =
    useSettingsStore();
  const [sortKey, setSortKey] = useState<SortKey>("name");
  const [sortDir, setSortDir] = useState<"asc" | "desc">("asc");
  const [search, setSearch] = useState("");

  const { data, isLoading, error } = useQuery({
    queryKey: ["dashboard", includePii],
    queryFn: () => fetchDashboardData(includePii),
  });

  const currentWeek = getCurrentISOWeek();

  const toggleSort = (key: SortKey) => {
    if (sortKey === key) setSortDir((d) => (d === "asc" ? "desc" : "asc"));
    else {
      setSortKey(key);
      setSortDir("asc");
    }
  };
  const si = (key: SortKey) =>
    sortKey === key ? (sortDir === "asc" ? " ↑" : " ↓") : "";

  const users = useMemo(() => {
    const arr = [...(data?.users ?? [])];

    arr.sort((a, b) => {
      let cmp = 0;
      if (sortKey === "name")
        cmp = a.display_name.localeCompare(b.display_name);
      else if (sortKey === "alias") cmp = a.alias.localeCompare(b.alias);
      else if (sortKey === "role") cmp = a.role.localeCompare(b.role);
      else if (sortKey === "load") {
        const aLoad =
          a.weekly_stats.find((s) => s.week === currentWeek)?.total_load ?? 0;
        const bLoad =
          b.weekly_stats.find((s) => s.week === currentWeek)?.total_load ?? 0;
        cmp = aLoad - bLoad;
      }
      return sortDir === "asc" ? cmp : -cmp;
    });

    if (!search.trim()) return arr;
    const q = search.toLowerCase();
    return arr.filter(
      (u) =>
        u.display_name.toLowerCase().includes(q) ||
        u.alias.toLowerCase().includes(q) ||
        u.role.toLowerCase().includes(q)
    );
  }, [data?.users, sortKey, sortDir, search, currentWeek]);

  // Compute current week projects for each user
  const currentWeekProjects = useMemo(() => {
    const map: Record<string, string[]> = {};
    for (const user of data?.users ?? []) {
      const slot = user.weekly_stats.find((s) => s.week === currentWeek);
      map[user.alias] = slot?.projects.map((p) => p.project) ?? [];
    }
    return map;
  }, [data?.users, currentWeek]);

  return (
    <div className="p-6">
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center gap-3">
          <h2 className="text-xl font-semibold text-gray-900">People</h2>
          {data && (
            <span className="text-sm text-gray-500">
              {data.metrics.users_count} people
            </span>
          )}
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
        <div className="rounded-lg border bg-white overflow-hidden">
          <table className="w-full text-sm">
            <thead>
              <tr className="border-b bg-gray-50">
                <th
                  onClick={() => toggleSort("name")}
                  className="text-left px-4 py-3 font-medium text-gray-600 cursor-pointer select-none hover:bg-gray-100 whitespace-nowrap"
                >
                  Name{si("name")}
                </th>
                <th
                  onClick={() => toggleSort("alias")}
                  className="text-left px-4 py-3 font-medium text-gray-600 cursor-pointer select-none hover:bg-gray-100 whitespace-nowrap"
                >
                  Alias{si("alias")}
                </th>
                <th
                  onClick={() => toggleSort("role")}
                  className="text-left px-4 py-3 font-medium text-gray-600 cursor-pointer select-none hover:bg-gray-100 whitespace-nowrap"
                >
                  Role{si("role")}
                </th>
                <th
                  onClick={() => toggleSort("load")}
                  className="text-left px-4 py-3 font-medium text-gray-600 cursor-pointer select-none hover:bg-gray-100 whitespace-nowrap"
                >
                  Current week{si("load")}
                </th>
                <th className="text-left px-4 py-3 font-medium text-gray-600 whitespace-nowrap">
                  Current projects
                </th>
              </tr>
            </thead>
            <tbody>
              {users.length === 0 && (
                <tr>
                  <td
                    colSpan={5}
                    className="px-4 py-8 text-center text-gray-500"
                  >
                    {search ? `No people matching "${search}".` : "No data."}
                  </td>
                </tr>
              )}
              {users.map((user) => {
                const slot = user.weekly_stats.find(
                  (s) => s.week === currentWeek
                );
                const load = slot?.total_load ?? 0;
                const projects = currentWeekProjects[user.alias] ?? [];
                return (
                  <tr
                    key={user.alias}
                    className="border-b last:border-0 hover:bg-gray-50"
                  >
                    <td className="px-4 py-3 font-medium text-gray-900">
                      {user.display_name}
                    </td>
                    <td className="px-4 py-3 text-gray-500 font-mono text-xs">
                      {user.alias}
                    </td>
                    <td className="px-4 py-3 text-gray-600">{user.role}</td>
                    <td className="px-4 py-3">
                      {load > 0 ? (
                        <span
                          className={`inline-flex items-center rounded px-2 py-0.5 text-xs font-medium ${fteCellClass(
                            load,
                            threshold
                          )}`}
                        >
                          {load}%
                        </span>
                      ) : (
                        <span className="text-gray-400 text-xs">—</span>
                      )}
                    </td>
                    <td className="px-4 py-3">
                      <div className="flex flex-wrap gap-1">
                        {projects.map((p) => (
                          <Badge key={p} variant="secondary">
                            {p}
                          </Badge>
                        ))}
                        {projects.length === 0 && (
                          <span className="text-gray-400 text-xs">—</span>
                        )}
                      </div>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
