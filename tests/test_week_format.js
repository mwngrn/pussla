const test = require('node:test');
const assert = require('node:assert/strict');
const { formatWeekLabel } = require('../src/dashboard/week_format');

test('formats canonical ISO week keys to Www', () => {
  assert.equal(formatWeekLabel('2026-W01'), 'W01');
  assert.equal(formatWeekLabel('2026-W52'), 'W52');
  assert.equal(formatWeekLabel('2020-W53'), 'W53');
});

test('leaves non-canonical values unchanged', () => {
  assert.equal(formatWeekLabel('W01'), 'W01');
  assert.equal(formatWeekLabel('2026-01'), '2026-01');
  assert.equal(formatWeekLabel(''), '');
});
