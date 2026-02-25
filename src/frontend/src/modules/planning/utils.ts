// ── ISO week helpers ──────────────────────────────────────────────────────────

export function parseIsoWeek(
  weekKey: string
): { year: number; week: number } | null {
  const match = /^(\d{4})-W(\d{2})$/.exec(weekKey);
  if (!match) return null;
  return { year: Number(match[1]), week: Number(match[2]) };
}

/** Returns the Monday (UTC) that starts a given ISO week. */
export function isoWeekStartDate(year: number, week: number): Date {
  const jan4 = new Date(Date.UTC(year, 0, 4));
  const jan4Day = jan4.getUTCDay() || 7;
  const mondayWeek1 = new Date(
    Date.UTC(year, 0, 4 - (jan4Day - 1))
  );
  return new Date(
    Date.UTC(
      mondayWeek1.getUTCFullYear(),
      mondayWeek1.getUTCMonth(),
      mondayWeek1.getUTCDate() + (week - 1) * 7
    )
  );
}

/** Returns the current ISO week key, e.g. "2026-W09". */
export function getCurrentISOWeek(): string {
  const now = new Date();
  const d = new Date(
    Date.UTC(now.getFullYear(), now.getMonth(), now.getDate())
  );
  const dayNum = d.getUTCDay() || 7;
  d.setUTCDate(d.getUTCDate() + 4 - dayNum);
  const yearStart = new Date(Date.UTC(d.getUTCFullYear(), 0, 1));
  const weekNum = Math.ceil(
    ((d.getTime() - yearStart.getTime()) / 86400000 + 1) / 7
  );
  return `${d.getUTCFullYear()}-W${String(weekNum).padStart(2, "0")}`;
}

/** Short label for a week, e.g. "W09". */
export function formatWeekLabel(weekKey: string): string {
  const parsed = parseIsoWeek(weekKey);
  if (!parsed) return weekKey;
  return `W${String(parsed.week).padStart(2, "0")}`;
}

/** Short date of the Monday of a week, e.g. "24 Feb". */
export function formatWeekDate(weekKey: string): string {
  const parsed = parseIsoWeek(weekKey);
  if (!parsed) return weekKey;
  const d = isoWeekStartDate(parsed.year, parsed.week);
  return d.toLocaleDateString("en-GB", { day: "2-digit", month: "short" });
}

/** Full date range for a week tooltip, e.g. "24 Feb 2026". */
export function formatWeekFull(weekKey: string): string {
  const parsed = parseIsoWeek(weekKey);
  if (!parsed) return weekKey;
  const d = isoWeekStartDate(parsed.year, parsed.week);
  return d.toLocaleDateString("en-GB", {
    day: "2-digit",
    month: "short",
    year: "numeric",
  });
}

// ── Calendar grouping (year / month headers) ──────────────────────────────────

const MONTH_NAMES = [
  "Jan","Feb","Mar","Apr","May","Jun",
  "Jul","Aug","Sep","Oct","Nov","Dec",
];

export interface YearGroup {
  label: string;
  span: number;
}

export interface MonthGroup {
  key: string;
  label: string;
  span: number;
  weeks: string[];
}

export function buildCalendarGroups(weeks: string[]): {
  yearGroups: YearGroup[];
  monthGroups: MonthGroup[];
} {
  const yearGroups: YearGroup[] = [];
  const monthGroups: MonthGroup[] = [];

  for (const weekKey of weeks) {
    const parsed = parseIsoWeek(weekKey);
    if (!parsed) continue;
    const weekStart = isoWeekStartDate(parsed.year, parsed.week);
    // Use Thursday of the week to determine month/year bucket (ISO rule)
    const anchor = new Date(
      Date.UTC(
        weekStart.getUTCFullYear(),
        weekStart.getUTCMonth(),
        weekStart.getUTCDate() + 3
      )
    );
    const yearLabel = String(parsed.year);
    const monthLabel = MONTH_NAMES[anchor.getUTCMonth()];
    const monthKey = `${yearLabel}-${String(anchor.getUTCMonth() + 1).padStart(2, "0")}`;

    const lastYear = yearGroups[yearGroups.length - 1];
    if (lastYear && lastYear.label === yearLabel) {
      lastYear.span += 1;
    } else {
      yearGroups.push({ label: yearLabel, span: 1 });
    }

    const lastMonth = monthGroups[monthGroups.length - 1];
    if (lastMonth && lastMonth.key === monthKey) {
      lastMonth.span += 1;
      lastMonth.weeks.push(weekKey);
    } else {
      monthGroups.push({ key: monthKey, label: monthLabel, span: 1, weeks: [weekKey] });
    }
  }

  return { yearGroups, monthGroups };
}

// ── Quarter helpers ───────────────────────────────────────────────────────────

export function quarterLabel(offset = 0): string {
  const today = new Date();
  const totalQ = Math.floor(today.getMonth() / 3) + offset;
  const year = today.getFullYear() + Math.floor(totalQ / 4);
  const q = ((totalQ % 4) + 4) % 4;
  return `Q${q + 1} ${year}`;
}

/**
 * Returns the ISO week key of the first week of the quarter at offset.
 * offset 0 = current quarter.
 */
export function getFirstWeekOfQuarter(offset = 0): string {
  const today = new Date();
  const totalQ = Math.floor(today.getMonth() / 3) + offset;
  const year = today.getFullYear() + Math.floor(totalQ / 4);
  const q = ((totalQ % 4) + 4) % 4;
  const startMonth = q * 3;

  // Find the ISO week of the 1st of that month
  const d = new Date(Date.UTC(year, startMonth, 1));
  const dayNum = d.getUTCDay() || 7;
  // Move to the Thursday of that week to get the ISO week number
  const thursday = new Date(d);
  thursday.setUTCDate(d.getUTCDate() + 4 - dayNum);
  const yearStart = new Date(Date.UTC(thursday.getUTCFullYear(), 0, 1));
  const weekNum = Math.ceil(
    ((thursday.getTime() - yearStart.getTime()) / 86400000 + 1) / 7
  );
  const isoYear = thursday.getUTCFullYear();
  return `${isoYear}-W${String(weekNum).padStart(2, "0")}`;
}

// ── FTE colour coding ─────────────────────────────────────────────────────────

export function fteCellClass(fte: number, threshold = 80): string {
  if (fte === 0) return "bg-gray-50 text-gray-400";
  if (fte > 100) return "bg-blue-100 text-blue-800";
  if (fte >= threshold) return "bg-green-100 text-green-800";
  if (fte >= Math.round(threshold * 0.9)) return "bg-yellow-100 text-yellow-800";
  return "bg-red-100 text-red-800";
}

// ── Project colour palette ────────────────────────────────────────────────────

const PALETTE = [
  "#2563EB", "#16A34A", "#D97706", "#DC2626", "#7C3AED",
  "#0891B2", "#EA580C", "#65A30D", "#DB2777", "#0D9488",
  "#4F46E5", "#9333EA",
];

/** Deterministic, consistent colour per project name. */
export function projectColor(name: string): string {
  let hash = 0;
  for (let i = 0; i < name.length; i++) {
    hash = ((hash * 31) + name.charCodeAt(i)) >>> 0;
  }
  return PALETTE[hash % PALETTE.length];
}

// ── Week averages ─────────────────────────────────────────────────────────────

export function calculateWeekAverages(
  weeks: string[],
  users: Array<{ weekly_stats: Array<{ week: string; total_load: number }> }>
): Record<string, number> {
  const result: Record<string, number> = {};
  for (const week of weeks) {
    let total = 0;
    let count = 0;
    for (const user of users) {
      const slot = user.weekly_stats.find((s) => s.week === week);
      if (slot) {
        total += slot.total_load;
        count += 1;
      }
    }
    result[week] = count > 0 ? Math.round((total / count) * 10) / 10 : 0;
  }
  return result;
}
