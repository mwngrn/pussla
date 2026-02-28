export const DEFAULT_WEEKLY_CAPACITY_HOURS = 40;

export function loadPercentToHours(
  loadPercent,
  capacityHours = DEFAULT_WEEKLY_CAPACITY_HOURS
) {
  return Math.round((loadPercent / 100) * capacityHours * 10) / 10;
}

export function hoursToLoadPercent(
  hours,
  capacityHours = DEFAULT_WEEKLY_CAPACITY_HOURS
) {
  if (capacityHours <= 0) return 0;
  return Math.round((hours / capacityHours) * 100);
}

export function isoWeekKeyFromDate(dateISO) {
  if (!/^\d{4}-\d{2}-\d{2}$/.test(dateISO)) return null;
  const [y, m, d] = dateISO.split("-").map((v) => Number(v));
  if (!y || !m || !d) return null;
  const dt = new Date(Date.UTC(y, m - 1, d));
  const dayNum = dt.getUTCDay() || 7;
  dt.setUTCDate(dt.getUTCDate() + 4 - dayNum);
  const yearStart = new Date(Date.UTC(dt.getUTCFullYear(), 0, 1));
  const weekNum = Math.ceil(((dt.getTime() - yearStart.getTime()) / 86400000 + 1) / 7);
  return `${dt.getUTCFullYear()}-W${String(weekNum).padStart(2, "0")}`;
}

export function mapMilestonesToWeeks(milestones, weeks) {
  const map = {};
  const weekSet = new Set(weeks);
  for (const ms of milestones) {
    const week = isoWeekKeyFromDate(ms.date);
    if (!week || !weekSet.has(week)) continue;
    if (!map[week]) map[week] = [];
    map[week].push(ms);
  }
  for (const week of Object.keys(map)) {
    map[week].sort((a, b) => a.date.localeCompare(b.date) || a.title.localeCompare(b.title));
  }
  return map;
}
