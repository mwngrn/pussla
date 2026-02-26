// ── Types ─────────────────────────────────────────────────────────────────────

export interface ProjectContext {
  status?: string | null;
  owner_alias?: string | null;
  summary?: string | null;
}

export interface ProjectSlot {
  project: string;
  load: number;
  context: ProjectContext;
}

export interface WeeklyStats {
  week: string; // YYYY-Www
  total_load: number;
  projects: ProjectSlot[];
}

export interface DashboardUser {
  alias: string;
  real_name: string | null;
  display_name: string;
  role: string;
  weekly_stats: WeeklyStats[];
}

export interface DashboardMetrics {
  users_count: number;
  average_utilization: number;
  overbooked_slots: number;
}

export interface RawAllocation {
  alias: string;
  week: string;
  project: string;
  load: number;
}

export interface DashboardData {
  generated_at: string;
  weeks: string[]; // YYYY-Www
  users: DashboardUser[];
  metrics: DashboardMetrics;
  raw_allocations: RawAllocation[];
}

// ── Fetchers ──────────────────────────────────────────────────────────────────

export async function fetchDashboardData(
  includePii: boolean
): Promise<DashboardData> {
  const res = await fetch(
    `/api/dashboard-data?include_pii=${includePii ? "1" : "0"}`
  );
  if (!res.ok) {
    const body = await res.json().catch(() => ({ error: res.statusText }));
    throw new Error(
      (body as { error?: string }).error ?? `HTTP ${res.status}`
    );
  }
  return res.json() as Promise<DashboardData>;
}

export interface UpdateAllocationPayload {
  alias: string;
  week: string;
  allocations: Array<{ project: string; load: number }>;
}

export interface UpdateAllocationResult {
  ok: boolean;
  updated: {
    alias: string;
    week: string;
    projects_count: number;
    total_load: number;
  };
}

export async function updateAllocation(
  payload: UpdateAllocationPayload
): Promise<UpdateAllocationResult> {
  const res = await fetch("/api/allocation/update", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });
  const data = await res.json().catch(() => ({ error: "Invalid response" }));
  if (!res.ok) {
    throw new Error(
      (data as { error?: string }).error ?? `HTTP ${res.status}`
    );
  }
  return data as UpdateAllocationResult;
}

// ── Derived helpers ───────────────────────────────────────────────────────────

/** Extract unique project names from dashboard data. */
export function extractProjects(data: DashboardData): string[] {
  const names = new Set<string>();
  for (const alloc of data.raw_allocations) {
    if (alloc.project) names.add(alloc.project);
  }
  return [...names].sort((a, b) => a.localeCompare(b));
}

/** Build a map of project name → context from any user's weekly_stats. */
export function buildProjectContextMap(
  data: DashboardData
): Record<string, ProjectContext> {
  const map: Record<string, ProjectContext> = {};
  for (const user of data.users) {
    for (const week of user.weekly_stats) {
      for (const slot of week.projects) {
        if (slot.project && !map[slot.project]) {
          map[slot.project] = slot.context;
        }
      }
    }
  }
  return map;
}
