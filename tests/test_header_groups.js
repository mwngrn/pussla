const test = require('node:test');
const assert = require('node:assert/strict');
const {
  buildCalendarGroups,
  calculateMonthAllocationPercentages,
  calculateWeekAllocationPercentages,
} = require('../src/dashboard/header_groups');

test('buildCalendarGroups groups years and months across ISO year boundaries', () => {
  const weeks = ['2025-W52', '2026-W01', '2026-W02', '2026-W09'];
  const groups = buildCalendarGroups(weeks);

  assert.deepEqual(groups.yearGroups, [
    { label: '2025', span: 1 },
    { label: '2026', span: 3 },
  ]);

  assert.deepEqual(groups.monthGroups.map(g => ({ label: g.label, span: g.span })), [
    { label: 'Dec', span: 1 },
    { label: 'Jan', span: 2 },
    { label: 'Feb', span: 1 },
  ]);
});

test('calculateMonthAllocationPercentages computes averages per month group', () => {
  const weeks = ['2026-W04', '2026-W05', '2026-W09'];
  const monthGroups = buildCalendarGroups(weeks).monthGroups;
  const users = [
    {
      weekly_stats: [
        { week: '2026-W04', total_load: 100 },
        { week: '2026-W05', total_load: 0 },
        { week: '2026-W09', total_load: 50 },
      ],
    },
    {
      weekly_stats: [
        { week: '2026-W04', total_load: 100 },
        { week: '2026-W05', total_load: 100 },
        { week: '2026-W09', total_load: 50 },
      ],
    },
  ];

  const percents = calculateMonthAllocationPercentages({ weeks, users, monthGroups });
  assert.deepEqual(percents, [75, 50]);
});

test('calculateWeekAllocationPercentages computes averages per week column', () => {
  const weeks = ['2026-W04', '2026-W05', '2026-W09'];
  const users = [
    {
      weekly_stats: [
        { week: '2026-W04', total_load: 100 },
        { week: '2026-W05', total_load: 0 },
        { week: '2026-W09', total_load: 50 },
      ],
    },
    {
      weekly_stats: [
        { week: '2026-W04', total_load: 80 },
        { week: '2026-W05', total_load: 100 },
        { week: '2026-W09', total_load: 60 },
      ],
    },
  ];

  const percents = calculateWeekAllocationPercentages({ weeks, users });
  assert.deepEqual(percents, [90, 50, 55]);
});
