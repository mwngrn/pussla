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

export function normalizeDraggedWeekRange(startWeek, endWeek) {
  if (!startWeek || !endWeek) return null;
  return startWeek <= endWeek
    ? { startWeek, endWeek }
    : { startWeek: endWeek, endWeek: startWeek };
}

export function mapActivitiesToWeeks(activities, weeks) {
  const map = {};
  const sorted = [...activities].sort(
    (a, b) =>
      a.start_date.localeCompare(b.start_date) ||
      a.end_date.localeCompare(b.end_date) ||
      a.label.localeCompare(b.label)
  );

  for (const activity of sorted) {
    const startWeek = isoWeekKeyFromDate(activity.start_date);
    const endWeek = isoWeekKeyFromDate(activity.end_date);
    if (!startWeek || !endWeek || endWeek < startWeek) continue;

    for (const week of weeks) {
      if (week < startWeek || week > endWeek) continue;
      if (!map[week]) map[week] = [];
      map[week].push({
        ...activity,
        isStart: week === startWeek,
        isEnd: week === endWeek,
      });
    }
  }

  return map;
}

export function buildActivityRowLayout(activities, weeks) {
  const rows = [...activities].sort(
    (a, b) =>
      a.start_date.localeCompare(b.start_date) ||
      a.end_date.localeCompare(b.end_date) ||
      a.label.localeCompare(b.label)
  );

  const byWeek = {};
  for (const week of weeks) {
    byWeek[week] = Array(rows.length).fill(null);
  }

  rows.forEach((activity, rowIndex) => {
    const startWeek = isoWeekKeyFromDate(activity.start_date);
    const endWeek = isoWeekKeyFromDate(activity.end_date);
    if (!startWeek || !endWeek || endWeek < startWeek) return;

    for (const week of weeks) {
      if (week < startWeek || week > endWeek) continue;
      byWeek[week][rowIndex] = {
        ...activity,
        isStart: week === startWeek,
        isEnd: week === endWeek,
        rowIndex,
      };
    }
  });

  return { rows, byWeek };
}
