import { useMemo, useState } from "react";
import { useQuery } from "@tanstack/react-query";
import {
  fetchDashboardData,
  buildProjectContextMap,
  extractProjects,
} from "@/api/dashboard";
import { useSettingsStore } from "@/store/settings.store";
import { Badge } from "@/components/ui/badge";
import { Search } from "lucide-react";
import { getCurrentISOWeek } from "@/modules/planning/utils";

type SortKey = "name" | "status" | "owner" | "people";

interface ProjectRow {
  name: string;
  status: string | null;
  owner: string | null;
  summary: string | null;
  peopleCount: number;
  currentPeople: string[];
}

export function ProjectsPage() {
  const { includePii } = useSettingsStore();
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

  const projects = useMemo<ProjectRow[]>(() => {
    if (!data) return [];
    const projectNames = extractProjects(data);
    const contextMap = buildProjectContextMap(data);

    // Count people per project (total unique aliases)
    const peopleMap: Record<string, Set<string>> = {};
    const currentPeopleMap: Record<string, Set<string>> = {};

    for (const user of data.users) {
      for (const stat of user.weekly_stats) {
        for (const proj of stat.projects) {
          if (!peopleMap[proj.project]) peopleMap[proj.project] = new Set();
          peopleMap[proj.project].add(user.alias);

          if (stat.week === currentWeek) {
            if (!currentPeopleMap[proj.project])
              currentPeopleMap[proj.project] = new Set();
            currentPeopleMap[proj.project].add(user.alias);
          }
        }
      }
    }

    const rows: ProjectRow[] = projectNames.map((name) => {
      const ctx = contextMap[name] ?? {};
      const currentAliases = [...(currentPeopleMap[name] ?? [])];
      // Map aliases to display names if PII is available
      const currentPeople = currentAliases.map((alias) => {
        const user = data.users.find((u) => u.alias === alias);
        return user?.display_name ?? alias;
      });

      return {
        name,
        status: ctx.status ?? null,
        owner: ctx.owner_alias ?? null,
        summary: ctx.summary ?? null,
        peopleCount: (peopleMap[name] ?? new Set()).size,
        currentPeople,
      };
    });

    rows.sort((a, b) => {
      let cmp = 0;
      if (sortKey === "name") cmp = a.name.localeCompare(b.name);
      else if (sortKey === "status")
        cmp = (a.status ?? "").localeCompare(b.status ?? "");
      else if (sortKey === "owner")
        cmp = (a.owner ?? "").localeCompare(b.owner ?? "");
      else if (sortKey === "people") cmp = a.peopleCount - b.peopleCount;
      return sortDir === "asc" ? cmp : -cmp;
    });

    if (!search.trim()) return rows;
    const q = search.toLowerCase();
    return rows.filter(
      (r) =>
        r.name.toLowerCase().includes(q) ||
        (r.owner ?? "").toLowerCase().includes(q) ||
        (r.summary ?? "").toLowerCase().includes(q)
    );
  }, [data, sortKey, sortDir, search, currentWeek]);

  const statusVariant = (
    status: string | null
  ): "success" | "secondary" | "destructive" | "outline" => {
    if (!status) return "outline";
    const s = status.toLowerCase();
    if (s === "active") return "success";
    if (s === "completed") return "secondary";
    if (s === "archived") return "destructive";
    return "outline";
  };

  return (
    <div className="p-6">
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center gap-3">
          <h2 className="text-xl font-semibold text-gray-900">Projects</h2>
          {projects.length > 0 && (
            <span className="text-sm text-gray-500">
              {projects.length} projects
            </span>
          )}
        </div>
        <div className="relative">
          <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-gray-400 pointer-events-none" />
          <input
            type="text"
            placeholder="Search projects…"
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
                  Project{si("name")}
                </th>
                <th
                  onClick={() => toggleSort("status")}
                  className="text-left px-4 py-3 font-medium text-gray-600 cursor-pointer select-none hover:bg-gray-100 whitespace-nowrap"
                >
                  Status{si("status")}
                </th>
                <th
                  onClick={() => toggleSort("owner")}
                  className="text-left px-4 py-3 font-medium text-gray-600 cursor-pointer select-none hover:bg-gray-100 whitespace-nowrap"
                >
                  Owner{si("owner")}
                </th>
                <th
                  onClick={() => toggleSort("people")}
                  className="text-left px-4 py-3 font-medium text-gray-600 cursor-pointer select-none hover:bg-gray-100 whitespace-nowrap"
                >
                  People{si("people")}
                </th>
                <th className="text-left px-4 py-3 font-medium text-gray-600 whitespace-nowrap">
                  This week
                </th>
                <th className="text-left px-4 py-3 font-medium text-gray-600">
                  Summary
                </th>
              </tr>
            </thead>
            <tbody>
              {projects.length === 0 && (
                <tr>
                  <td
                    colSpan={6}
                    className="px-4 py-8 text-center text-gray-500"
                  >
                    {search
                      ? `No projects matching "${search}".`
                      : "No data."}
                  </td>
                </tr>
              )}
              {projects.map((project) => (
                <tr
                  key={project.name}
                  className="border-b last:border-0 hover:bg-gray-50"
                >
                  <td className="px-4 py-3 font-medium text-gray-900">
                    {project.name}
                  </td>
                  <td className="px-4 py-3">
                    {project.status ? (
                      <Badge variant={statusVariant(project.status)}>
                        {project.status}
                      </Badge>
                    ) : (
                      <span className="text-gray-400 text-xs">—</span>
                    )}
                  </td>
                  <td className="px-4 py-3 text-gray-600 font-mono text-xs">
                    {project.owner ?? (
                      <span className="text-gray-400 font-sans">—</span>
                    )}
                  </td>
                  <td className="px-4 py-3 text-center text-gray-600">
                    {project.peopleCount}
                  </td>
                  <td className="px-4 py-3">
                    <div className="flex flex-wrap gap-1">
                      {project.currentPeople.map((p) => (
                        <Badge key={p} variant="secondary">
                          {p}
                        </Badge>
                      ))}
                      {project.currentPeople.length === 0 && (
                        <span className="text-gray-400 text-xs">—</span>
                      )}
                    </div>
                  </td>
                  <td className="px-4 py-3 text-gray-500 text-xs max-w-xs truncate">
                    {project.summary ?? ""}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
