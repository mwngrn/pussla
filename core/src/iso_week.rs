//! ISO week helpers (ADR-004): parsing, normalization, and sort keys for
//! `"YYYY-Www"` strings. Shared by validation and aggregation so the two
//! never drift on what counts as a valid week.

use regex::Regex;
use std::sync::OnceLock;

fn pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^(\d{4})-W(0[1-9]|[1-4][0-9]|5[0-3])$").unwrap())
}

/// Parse `"YYYY-Www"` into `(year, week)`. Returns `None` for anything else,
/// including week 00 or week > 53.
pub fn parse(value: &str) -> Option<(u32, u32)> {
    let caps = pattern().captures(value)?;
    let year: u32 = caps[1].parse().ok()?;
    let week: u32 = caps[2].parse().ok()?;
    Some((year, week))
}

pub fn is_valid(value: &str) -> bool {
    parse(value).is_some()
}

/// Re-render to the canonical zero-padded form (parsing already enforces
/// the padded shape, so this is mostly a validating passthrough today, but
/// keeps the door open for lenient input later without breaking callers).
pub fn normalize(value: &str) -> Option<String> {
    let (year, week) = parse(value)?;
    Some(format!("{year:04}-W{week:02}"))
}

/// Sort key for chronological ordering; invalid weeks sort last, matching
/// `pussla_engine.py`'s `_week_sort_key` fallback of `(9999, 53)`.
pub fn sort_key(value: &str) -> (u32, u32) {
    parse(value).unwrap_or((9999, 53))
}

/// Maps an ISO week to the `"YYYY-MM"` calendar month containing that
/// week's Monday — port of `aggregate_planning_data.py`'s
/// `get_month_from_iso_week` (Python: `datetime.strptime(iso_week + "-1",
/// "%G-W%V-%u")`). Note ISO week 1 can fall in the *previous* December
/// (e.g. `2026-W01` -> `"2025-12"`, verified against Python) — this is the
/// ADR-004 "ISO Week 1 often spans two years" caveat made concrete. Returns
/// `None` for a syntactically valid-looking week that doesn't exist in that
/// ISO year (e.g. `"2025-W53"` — 2025 only has 52 ISO weeks), matching
/// Python raising `ValueError` there.
pub fn month_from_week(value: &str) -> Option<String> {
    let (year, week) = parse(value)?;
    let date = chrono::NaiveDate::from_isoywd_opt(year as i32, week, chrono::Weekday::Mon)?;
    use chrono::Datelike;
    Some(format!("{:04}-{:02}", date.year(), date.month()))
}

/// Maps an ISO week to a `"YYYY-Qn"` calendar quarter, via [`month_from_week`].
pub fn quarter_from_week(value: &str) -> Option<String> {
    let month_str = month_from_week(value)?;
    let (year, month) = month_str.split_once('-')?;
    let month: u32 = month.parse().ok()?;
    let quarter = (month - 1) / 3 + 1;
    Some(format!("{year}-Q{quarter}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_week() {
        assert_eq!(parse("2026-W09"), Some((2026, 9)));
        assert_eq!(parse("2026-W53"), Some((2026, 53)));
        assert_eq!(parse("2026-W01"), Some((2026, 1)));
    }

    #[test]
    fn rejects_invalid_week() {
        assert_eq!(parse("2026-W00"), None);
        assert_eq!(parse("2026-W54"), None);
        assert_eq!(parse("2026-9"), None);
        assert_eq!(parse("not-a-week"), None);
    }

    #[test]
    fn sort_key_pushes_invalid_last() {
        assert_eq!(sort_key("2026-W09"), (2026, 9));
        assert_eq!(sort_key("garbage"), (9999, 53));
    }

    #[test]
    fn month_from_week_matches_python_datetime_strptime_gwv() {
        // Verified against:
        // python3 -c "from datetime import datetime; print(datetime.strptime('<W>-1','%G-W%V-%u').strftime('%Y-%m'))"
        assert_eq!(month_from_week("2026-W01"), Some("2025-12".to_string()));
        assert_eq!(month_from_week("2026-W09"), Some("2026-02".to_string()));
        assert_eq!(month_from_week("2026-W52"), Some("2026-12".to_string()));
        assert_eq!(month_from_week("2026-W53"), Some("2026-12".to_string()));
    }

    #[test]
    fn month_from_week_none_for_week_that_does_not_exist_in_that_iso_year() {
        // 2025 has only 52 ISO weeks; Python's strptime raises ValueError here.
        assert_eq!(month_from_week("2025-W53"), None);
    }

    #[test]
    fn quarter_from_week_buckets_by_calendar_quarter() {
        assert_eq!(quarter_from_week("2026-W01"), Some("2025-Q4".to_string()));
        assert_eq!(quarter_from_week("2026-W09"), Some("2026-Q1".to_string()));
        assert_eq!(quarter_from_week("2026-W52"), Some("2026-Q4".to_string()));
    }
}
