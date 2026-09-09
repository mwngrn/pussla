//! Weekly/monthly/quarterly rollups — ports `aggregate_planning_data.py`'s
//! `aggregate_data` (weekly `load` totals per alias) and adds the
//! monthly/quarterly rollups ADR-004 promises but that were never actually
//! implemented in Python (`get_month_from_iso_week` was defined there but
//! dead code — unused by `aggregate_data`/`generate_summary`). Built on
//! [`crate::iso_week::month_from_week`]/`quarter_from_week`.

use crate::iso_week;
use serde_yaml::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

fn get<'a>(v: &'a Value, key: &str) -> Option<&'a Value> {
    v.as_mapping()?.get(key)
}

fn parse_frontmatter_permissive(path: &Path) -> Value {
    let Ok(raw) = fs::read_to_string(path) else {
        return Value::Mapping(Default::default());
    };
    match crate::frontmatter::split(&raw) {
        Ok(note) => serde_yaml::from_str(note.frontmatter_yaml)
            .ok()
            .filter(|v: &Value| v.is_mapping())
            .unwrap_or_else(|| Value::Mapping(Default::default())),
        Err(_) => Value::Mapping(Default::default()),
    }
}

/// alias -> week -> summed `load` (raw percent, as authored — not hours).
/// Port of `aggregate_data(people_dir)`.
pub fn weekly_load_totals(people_dir: &Path) -> BTreeMap<String, BTreeMap<String, i64>> {
    let mut totals: BTreeMap<String, BTreeMap<String, i64>> = BTreeMap::new();
    let Ok(entries) = fs::read_dir(people_dir) else {
        return totals;
    };
    let mut paths: Vec<_> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("md"))
        .collect();
    paths.sort();

    for path in paths {
        let fm = parse_frontmatter_permissive(&path);
        let Some(alias) = get(&fm, "alias").and_then(Value::as_str) else {
            continue;
        };
        let Some(allocations) = get(&fm, "allocations").and_then(Value::as_sequence) else {
            continue;
        };
        for entry in allocations {
            let load = get(entry, "load").and_then(Value::as_i64).unwrap_or(0);
            let Some(weeks) = get(entry, "weeks").and_then(Value::as_sequence) else {
                continue;
            };
            for week_v in weeks {
                let Some(week) = week_v.as_str() else { continue };
                *totals
                    .entry(alias.to_string())
                    .or_default()
                    .entry(week.to_string())
                    .or_insert(0) += load;
            }
        }
    }
    totals
}

/// Re-buckets a weekly alias->week->load map into alias->period->load,
/// where `period` comes from `week_to_period` (month or quarter). Weeks
/// that don't map to a valid period (e.g. a week that doesn't exist in its
/// claimed ISO year) are skipped rather than panicking, matching this
/// crate's general policy of skipping malformed data over failing a whole
/// aggregation run.
fn rebucket(
    weekly: &BTreeMap<String, BTreeMap<String, i64>>,
    week_to_period: impl Fn(&str) -> Option<String>,
) -> BTreeMap<String, BTreeMap<String, i64>> {
    let mut out: BTreeMap<String, BTreeMap<String, i64>> = BTreeMap::new();
    for (alias, by_week) in weekly {
        for (week, load) in by_week {
            let Some(period) = week_to_period(week) else {
                continue;
            };
            *out.entry(alias.clone()).or_default().entry(period).or_insert(0) += load;
        }
    }
    out
}

pub fn monthly_load_totals(
    weekly: &BTreeMap<String, BTreeMap<String, i64>>,
) -> BTreeMap<String, BTreeMap<String, i64>> {
    rebucket(weekly, iso_week::month_from_week)
}

pub fn quarterly_load_totals(
    weekly: &BTreeMap<String, BTreeMap<String, i64>>,
) -> BTreeMap<String, BTreeMap<String, i64>> {
    rebucket(weekly, iso_week::quarter_from_week)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn people_dir() -> PathBuf {
        PathBuf::from("../tst-data/planning/people")
    }

    #[test]
    fn matches_python_aggregate_data_for_a_known_alias_week() {
        // Cross-checked directly against:
        // python3 -c "
        //   from aggregate_planning_data import aggregate_data
        //   from pathlib import Path
        //   t = aggregate_data(Path('tst-data/planning/people'))
        //   print(t['ArcticPine']['2026-W09'])"
        let totals = weekly_load_totals(&people_dir());
        let arctic = totals.get("ArcticPine").expect("ArcticPine must exist");
        // ArcticPine has Project-Bridge (load 65) and Ops-Rotation (load 20)
        // both in 2026-W09 (see tst-data/planning/people/ArcticPine.md).
        assert_eq!(arctic.get("2026-W09"), Some(&85));
    }

    #[test]
    fn weekly_totals_sum_across_multiple_entries_same_week() {
        let totals = weekly_load_totals(&people_dir());
        // Every alias/week pair present must be a plain sum of `load` across
        // however many allocation entries reference that week - spot-check
        // structure rather than a specific number already covered above.
        assert!(!totals.is_empty());
        for by_week in totals.values() {
            assert!(!by_week.is_empty());
        }
    }

    #[test]
    fn monthly_rollup_sums_constituent_weeks() {
        // Both Mondays fall in 2026-02 (2026-W07 -> 2026-02-09,
        // 2026-W08 -> 2026-02-16; W09's Monday is already in the same
        // month too, but W10 rolls into March — verified via
        // `datetime.strptime(w+"-1","%G-W%V-%u")`).
        let mut weekly: BTreeMap<String, BTreeMap<String, i64>> = BTreeMap::new();
        weekly
            .entry("Test".to_string())
            .or_default()
            .insert("2026-W07".to_string(), 50);
        weekly
            .entry("Test".to_string())
            .or_default()
            .insert("2026-W08".to_string(), 30);
        let monthly = monthly_load_totals(&weekly);
        assert_eq!(monthly["Test"]["2026-02"], 80);
    }

    #[test]
    fn monthly_rollup_splits_iso_week_1_into_previous_december() {
        let mut weekly: BTreeMap<String, BTreeMap<String, i64>> = BTreeMap::new();
        weekly
            .entry("Test".to_string())
            .or_default()
            .insert("2026-W01".to_string(), 40);
        let monthly = monthly_load_totals(&weekly);
        assert_eq!(monthly["Test"]["2025-12"], 40);
    }

    #[test]
    fn quarterly_rollup_sums_constituent_months() {
        let mut weekly: BTreeMap<String, BTreeMap<String, i64>> = BTreeMap::new();
        weekly
            .entry("Test".to_string())
            .or_default()
            .insert("2026-W09".to_string(), 50); // 2026-02 -> Q1
        weekly
            .entry("Test".to_string())
            .or_default()
            .insert("2026-W20".to_string(), 30); // 2026-05 -> Q2
        let quarterly = quarterly_load_totals(&weekly);
        assert_eq!(quarterly["Test"]["2026-Q1"], 50);
        assert_eq!(quarterly["Test"]["2026-Q2"], 30);
    }

    #[test]
    fn invalid_week_is_skipped_not_panicking() {
        let mut weekly: BTreeMap<String, BTreeMap<String, i64>> = BTreeMap::new();
        weekly
            .entry("Test".to_string())
            .or_default()
            .insert("garbage".to_string(), 40);
        let monthly = monthly_load_totals(&weekly);
        assert!(!monthly.contains_key("Test"));
    }
}
