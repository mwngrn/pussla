// ── Types ─────────────────────────────────────────────────────────────────────

export interface ProjectContext {
  project_id?: string | null;
  status?: string | null;
  owner_alias?: string | null;
  start_week?: string | null;
  end_week?: string | null;
  start_week_override?: string | null;
  end_week_override?: string | null;
  derived_start_week?: string | null;
  derived_end_week?: string | null;
  resolved_start_week?: string | null;
  resolved_end_week?: string | null;
  hourly_rate?: number | null;
  milestones?: Array<{ id: string; title: string; date: string }>;
  activities?: Array<{ id: string; label: string; start_date: string; end_date: string }>;
  summary?: string | null;
  source_file?: string | null;
}

export interface ProjectSlot {
  project: string;
  load: number;
  planned_hours?: number;
  capacity_hours?: number;
  state?: "tentative" | "committed";
  context: ProjectContext;
}

export interface WeeklyStats {
  week: string; // YYYY-Www
  total_load: number;
  total_planned_hours?: number;
  capacity_hours?: number;
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
  planned_hours?: number;
  capacity_hours?: number;
}

export interface DashboardData {
  generated_at: string;
  weeks: string[]; // YYYY-Www
  users: DashboardUser[];
  projects?: Array<{ name: string } & ProjectContext>;
  metrics: DashboardMetrics;
  raw_allocations: RawAllocation[];
}

export interface SyncChange {
  code: string;
  path: string;
}

export interface SyncStatus {
  repo_root: string;
  data_dir: string;
  data_scope: string;
  branch: string;
  head: string;
  last_commit_subject: string;
  publish_target: string | null;
  has_publish_target: boolean;
  auto_configured_target: boolean;
  scoped_dirty: boolean;
  repo_dirty: boolean;
  scoped_changes: SyncChange[];
  repo_change_count: number;
  ahead: number | null;
  behind: number | null;
}

export interface PublishResult {
  ok: boolean;
  commit_id: string;
  summary: string;
  publish_target: string;
  status: SyncStatus;
}

export interface RefreshResult {
  ok: boolean;
  updated: boolean;
  message: string;
  previous_head?: string;
  head?: string;
  status: SyncStatus;
}

// ── Fetchers ──────────────────────────────────────────────────────────────────
//
// Calls the Tauri desktop shell's commands (src-tauri/src/main.rs) directly —
// no local HTTP server. See docs/decisions/adr-011-rust-core-desktop-first-architecture.md.
// `invoke()` rejects with whatever string the Rust command's `Err` carries;
// normalized to an `Error` here so callers can keep using `.message` like
// they did with the previous fetch()-based errors.

import { invoke } from "@tauri-apps/api/core";

async function invokeOrThrow<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (err) {
    throw new Error(typeof err === "string" ? err : String(err));
  }
}

export async function fetchDashboardData(
  includePii: boolean
): Promise<DashboardData> {
  return invokeOrThrow<DashboardData>("get_dashboard_data", {
    include_pii: includePii,
  });
}

export async function fetchSyncStatus(): Promise<SyncStatus> {
  return invokeOrThrow<SyncStatus>("sync_status");
}

export async function publishPlanningChanges(summary: string): Promise<PublishResult> {
  return invokeOrThrow<PublishResult>("sync_publish", { summary });
}

export async function refreshPlanningData(): Promise<RefreshResult> {
  return invokeOrThrow<RefreshResult>("sync_refresh");
}

export interface UpdateAllocationPayload {
  alias: string;
  week: string;
  allocations: Array<{
    project: string;
    load?: number;
    planned_hours?: number;
    capacity_hours?: number;
  }>;
}

export interface UpdateAllocationResult {
  ok: boolean;
  updated: {
    alias: string;
    week: string;
    projects_count: number;
    total_load: number;
    total_planned_hours?: number;
    capacity_hours?: number;
  };
}

export interface UpdateProjectPayload {
  project: string;
  updates: {
    hourly_rate?: number | null;
    start_week_override?: string | null;
    end_week_override?: string | null;
    milestones?: Array<{ id?: string; title: string; date: string }>;
    activities?: Array<{ id?: string; label: string; start_date: string; end_date: string }>;
  };
}

export interface UpdateProjectResult {
  ok: boolean;
  updated: {
    project: string;
    file: string;
  };
}

export async function updateAllocation(
  payload: UpdateAllocationPayload
): Promise<UpdateAllocationResult> {
  return invokeOrThrow<UpdateAllocationResult>("update_allocation", { payload });
}

export async function updateProject(
  payload: UpdateProjectPayload
): Promise<UpdateProjectResult> {
  return invokeOrThrow<UpdateProjectResult>("update_project", { payload });
}

// ── Derived helpers ───────────────────────────────────────────────────────────

/** Extract unique project names from dashboard data. */
export function extractProjects(data: DashboardData): string[] {
  const names = new Set<string>();
  for (const p of data.projects ?? []) {
    if (p.name) names.add(p.name);
  }
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
  for (const project of data.projects ?? []) {
    if (project.name) {
      const { name, ...ctx } = project;
      map[name] = ctx;
    }
  }
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
