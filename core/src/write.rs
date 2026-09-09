//! Write-path mutators — ports `pussla_engine.py`'s `update_week_allocations`
//! and `update_project_metadata`. These are the two operations the desktop
//! app's edit UI needs (task-45.6's `save_allocation`/`save_project`
//! commands), as opposed to the read-only [`crate::dashboard`] path.
//!
//! Both follow the same shape as the Python originals: parse a note's
//! frontmatter into a mutable YAML mapping, apply the update, re-serialize
//! with keys in their original order (`serde_yaml::Mapping` preserves
//! insertion order, matching Python dict order + `yaml.safe_dump(...,
//! sort_keys=False)`), and write atomically (temp file in the same
//! directory, then rename) so a crash mid-write never corrupts the note.

use crate::dashboard::{to_hours_from_load, to_load_from_hours};
use crate::iso_week;
use serde_yaml::{Mapping, Value};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

const DEFAULT_CAPACITY_HOURS: f64 = 40.0;

#[derive(Debug, Error)]
pub enum WriteError {
    #[error("{0}")]
    InvalidInput(String),
    #[error("{0}")]
    NotFound(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("yaml error: {0}")]
    Yaml(#[from] serde_yaml::Error),
}

fn read_frontmatter_mapping(path: &Path) -> Result<(Mapping, String), WriteError> {
    let raw = fs::read_to_string(path)?;
    let note = crate::frontmatter::split(&raw)
        .map_err(|e| WriteError::InvalidInput(format!("failed to parse frontmatter: {e}")))?;
    let value: Value = serde_yaml::from_str(note.frontmatter_yaml)?;
    let mapping = value
        .as_mapping()
        .cloned()
        .ok_or_else(|| WriteError::InvalidInput("frontmatter must be a YAML object".to_string()))?;
    Ok((mapping, note.body.to_string()))
}

/// Atomic write: temp file in the same directory, then rename — matches
/// `NamedTemporaryFile(..., dir=path.parent) ...; temp_path.replace(path)`.
fn write_note_atomically(path: &Path, mapping: &Mapping, body: &str) -> Result<(), WriteError> {
    let yaml = serde_yaml::to_string(&Value::Mapping(mapping.clone()))?;
    let rendered = format!("---\n{}\n---\n{}", yaml.trim_end(), body);

    let parent = path.parent().ok_or_else(|| {
        WriteError::InvalidInput(format!("path has no parent directory: {}", path.display()))
    })?;
    fs::create_dir_all(parent)?;
    let tmp_path = parent.join(format!(
        ".{}.tmp-{}",
        path.file_name().and_then(|n| n.to_str()).unwrap_or("note"),
        std::process::id()
    ));
    fs::write(&tmp_path, rendered)?;
    fs::rename(&tmp_path, path)?;
    Ok(())
}

fn str_field(m: &Mapping, key: &str) -> Option<String> {
    m.get(key)?.as_str().map(String::from)
}

// --- update_week_allocations ---

#[derive(Debug, Clone)]
pub struct AllocationUpdate {
    pub project: String,
    /// Provide exactly one of `load` or `planned_hours`, matching the
    /// Python original's either/or contract.
    pub load: Option<i64>,
    pub planned_hours: Option<f64>,
    pub capacity_hours: Option<f64>,
    pub state: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateWeekAllocationsResult {
    pub alias: String,
    pub week: String,
    pub projects_count: usize,
    pub total_load: i64,
    pub total_planned_hours: f64,
    pub capacity_hours: f64,
}

struct MergedEntry {
    planned_hours: f64,
    capacity_hours: f64,
    load: i64,
    state: String,
}

pub fn update_week_allocations(
    planning_dir: &Path,
    alias: &str,
    week: &str,
    allocations: &[AllocationUpdate],
) -> Result<UpdateWeekAllocationsResult, WriteError> {
    if alias.trim().is_empty() {
        return Err(WriteError::InvalidInput(
            "alias must be a non-empty string".to_string(),
        ));
    }
    let normalized_week = iso_week::normalize(week)
        .ok_or_else(|| WriteError::InvalidInput("week must be in YYYY-Www format".to_string()))?;

    // project+state -> merged entry, insertion order preserved (Vec, not
    // HashMap) so file output stays deterministic run-to-run.
    let mut merged_order: Vec<(String, String)> = vec![];
    let mut merged: std::collections::HashMap<(String, String), MergedEntry> =
        std::collections::HashMap::new();

    for entry in allocations {
        let project = entry.project.trim();
        if project.is_empty() {
            return Err(WriteError::InvalidInput(
                "allocation project must be a non-empty string".to_string(),
            ));
        }
        let state_value = match entry.state.as_deref() {
            None => "committed".to_string(),
            Some(s) => {
                let normalized = s.trim().to_lowercase();
                if normalized == "tentative" || normalized == "committed" {
                    normalized
                } else {
                    return Err(WriteError::InvalidInput(
                        "allocation state must be 'tentative' or 'committed'".to_string(),
                    ));
                }
            }
        };
        let capacity = entry.capacity_hours.unwrap_or(DEFAULT_CAPACITY_HOURS);
        if capacity <= 0.0 {
            return Err(WriteError::InvalidInput(
                "allocation capacity_hours must be greater than 0".to_string(),
            ));
        }

        let (hours, load_value) = match entry.planned_hours {
            Some(h) => {
                if h < 0.0 {
                    return Err(WriteError::InvalidInput(
                        "allocation planned_hours cannot be negative".to_string(),
                    ));
                }
                (h, to_load_from_hours(h, capacity))
            }
            None => {
                let load = entry.load.ok_or_else(|| {
                    WriteError::InvalidInput("allocation load must be an integer".to_string())
                })?;
                if load < 0 {
                    return Err(WriteError::InvalidInput(
                        "allocation load cannot be negative".to_string(),
                    ));
                }
                (to_hours_from_load(load, capacity), load)
            }
        };

        let key = (project.to_string(), state_value.clone());
        match merged.get(&key) {
            None => {
                merged_order.push(key.clone());
                merged.insert(
                    key,
                    MergedEntry {
                        planned_hours: hours,
                        capacity_hours: capacity,
                        load: load_value,
                        state: state_value,
                    },
                );
            }
            Some(current) => {
                let total_hours = current.planned_hours + hours;
                let merged_capacity = current.capacity_hours;
                let merged_load = to_load_from_hours(total_hours, merged_capacity);
                merged.insert(
                    key,
                    MergedEntry {
                        planned_hours: crate::rounding::round1(total_hours),
                        capacity_hours: merged_capacity,
                        load: merged_load,
                        state: state_value,
                    },
                );
            }
        }
    }

    let normalized_entries: Vec<(String, MergedEntry)> = merged_order
        .into_iter()
        .filter_map(|key| merged.remove(&key).map(|v| (key.0, v)))
        .filter(|(_, v)| v.planned_hours > 0.0)
        .collect();

    let people_file = planning_dir.join("people").join(format!("{alias}.md"));
    if !people_file.exists() {
        return Err(WriteError::NotFound(format!(
            "people file not found for alias '{alias}'"
        )));
    }

    let (mut data, body) = read_frontmatter_mapping(&people_file)?;

    match str_field(&data, "alias") {
        Some(existing) if !existing.trim().is_empty() => {
            if existing != alias {
                return Err(WriteError::InvalidInput(format!(
                    "people file alias mismatch: expected '{alias}', found '{existing}'"
                )));
            }
        }
        _ => {
            data.insert(
                Value::String("alias".to_string()),
                Value::String(alias.to_string()),
            );
        }
    }

    let raw_allocations: Vec<Value> = data
        .get("allocations")
        .and_then(Value::as_sequence)
        .cloned()
        .unwrap_or_default();

    let mut rebuilt: Vec<Mapping> = vec![];
    for entry in &raw_allocations {
        let Some(entry_map) = entry.as_mapping() else {
            continue;
        };
        let (Some(project), Some(weeks)) = (
            str_field(entry_map, "project"),
            entry_map.get("weeks").and_then(Value::as_sequence),
        ) else {
            continue;
        };
        let state_value = match str_field(entry_map, "state") {
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
        let capacity = match entry_map.get("capacity_hours") {
            None | Some(Value::Null) => DEFAULT_CAPACITY_HOURS,
            Some(v) => match v.as_f64() {
                Some(c) => c,
                None => continue,
            },
        };
        if capacity <= 0.0 {
            continue;
        }
        let (hours, load_value) = match entry_map.get("planned_hours").and_then(Value::as_f64) {
            Some(h) => (h, to_load_from_hours(h, capacity)),
            None => match entry_map.get("load").and_then(Value::as_i64) {
                Some(load) => (to_hours_from_load(load, capacity), load),
                None => continue,
            },
        };

        let mut kept_weeks: Vec<String> = weeks
            .iter()
            .filter_map(|w| w.as_str())
            .filter_map(iso_week::normalize)
            .filter(|w| *w != normalized_week)
            .collect();
        if kept_weeks.is_empty() {
            continue;
        }
        kept_weeks.sort_by_key(|w| iso_week::sort_key(w));
        kept_weeks.dedup();

        let mut m = Mapping::new();
        m.insert(Value::String("project".into()), Value::String(project));
        m.insert(
            Value::String("weeks".into()),
            Value::Sequence(kept_weeks.into_iter().map(Value::String).collect()),
        );
        m.insert(
            Value::String("planned_hours".into()),
            serde_yaml::to_value(crate::rounding::round1(hours))?,
        );
        m.insert(
            Value::String("capacity_hours".into()),
            serde_yaml::to_value(capacity)?,
        );
        m.insert(Value::String("load".into()), Value::Number(load_value.into()));
        m.insert(Value::String("state".into()), Value::String(state_value));
        rebuilt.push(m);
    }

    for (project, entry) in &normalized_entries {
        let existing = rebuilt.iter_mut().find(|candidate| {
            str_field(candidate, "project").as_deref() == Some(project.as_str())
                && candidate.get("load").and_then(Value::as_i64) == Some(entry.load)
                && candidate.get("planned_hours").and_then(Value::as_f64)
                    == Some(entry.planned_hours)
                && candidate.get("capacity_hours").and_then(Value::as_f64)
                    == Some(entry.capacity_hours)
                && str_field(candidate, "state").as_deref() == Some(entry.state.as_str())
        });

        match existing {
            None => {
                let mut m = Mapping::new();
                m.insert(Value::String("project".into()), Value::String(project.clone()));
                m.insert(
                    Value::String("weeks".into()),
                    Value::Sequence(vec![Value::String(normalized_week.clone())]),
                );
                m.insert(
                    Value::String("planned_hours".into()),
                    serde_yaml::to_value(entry.planned_hours)?,
                );
                m.insert(
                    Value::String("capacity_hours".into()),
                    serde_yaml::to_value(entry.capacity_hours)?,
                );
                m.insert(Value::String("load".into()), Value::Number(entry.load.into()));
                m.insert(Value::String("state".into()), Value::String(entry.state.clone()));
                rebuilt.push(m);
            }
            Some(candidate) => {
                let mut weeks: Vec<String> = candidate
                    .get("weeks")
                    .and_then(Value::as_sequence)
                    .into_iter()
                    .flatten()
                    .filter_map(|w| w.as_str().map(String::from))
                    .collect();
                if !weeks.contains(&normalized_week) {
                    weeks.push(normalized_week.clone());
                }
                weeks.sort_by_key(|w| iso_week::sort_key(w));
                weeks.dedup();
                candidate.insert(
                    Value::String("weeks".into()),
                    Value::Sequence(weeks.into_iter().map(Value::String).collect()),
                );
            }
        }
    }

    data.insert(
        Value::String("allocations".to_string()),
        Value::Sequence(rebuilt.into_iter().map(Value::Mapping).collect()),
    );

    write_note_atomically(&people_file, &data, &body)?;

    let total_load: i64 = normalized_entries.iter().map(|(_, e)| e.load).sum();
    let total_planned_hours = crate::rounding::round1(
        normalized_entries.iter().map(|(_, e)| e.planned_hours).sum(),
    );

    Ok(UpdateWeekAllocationsResult {
        alias: alias.to_string(),
        week: normalized_week,
        projects_count: normalized_entries.len(),
        total_load,
        total_planned_hours,
        capacity_hours: DEFAULT_CAPACITY_HOURS,
    })
}

// --- update_project_metadata ---

#[derive(Debug, Clone)]
pub struct MilestoneUpdate {
    pub id: Option<String>,
    pub title: String,
    pub date: String,
}

#[derive(Debug, Clone)]
pub struct ActivityUpdate {
    pub id: Option<String>,
    pub label: String,
    pub start_date: String,
    pub end_date: String,
}

/// `None` on any field means "leave as-is"; `Some(None)` means "clear it"
/// (mirrors Python's `if key in updates` + `value is None` distinction).
#[derive(Debug, Clone, Default)]
pub struct ProjectMetadataUpdate {
    pub hourly_rate: Option<Option<f64>>,
    pub start_week_override: Option<Option<String>>,
    pub end_week_override: Option<Option<String>>,
    pub milestones: Option<Vec<MilestoneUpdate>>,
    pub activities: Option<Vec<ActivityUpdate>>,
}

#[derive(Debug, Clone)]
pub struct UpdateProjectMetadataResult {
    pub project: String,
    pub file: String,
}

fn is_date(s: &str) -> bool {
    s.len() == 10
        && s.as_bytes()[4] == b'-'
        && s.as_bytes()[7] == b'-'
        && s[..4].chars().all(|c| c.is_ascii_digit())
        && s[5..7].chars().all(|c| c.is_ascii_digit())
        && s[8..10].chars().all(|c| c.is_ascii_digit())
}

pub fn update_project_metadata(
    planning_dir: &Path,
    project: &str,
    updates: &ProjectMetadataUpdate,
) -> Result<UpdateProjectMetadataResult, WriteError> {
    if project.trim().is_empty() {
        return Err(WriteError::InvalidInput(
            "project must be a non-empty string".to_string(),
        ));
    }

    let projects_dir = planning_dir.join("projects");
    if !projects_dir.exists() {
        return Err(WriteError::NotFound(
            "projects directory not found".to_string(),
        ));
    }

    let mut target: Option<(PathBuf, Mapping, String)> = None;
    let mut entries: Vec<PathBuf> = fs::read_dir(&projects_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("md"))
        .collect();
    entries.sort();
    for path in entries {
        let Ok((fm, body)) = read_frontmatter_mapping(&path) else {
            continue;
        };
        let name = str_field(&fm, "name");
        let stem = path.file_stem().and_then(|s| s.to_str()).map(String::from);
        if name.as_deref() == Some(project) || stem.as_deref() == Some(project) {
            target = Some((path, fm, body));
            break;
        }
    }
    let (target_path, mut frontmatter, body) = target
        .ok_or_else(|| WriteError::NotFound(format!("project file not found for '{project}'")))?;

    if let Some(rate) = updates.hourly_rate {
        match rate {
            None => {
                frontmatter.remove("hourly_rate");
            }
            Some(r) if r >= 0.0 => {
                // Python: round(float(rate), 2). Not a banker's-rounding
                // tie risk in practice for currency-like rates, so plain
                // f64::round at 2 decimals is fine here (unlike the
                // load/hours math in dashboard.rs/validation.rs).
                let rounded = (r * 100.0).round() / 100.0;
                frontmatter.insert(
                    Value::String("hourly_rate".into()),
                    serde_yaml::to_value(rounded)?,
                );
            }
            Some(_) => {
                return Err(WriteError::InvalidInput(
                    "hourly_rate must be a non-negative number or null".to_string(),
                ))
            }
        }
    }

    for (field, value) in [
        ("start_week_override", updates.start_week_override.clone()),
        ("end_week_override", updates.end_week_override.clone()),
    ] {
        if let Some(v) = value {
            match v {
                None => {
                    frontmatter.remove(field);
                }
                Some(week) if !week.is_empty() => {
                    let normalized = iso_week::normalize(&week).ok_or_else(|| {
                        WriteError::InvalidInput(format!(
                            "{field} must be an ISO week (YYYY-Www) or null"
                        ))
                    })?;
                    frontmatter.insert(Value::String(field.into()), Value::String(normalized));
                }
                Some(_) => {
                    frontmatter.remove(field);
                }
            }
        }
    }

    if let Some(milestones) = &updates.milestones {
        let mut normalized: Vec<(String, Mapping)> = vec![];
        for (idx, ms) in milestones.iter().enumerate() {
            if ms.title.trim().is_empty() {
                return Err(WriteError::InvalidInput(
                    "milestone title must be a non-empty string".to_string(),
                ));
            }
            if !is_date(&ms.date) {
                return Err(WriteError::InvalidInput(
                    "milestone date must be in YYYY-MM-DD format".to_string(),
                ));
            }
            let id = ms.id.clone().unwrap_or_else(|| format!("ms-{}", idx + 1));
            let mut m = Mapping::new();
            m.insert(Value::String("id".into()), Value::String(id));
            m.insert(Value::String("title".into()), Value::String(ms.title.trim().to_string()));
            m.insert(Value::String("date".into()), Value::String(ms.date.clone()));
            normalized.push((ms.date.clone(), m));
        }
        normalized.sort_by(|a, b| a.0.cmp(&b.0));
        frontmatter.insert(
            Value::String("milestones".into()),
            Value::Sequence(normalized.into_iter().map(|(_, m)| Value::Mapping(m)).collect()),
        );
    }

    if let Some(activities) = &updates.activities {
        let mut normalized: Vec<Mapping> = vec![];
        for (idx, act) in activities.iter().enumerate() {
            if act.label.trim().is_empty() {
                return Err(WriteError::InvalidInput(
                    "activity label must be a non-empty string".to_string(),
                ));
            }
            if !is_date(&act.start_date) {
                return Err(WriteError::InvalidInput(
                    "activity start_date must be in YYYY-MM-DD format".to_string(),
                ));
            }
            if !is_date(&act.end_date) {
                return Err(WriteError::InvalidInput(
                    "activity end_date must be in YYYY-MM-DD format".to_string(),
                ));
            }
            if act.start_date > act.end_date {
                return Err(WriteError::InvalidInput(
                    "activity start_date must be on or before end_date".to_string(),
                ));
            }
            let id = act.id.clone().unwrap_or_else(|| format!("act-{}", idx + 1));
            let mut m = Mapping::new();
            m.insert(Value::String("id".into()), Value::String(id));
            m.insert(Value::String("label".into()), Value::String(act.label.trim().to_string()));
            m.insert(Value::String("start_date".into()), Value::String(act.start_date.clone()));
            m.insert(Value::String("end_date".into()), Value::String(act.end_date.clone()));
            normalized.push(m);
        }
        normalized.sort_by(|a, b| {
            let key = |m: &Mapping| {
                (
                    str_field(m, "start_date").unwrap_or_default(),
                    str_field(m, "end_date").unwrap_or_default(),
                    str_field(m, "label").unwrap_or_default(),
                )
            };
            key(a).cmp(&key(b))
        });
        frontmatter.insert(
            Value::String("activities".into()),
            Value::Sequence(normalized.into_iter().map(Value::Mapping).collect()),
        );
    }

    write_note_atomically(&target_path, &frontmatter, &body)?;

    Ok(UpdateProjectMetadataResult {
        project: project.to_string(),
        file: target_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string(),
    })
}
