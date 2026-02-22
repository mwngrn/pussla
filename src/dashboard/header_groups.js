function parseIsoWeek(weekKey) {
  const match = /^(\d{4})-W(\d{2})$/.exec(String(weekKey || ""));
  if (!match) return null;
  return { year: Number(match[1]), week: Number(match[2]) };
}

function isoWeekStartDate(year, week) {
  const jan4 = new Date(Date.UTC(year, 0, 4));
  const jan4Day = jan4.getUTCDay() || 7;
  const mondayWeek1 = new Date(Date.UTC(year, 0, 4 - (jan4Day - 1)));
  return new Date(
    Date.UTC(
      mondayWeek1.getUTCFullYear(),
      mondayWeek1.getUTCMonth(),
      mondayWeek1.getUTCDate() + (week - 1) * 7
    )
  );
}

function buildCalendarGroups(weeks) {
  const monthNames = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
  const yearGroups = [];
  const monthGroups = [];

  for (const weekKey of weeks) {
    const parsed = parseIsoWeek(weekKey);
    if (!parsed) continue;
    const weekStart = isoWeekStartDate(parsed.year, parsed.week);
    const isoAnchor = new Date(
      Date.UTC(
        weekStart.getUTCFullYear(),
        weekStart.getUTCMonth(),
        weekStart.getUTCDate() + 3
      )
    );
    const yearLabel = String(parsed.year);
    const monthNumber = isoAnchor.getUTCMonth() + 1;
    const monthLabel = monthNames[isoAnchor.getUTCMonth()];
    const monthKey = `${yearLabel}-${String(monthNumber).padStart(2, "0")}`;

    const yearGroup = yearGroups[yearGroups.length - 1];
    if (yearGroup && yearGroup.label === yearLabel) {
      yearGroup.span += 1;
    } else {
      yearGroups.push({ label: yearLabel, span: 1 });
    }

    const monthGroup = monthGroups[monthGroups.length - 1];
    if (monthGroup && monthGroup.key === monthKey) {
      monthGroup.span += 1;
      monthGroup.weeks.push(weekKey);
    } else {
      monthGroups.push({ key: monthKey, label: monthLabel, span: 1, weeks: [weekKey] });
    }
  }

  return { yearGroups, monthGroups };
}

function calculateMonthAllocationPercentages({ weeks, users, monthGroups }) {
  const weekIndex = new Map();
  for (let i = 0; i < weeks.length; i += 1) {
    weekIndex.set(weeks[i], i);
  }

  return monthGroups.map((group) => {
    if (!users.length || !group.weeks.length) return 0;

    let totalLoad = 0;
    let slotCount = 0;
    for (const user of users) {
      const stats = Array.isArray(user.weekly_stats) ? user.weekly_stats : [];
      for (const weekKey of group.weeks) {
        const idx = weekIndex.get(weekKey);
        if (idx === undefined) continue;
        const slot = stats[idx];
        const load = slot && typeof slot.total_load === "number" ? slot.total_load : 0;
        totalLoad += load;
        slotCount += 1;
      }
    }

    if (!slotCount) return 0;
    return Math.round((totalLoad / slotCount) * 10) / 10;
  });
}

function calculateWeekAllocationPercentages({ weeks, users }) {
  return weeks.map((weekKey, idx) => {
    if (!users.length) return 0;
    let totalLoad = 0;
    let slotCount = 0;

    for (const user of users) {
      const stats = Array.isArray(user.weekly_stats) ? user.weekly_stats : [];
      const slot = stats[idx];
      if (!slot || slot.week !== weekKey) continue;
      const load = typeof slot.total_load === "number" ? slot.total_load : 0;
      totalLoad += load;
      slotCount += 1;
    }

    if (!slotCount) return 0;
    return Math.round((totalLoad / slotCount) * 10) / 10;
  });
}

if (typeof window !== "undefined") {
  window.buildCalendarGroups = buildCalendarGroups;
  window.calculateMonthAllocationPercentages = calculateMonthAllocationPercentages;
  window.calculateWeekAllocationPercentages = calculateWeekAllocationPercentages;
}

if (typeof module !== "undefined" && module.exports) {
  module.exports = {
    parseIsoWeek,
    isoWeekStartDate,
    buildCalendarGroups,
    calculateMonthAllocationPercentages,
    calculateWeekAllocationPercentages,
  };
}
