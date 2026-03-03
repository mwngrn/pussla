import test from "node:test";
import assert from "node:assert/strict";
import {
  buildActivityRowLayout,
  DEFAULT_WEEKLY_CAPACITY_HOURS,
  hoursToLoadPercent,
  loadPercentToHours,
  isoWeekKeyFromDate,
  mapActivitiesToWeeks,
  mapMilestonesToWeeks,
  normalizeDraggedWeekRange,
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

test("normalizeDraggedWeekRange returns sorted week bounds", () => {
  assert.deepEqual(normalizeDraggedWeekRange("2026-W09", "2026-W12"), {
    startWeek: "2026-W09",
    endWeek: "2026-W12",
  });
  assert.deepEqual(normalizeDraggedWeekRange("2026-W12", "2026-W09"), {
    startWeek: "2026-W09",
    endWeek: "2026-W12",
  });
});

test("mapActivitiesToWeeks creates week segments with start/end flags", () => {
  const activities = [
    {
      id: "a1",
      label: "Discovery",
      start_date: "2026-02-23", // W09
      end_date: "2026-03-08", // W10
    },
    {
      id: "a2",
      label: "Build",
      start_date: "2026-03-02", // W10
      end_date: "2026-03-15", // W11
    },
  ];
  const weeks = ["2026-W09", "2026-W10", "2026-W11"];

  const byWeek = mapActivitiesToWeeks(activities, weeks);
  assert.deepEqual(Object.keys(byWeek).sort(), ["2026-W09", "2026-W10", "2026-W11"]);
  assert.equal(byWeek["2026-W09"][0].id, "a1");
  assert.equal(byWeek["2026-W09"][0].isStart, true);
  assert.equal(byWeek["2026-W09"][0].isEnd, false);
  assert.deepEqual(
    byWeek["2026-W10"].map((a) => a.id),
    ["a1", "a2"]
  );
  assert.equal(byWeek["2026-W10"][0].isEnd, true);
  assert.equal(byWeek["2026-W10"][1].isStart, true);
  assert.equal(byWeek["2026-W11"][0].id, "a2");
  assert.equal(byWeek["2026-W11"][0].isEnd, true);
});

test("buildActivityRowLayout reserves one fixed row per activity across all weeks", () => {
  const activities = [
    {
      id: "a1",
      label: "Early",
      start_date: "2026-02-23", // W09
      end_date: "2026-03-08", // W10
    },
    {
      id: "a2",
      label: "Late",
      start_date: "2026-03-09", // W11
      end_date: "2026-03-15", // W11
    },
  ];
  const weeks = ["2026-W09", "2026-W10", "2026-W11"];

  const layout = buildActivityRowLayout(activities, weeks);
  assert.equal(layout.rows.length, 2);
  assert.equal(layout.byWeek["2026-W09"].length, 2);
  assert.equal(layout.byWeek["2026-W11"].length, 2);
  assert.equal(layout.byWeek["2026-W09"][0]?.id, "a1");
  assert.equal(layout.byWeek["2026-W09"][1], null);
  assert.equal(layout.byWeek["2026-W11"][0], null);
  assert.equal(layout.byWeek["2026-W11"][1]?.id, "a2");
});
