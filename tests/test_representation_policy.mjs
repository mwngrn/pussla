import test from "node:test";
import assert from "node:assert/strict";
import {
  DEFAULT_WEEKLY_CAPACITY_HOURS,
  hoursToLoadPercent,
  loadPercentToHours,
  isoWeekKeyFromDate,
  mapMilestonesToWeeks,
} from "../src/frontend/src/modules/project-management/representation_policy.js";

test("percent-hours conversion follows default 40h capacity", () => {
  assert.equal(DEFAULT_WEEKLY_CAPACITY_HOURS, 40);
  assert.equal(loadPercentToHours(50), 20);
  assert.equal(loadPercentToHours(25), 10);
  assert.equal(hoursToLoadPercent(20), 50);
  assert.equal(hoursToLoadPercent(10), 25);
});

test("percent-hours conversion supports custom capacity hours", () => {
  assert.equal(loadPercentToHours(50, 32), 16);
  assert.equal(hoursToLoadPercent(16, 32), 50);
});

test("isoWeekKeyFromDate resolves ISO week keys and rejects invalid input", () => {
  assert.equal(isoWeekKeyFromDate("2026-02-23"), "2026-W09");
  assert.equal(isoWeekKeyFromDate("2026-03-02"), "2026-W10");
  assert.equal(isoWeekKeyFromDate("2026-2-3"), null);
  assert.equal(isoWeekKeyFromDate("bad"), null);
});

test("mapMilestonesToWeeks groups milestones by displayed weeks and sorts within week", () => {
  const milestones = [
    { id: "m2", title: "Beta", date: "2026-02-25" }, // W09
    { id: "m1", title: "Alpha", date: "2026-02-24" }, // W09
    { id: "m3", title: "Gamma", date: "2026-03-03" }, // W10
    { id: "m4", title: "Out", date: "2026-12-01" }, // outside target weeks
  ];
  const weeks = ["2026-W09", "2026-W10"];

  const byWeek = mapMilestonesToWeeks(milestones, weeks);
  assert.deepEqual(Object.keys(byWeek).sort(), ["2026-W09", "2026-W10"]);
  assert.deepEqual(
    byWeek["2026-W09"].map((m) => m.id),
    ["m1", "m2"]
  );
  assert.deepEqual(
    byWeek["2026-W10"].map((m) => m.id),
    ["m3"]
  );
});
