//! Dashboard data assembly — ported from `pussla_engine.py`'s
//! `build_dashboard_data`. Produces the exact JSON shape
//! `src/frontend/src/api/dashboard.ts` expects (`DashboardData` and
//! friends), so the frontend's TypeScript types and call sites don't need
//! to change when this replaces the Python backend (see task-45.6/45.7).
//!
//! Like [`crate::validation`], this works on raw `serde_yaml::Value` rather
//! than the strict typed models, to match the Python original's permissive
//! per-entry skipping of malformed data instead of failing the whole build.

use crate::iso_week;
use crate::rounding::round1;
use regex::Regex;
use serde::Serialize;
use serde_yaml::Value;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const DEFAULT_CAPACITY_HOURS: f64 = 40.0;

fn date_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap())
}

pub(crate) fn to_hours_from_load(load: i64, capacity_hours: f64) -> f64 {
    round1((load as f64 / 100.0) * capacity_hours)
}

pub(crate) fn to_load_from_hours(hours: f64, capacity_hours: f64) -> i64 {
    if capacity_hours <= 0.0 {
        return 0;
    }
    crate::rounding::python_round((hours / capacity_hours) * 100.0) as i64
}

fn parse_frontmatter_permissive(path: &Path) -> (Value, String) {
    let raw = fs::read_to_string(path).unwrap_or_default();
    match crate::frontmatter::split(&raw) {
        Ok(note) => {
            let value: Value =
                serde_yaml::from_str(note.frontmatter_yaml).unwrap_or(Value::Null);
            if matches!(value, Value::Mapping(_)) {
                (value, note.body.to_string())
            } else {
                (Value::Mapping(Default::default()), raw)
            }
        }
        Err(_) => (Value::Mapping(Default::default()), raw),
    }
}

fn sorted_md_files(dir: &Path) -> Vec<PathBuf> {
    if !dir.exists() {
        return vec![];
    }
    let mut files: Vec<PathBuf> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("md"))
        .collect();
    files.sort();
    files
}

fn get<'a>(v: &'a Value, key: &str) -> Option<&'a Value> {
    v.as_mapping()?.get(key)
}
fn get_str<'a>(v: &'a Value, key: &str) -> Option<&'a str> {
    get(v, key)?.as_str()
}
fn get_seq<'a>(v: &'a Value, key: &str) -> Option<&'a Vec<Value>> {
    get(v, key)?.as_sequence()
}

fn normalize_iso_date(v: &Value) -> Option<String> {
    let s = v.as_str()?;
    if date_re().is_match(s) {
        Some(s.to_string())
    } else {
        None
    }
}

// --- Output shapes, matching src/frontend/src/api/dashboard.ts exactly ---

#[derive(Debug, Clone, Serialize)]
pub struct Milestone {
    pub id: String,
    pub title: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Activity {
    pub id: String,
    pub label: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ProjectContext {
    pub project_id: Option<String>,
    pub status: Option<String>,
    pub owner_alias: Option<String>,
    pub start_week: Option<String>,
    pub end_week: Option<String>,
    pub start_week_override: Option<String>,
    pub end_week_override: Option<String>,
    pub hourly_rate: Option<f64>,
    pub milestones: Vec<Milestone>,
    pub activities: Vec<Activity>,
    pub summary: Option<String>,
    pub source_file: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectSlot {
    pub project: String,
    pub load: i64,
    pub planned_hours: f64,
    pub capacity_hours: f64,
    pub state: String,
    pub context: ProjectContext,
}

#[derive(Debug, Clone, Serialize)]
pub struct WeeklyStats {
    pub week: String,
    pub total_load: f64,
    pub total_planned_hours: f64,
    pub capacity_hours: f64,
    pub projects: Vec<ProjectSlot>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DashboardUser {
    pub alias: String,
    pub real_name: Option<String>,
    pub display_name: String,
    pub role: String,
    pub role_id: Option<String>,
    pub skills: Vec<String>,
    pub weekly_stats: Vec<WeeklyStats>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DashboardMetrics {
    pub users_count: usize,
    pub average_utilization: f64,
    pub overbooked_slots: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct RawAllocation {
    pub alias: String,
    pub week: String,
    pub project: String,
    pub load: i64,
    pub planned_hours: f64,
    pub capacity_hours: f64,
    pub state: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectEntry {
    pub name: String,
    #[serde(flatten)]
    pub context: ProjectContext,
    pub derived_start_week: Option<String>,
    pub derived_end_week: Option<String>,
    pub resolved_start_week: Option<String>,
    pub resolved_end_week: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DashboardData {
    pub generated_at: String,
    pub weeks: Vec<String>,
    pub users: Vec<DashboardUser>,
    pub projects: Vec<ProjectEntry>,
    pub metrics: DashboardMetrics,
    pub raw_allocations: Vec<RawAllocation>,
}

// --- Collection helpers (ported from pussla_engine.py) ---

fn collect_identities(identity_dir: &Path) -> HashMap<String, Option<String>> {
    let mut out = HashMap::new();
    for path in sorted_md_files(identity_dir) {
        let (fm, _) = parse_frontmatter_permissive(&path);
        let Some(alias) = get_str(&fm, "alias").filter(|s| !s.trim().is_empty()) else {
            continue;
        };
        let real_name = get_str(&fm, "real_name").map(String::from);
        out.insert(alias.to_string(), real_name);
    }
    out
}

fn collect_roles(roles_dir: &Path) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for path in sorted_md_files(roles_dir) {
        let (fm, _) = parse_frontmatter_permissive(&path);
        let Some(role_id) = get_str(&fm, "role_id").map(str::trim).filter(|s| !s.is_empty())
        else {
            continue;
        };
        let name = get_str(&fm, "name")
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or(role_id);
        out.insert(role_id.to_string(), name.to_string());
    }
    out
}

fn collect_project_context(projects_dir: &Path) -> BTreeMap<String, ProjectContext> {
    let mut out = BTreeMap::new();
    for path in sorted_md_files(projects_dir) {
        let (fm, body) = parse_frontmatter_permissive(&path);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();
        let name = get_str(&fm, "name")
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(String::from)
            .unwrap_or_else(|| stem.clone());

        let summary = body
            .lines()
            .map(str::trim)
            .find(|l| !l.is_empty())
            .unwrap_or("")
            .to_string();

        let mut milestones: Vec<(String, Milestone)> = get_seq(&fm, "milestones")
            .into_iter()
            .flatten()
            .enumerate()
            .filter_map(|(idx, ms)| {
                let title = get_str(ms, "title")?.trim();
                if title.is_empty() {
                    return None;
                }
                let date = normalize_iso_date(get(ms, "date")?)?;
                let id = get_str(ms, "id")
                    .map(String::from)
                    .unwrap_or_else(|| format!("ms-{}", idx + 1));
                Some((
                    date.clone(),
                    Milestone {
                        id,
                        title: title.to_string(),
                        date,
                    },
                ))
            })
            .collect();
        milestones.sort_by(|a, b| a.0.cmp(&b.0));
        let milestones: Vec<Milestone> = milestones.into_iter().map(|(_, m)| m).collect();

        let mut activities: Vec<Activity> = get_seq(&fm, "activities")
            .into_iter()
            .flatten()
            .enumerate()
            .filter_map(|(idx, act)| {
                let label = get_str(act, "label")?.trim();
                if label.is_empty() {
                    return None;
                }
                let start_date = normalize_iso_date(get(act, "start_date")?)?;
                let end_date = normalize_iso_date(get(act, "end_date")?)?;
                if start_date > end_date {
                    return None;
                }
                let id = get_str(act, "id")
                    .map(String::from)
                    .unwrap_or_else(|| format!("act-{}", idx + 1));
                Some(Activity {
                    id,
                    label: label.to_string(),
                    start_date,
                    end_date,
                })
            })
            .collect();
        activities.sort_by(|a, b| {
            (a.start_date.clone(), a.end_date.clone(), a.label.clone()).cmp(&(
                b.start_date.clone(),
                b.end_date.clone(),
                b.label.clone(),
            ))
        });

        out.insert(
            name,
            ProjectContext {
                project_id: get_str(&fm, "project_id").map(String::from).or_else(|| Some(stem.clone())),
                status: get_str(&fm, "status").map(String::from),
                owner_alias: get_str(&fm, "owner_alias").map(String::from),
                start_week: get_str(&fm, "start_week").map(String::from),
                end_week: get_str(&fm, "end_week").map(String::from),
                start_week_override: get_str(&fm, "start_week_override").map(String::from),
                end_week_override: get_str(&fm, "end_week_override").map(String::from),
                hourly_rate: get(&fm, "hourly_rate").and_then(Value::as_f64),
                milestones,
                activities,
                summary: Some(summary),
                source_file: Some(path.file_name().unwrap().to_string_lossy().to_string()),
            },
        );
    }
    out
}

#[derive(Default)]
struct WeekBucket {
    total_planned_hours: f64,
    capacity_hours: f64,
    projects: Vec<ProjectSlot>,
}

pub fn build_dashboard_data(
    planning_dir: &Path,
    identity_dir: &Path,
    include_pii: bool,
    generated_at: String,
) -> DashboardData {
    let people_dir = planning_dir.join("people");
    let roles_dir = planning_dir.join("roles");
    let projects_dir = planning_dir.join("projects");

    let identities = collect_identities(identity_dir);
    let roles = collect_roles(&roles_dir);
    let project_context = collect_project_context(&projects_dir);

    struct UserAcc {
        real_name: Option<String>,
        role: String,
        role_id: Option<String>,
        skills: Vec<String>,
        weekly: HashMap<String, WeekBucket>,
    }

    let mut users_by_alias: BTreeMap<String, UserAcc> = BTreeMap::new();
    let mut seen_weeks: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut raw_allocations: Vec<RawAllocation> = vec![];
    let mut project_week_bounds: HashMap<String, (String, String)> = HashMap::new();

    for path in sorted_md_files(&people_dir) {
        let (fm, _) = parse_frontmatter_permissive(&path);
        let Some(alias) = get_str(&fm, "alias").map(String::from) else {
            continue;
        };
        let Some(entries) = get_seq(&fm, "allocations") else {
            continue;
        };

        let identity_real_name = identities.get(&alias).cloned().flatten();
        let role_id = get_str(&fm, "role_id").map(String::from);
        let role_name = role_id.as_deref().and_then(|r| roles.get(r)).cloned();
        let role_value = role_name
            .or_else(|| role_id.clone().filter(|r| !r.trim().is_empty()))
            .unwrap_or_else(|| "Consultant".to_string());
        let skills: Vec<String> = get_seq(&fm, "skills")
            .into_iter()
            .flatten()
            .filter_map(|v| v.as_str().map(String::from))
            .collect();

        let user = users_by_alias.entry(alias.clone()).or_insert(UserAcc {
            real_name: if include_pii { identity_real_name } else { None },
            role: role_value,
            role_id,
            skills,
            weekly: HashMap::new(),
        });

        for entry in entries {
            let (Some(project), Some(weeks)) =
                (get_str(entry, "project"), get_seq(entry, "weeks"))
            else {
                continue;
            };
            let state_value = match get_str(entry, "state") {
                None => "committed".to_string(),
                Some(s) => {
                    let normalized = s.trim().to_lowercase();
                    if normalized == "tentative" || normalized == "committed" {
                        normalized
                    } else {
                        continue;
                    }
                }
            };
            let capacity = match get(entry, "capacity_hours") {
                None | Some(Value::Null) => DEFAULT_CAPACITY_HOURS,
                Some(v) => match v.as_f64() {
                    Some(c) => c,
                    None => continue,
                },
            };
            if capacity <= 0.0 {
                continue;
            }
            let (hours, load_value) = match get(entry, "planned_hours").and_then(Value::as_f64) {
                Some(h) => (h, to_load_from_hours(h, capacity)),
                None => match get(entry, "load").and_then(Value::as_i64) {
                    Some(load) => (to_hours_from_load(load, capacity), load),
                    None => continue,
                },
            };

            for week_v in weeks {
                let Some(week) = week_v.as_str() else { continue };
                let Some(normalized) = iso_week::normalize(week) else {
                    continue;
                };
                seen_weeks.insert(normalized.clone());

                let bounds = project_week_bounds
                    .entry(project.to_string())
                    .or_insert_with(|| (normalized.clone(), normalized.clone()));
                if normalized < bounds.0 {
                    bounds.0 = normalized.clone();
                }
                if normalized > bounds.1 {
                    bounds.1 = normalized.clone();
                }

                let bucket = user.weekly.entry(normalized.clone()).or_insert_with(|| {
                    WeekBucket {
                        total_planned_hours: 0.0,
                        capacity_hours: DEFAULT_CAPACITY_HOURS,
                        projects: vec![],
                    }
                });
                bucket.total_planned_hours += hours;
                if capacity > 0.0 {
                    bucket.capacity_hours = capacity;
                }
                let context = project_context.get(project).cloned().unwrap_or_default();
                bucket.projects.push(ProjectSlot {
                    project: project.to_string(),
                    load: load_value,
                    planned_hours: round1(hours),
                    capacity_hours: capacity,
                    state: state_value.clone(),
                    context,
                });

                raw_allocations.push(RawAllocation {
                    alias: alias.clone(),
                    week: normalized.clone(),
                    project: project.to_string(),
                    load: load_value,
                    planned_hours: round1(hours),
                    capacity_hours: capacity,
                    state: state_value.clone(),
                });
            }
        }
    }

    let mut sorted_weeks: Vec<String> = seen_weeks.into_iter().collect();
    sorted_weeks.sort_by_key(|w| iso_week::sort_key(w));

    let mut users: Vec<DashboardUser> = vec![];
    for (alias, acc) in &users_by_alias {
        let mut weekly_stats = vec![];
        for week in &sorted_weeks {
            let (total_planned_hours, capacity_hours, projects) = match acc.weekly.get(week) {
                Some(b) => (b.total_planned_hours, b.capacity_hours, b.projects.clone()),
                None => (0.0, DEFAULT_CAPACITY_HOURS, vec![]),
            };
            let total_load = if capacity_hours > 0.0 {
                round1((total_planned_hours / capacity_hours) * 100.0)
            } else {
                0.0
            };
            weekly_stats.push(WeeklyStats {
                week: week.clone(),
                total_load,
                total_planned_hours: round1(total_planned_hours),
                capacity_hours: round1(capacity_hours),
                projects,
            });
        }

        let display_name = acc
            .real_name
            .clone()
            .filter(|n| !n.trim().is_empty())
            .unwrap_or_else(|| alias.clone());

        users.push(DashboardUser {
            alias: alias.clone(),
            real_name: acc.real_name.clone(),
            display_name,
            role: acc.role.clone(),
            role_id: acc.role_id.clone(),
            skills: acc.skills.clone(),
            weekly_stats,
        });
    }

    let mut total_slots = 0usize;
    let mut total_load_points = 0.0f64;
    let mut overbooked_slots = 0usize;
    for user in &users {
        for slot in &user.weekly_stats {
            total_slots += 1;
            total_load_points += slot.total_load;
            if slot.total_load > 100.0 {
                overbooked_slots += 1;
            }
        }
    }
    let average_utilization = if total_slots > 0 {
        round1(total_load_points / total_slots as f64)
    } else {
        0.0
    };

    let mut projects: Vec<ProjectEntry> = vec![];
    for (name, ctx) in &project_context {
        let bounds = project_week_bounds.get(name);
        let derived_start = bounds.map(|b| b.0.clone());
        let derived_end = bounds.map(|b| b.1.clone());
        let resolved_start = ctx
            .start_week_override
            .clone()
            .or_else(|| derived_start.clone())
            .or_else(|| ctx.start_week.clone());
        let resolved_end = ctx
            .end_week_override
            .clone()
            .or_else(|| derived_end.clone())
            .or_else(|| ctx.end_week.clone());
        projects.push(ProjectEntry {
            name: name.clone(),
            context: ctx.clone(),
            derived_start_week: derived_start,
            derived_end_week: derived_end,
            resolved_start_week: resolved_start,
            resolved_end_week: resolved_end,
        });
    }

    DashboardData {
        generated_at,
        weeks: sorted_weeks,
        users,
        projects,
        metrics: DashboardMetrics {
            users_count: users_by_alias.len(),
            average_utilization,
            overbooked_slots,
        },
        raw_allocations,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_golden() -> serde_json::Value {
        let raw = fs::read_to_string("tests/fixtures/dashboard_reference.json").unwrap();
        let mut v: serde_json::Value = serde_json::from_str(&raw).unwrap();
        v.as_object_mut().unwrap().remove("generated_at");
        v
    }

    #[test]
    fn matches_python_reference_output() {
        let data = build_dashboard_data(
            Path::new("../tst-data/planning"),
            Path::new("../tst-data/identity"),
            true,
            "IGNORED".to_string(),
        );
        let mut actual = serde_json::to_value(&data).unwrap();
        actual.as_object_mut().unwrap().remove("generated_at");

        let expected = load_golden();
        assert_eq!(
            actual, expected,
            "dashboard output diverged from the Python reference (core/tests/fixtures/dashboard_reference.json)"
        );
    }

    #[test]
    fn respects_include_pii_false() {
        let data = build_dashboard_data(
            Path::new("../tst-data/planning"),
            Path::new("../tst-data/identity"),
            false,
            "x".to_string(),
        );
        for user in &data.users {
            assert!(user.real_name.is_none());
            assert_eq!(user.display_name, user.alias);
        }
    }
}
