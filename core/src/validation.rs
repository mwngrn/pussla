//! Validation rules ported from `src/validate_planning_data.py`: schema
//! checks, over-allocation, cross-references, and PII-leak scanning.
//!
//! Deliberately works on raw parsed YAML (`serde_yaml::Value`) rather than
//! the strict typed models in [`crate::planning`]/[`crate::project`] —
//! matching the Python script's own style of accumulating per-field errors
//! instead of failing hard on the first bad field, which is what lets this
//! module report *every* problem in a file instead of just the first one.

use crate::frontmatter;
use crate::iso_week;
use regex::Regex;
use serde_yaml::Value;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const DEFAULT_CAPACITY_HOURS: f64 = 40.0;

fn role_id_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^[A-Za-z0-9]+(?:-[A-Za-z0-9]+)*$").unwrap())
}

fn email_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}").unwrap())
}

fn phone_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\+?\d[\d \t().-]{7,}\d").unwrap())
}

fn normalize_skill(value: &str) -> String {
    value.trim().to_lowercase()
}

fn sorted_md_files(dir: &Path) -> Vec<PathBuf> {
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

fn parse_frontmatter_or_error(path: &Path) -> Result<(Value, String), String> {
    let raw = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let note = frontmatter::split(&raw).map_err(|e| e.to_string())?;
    let value: Value =
        serde_yaml::from_str(note.frontmatter_yaml).map_err(|e| e.to_string())?;
    if !matches!(value, Value::Mapping(_)) {
        return Err("frontmatter must parse to a YAML object".to_string());
    }
    Ok((value, note.body.to_string()))
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

pub fn load_real_names(identity_dir: &Path) -> Vec<String> {
    if !identity_dir.exists() {
        return vec![];
    }
    sorted_md_files(identity_dir)
        .into_iter()
        .filter_map(|path| parse_frontmatter_or_error(&path).ok())
        .filter_map(|(fm, _)| get_str(&fm, "real_name").map(|s| s.trim().to_string()))
        .filter(|s| !s.is_empty())
        .collect()
}

pub struct RoleValidation {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub role_names: HashMap<String, String>,
}

pub fn validate_roles(roles_dir: &Path) -> RoleValidation {
    let mut errors = vec![];
    let mut warnings = vec![];
    let mut role_names: HashMap<String, String> = HashMap::new();
    const REQUIRED: &[&str] = &["role_id", "name"];

    for path in sorted_md_files(roles_dir) {
        let (fm, _body) = match parse_frontmatter_or_error(&path) {
            Ok(v) => v,
            Err(e) => {
                errors.push(format!("{}: {e}", path.display()));
                continue;
            }
        };

        let mut missing: Vec<&str> = REQUIRED
            .iter()
            .filter(|k| get(&fm, k).is_none())
            .copied()
            .collect();
        missing.sort();
        if !missing.is_empty() {
            errors.push(format!(
                "{}: missing required frontmatter field(s): {}",
                path.display(),
                missing.join(", ")
            ));
            continue;
        }

        let role_id = get_str(&fm, "role_id").unwrap_or("").trim();
        if role_id.is_empty() {
            errors.push(format!("{}: 'role_id' must be a non-empty string", path.display()));
            continue;
        }
        if !role_id_re().is_match(role_id) {
            errors.push(format!(
                "{}: 'role_id' must match {}",
                path.display(),
                role_id_re().as_str()
            ));
            continue;
        }
        if role_names.contains_key(role_id) {
            errors.push(format!("{}: duplicate role_id '{role_id}'", path.display()));
            continue;
        }

        let name = get_str(&fm, "name").unwrap_or("").trim();
        if name.is_empty() {
            errors.push(format!("{}: 'name' must be a non-empty string", path.display()));
            continue;
        }

        if let Value::Mapping(map) = &fm {
            let mut extra: Vec<String> = map
                .keys()
                .filter_map(|k| k.as_str())
                .filter(|k| !REQUIRED.contains(k))
                .map(String::from)
                .collect();
            extra.sort();
            for key in extra {
                warnings.push(format!(
                    "{}: unknown role frontmatter field '{key}' (allowed, but ignored by schema)",
                    path.display()
                ));
            }
        }

        role_names.insert(role_id.to_string(), name.to_string());
    }

    RoleValidation {
        errors,
        warnings,
        role_names,
    }
}

pub struct SkillsValidation {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub canonical: HashSet<String>,
    pub synonyms: HashMap<String, String>,
}

pub fn validate_skills_catalog(skills_path: &Path) -> SkillsValidation {
    let mut errors = vec![];
    let mut warnings = vec![];
    let mut canonical: HashSet<String> = HashSet::new();
    let mut synonyms: HashMap<String, String> = HashMap::new();

    let (fm, _body) = match parse_frontmatter_or_error(skills_path) {
        Ok(v) => v,
        Err(e) => {
            errors.push(format!("{}: {e}", skills_path.display()));
            return SkillsValidation {
                errors,
                warnings,
                canonical,
                synonyms,
            };
        }
    };

    if get(&fm, "canonical_skills").is_none() {
        errors.push(format!(
            "{}: missing required frontmatter field(s): canonical_skills",
            skills_path.display()
        ));
        return SkillsValidation {
            errors,
            warnings,
            canonical,
            synonyms,
        };
    }

    let Some(skills_list) = get_seq(&fm, "canonical_skills") else {
        errors.push(format!(
            "{}: 'canonical_skills' must be a list",
            skills_path.display()
        ));
        return SkillsValidation {
            errors,
            warnings,
            canonical,
            synonyms,
        };
    };

    for (idx, skill) in skills_list.iter().enumerate() {
        match skill.as_str() {
            Some(s) if !s.trim().is_empty() => {
                canonical.insert(normalize_skill(s));
            }
            _ => errors.push(format!(
                "{}: canonical_skills[{idx}] must be a non-empty string",
                skills_path.display()
            )),
        }
    }

    if let Some(raw_synonyms) = get(&fm, "synonyms") {
        if !raw_synonyms.is_null() {
            let Some(map) = raw_synonyms.as_mapping() else {
                errors.push(format!(
                    "{}: 'synonyms' must be an object/map when provided",
                    skills_path.display()
                ));
                return SkillsValidation {
                    errors,
                    warnings,
                    canonical,
                    synonyms,
                };
            };
            for (raw_key, raw_target) in map {
                let (Some(k), Some(t)) = (raw_key.as_str(), raw_target.as_str()) else {
                    errors.push(format!(
                        "{}: synonym entries must be non-empty strings",
                        skills_path.display()
                    ));
                    continue;
                };
                if k.trim().is_empty() || t.trim().is_empty() {
                    errors.push(format!(
                        "{}: synonym entries must be non-empty strings",
                        skills_path.display()
                    ));
                    continue;
                }
                let key = normalize_skill(k);
                let target = normalize_skill(t);
                if !canonical.is_empty() && !canonical.contains(&target) {
                    warnings.push(format!(
                        "{}: synonym '{k}' points to non-canonical target '{t}'",
                        skills_path.display()
                    ));
                }
                synonyms.insert(key, target);
            }
        }
    }

    SkillsValidation {
        errors,
        warnings,
        canonical,
        synonyms,
    }
}

pub struct PeopleValidation {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    /// alias -> week -> total planned hours
    pub totals: HashMap<String, HashMap<String, f64>>,
    pub referenced_projects: HashSet<String>,
    pub known_aliases: HashSet<String>,
}

#[allow(clippy::too_many_arguments)]
pub fn validate_people(
    people_dir: &Path,
    known_roles: &HashSet<String>,
    canonical_skills: &HashSet<String>,
    skill_synonyms: &HashMap<String, String>,
    fail_on_overallocation: bool,
) -> PeopleValidation {
    let mut errors = vec![];
    let mut warnings = vec![];
    let mut totals: HashMap<String, HashMap<String, f64>> = HashMap::new();
    let mut capacities: HashMap<String, HashMap<String, f64>> = HashMap::new();
    let mut referenced_projects: HashSet<String> = HashSet::new();
    let mut known_aliases: HashSet<String> = HashSet::new();

    for path in sorted_md_files(people_dir) {
        let (fm, _body) = match parse_frontmatter_or_error(&path) {
            Ok(v) => v,
            Err(e) => {
                errors.push(format!("{}: {e}", path.display()));
                continue;
            }
        };

        let alias = get_str(&fm, "alias").unwrap_or("").trim().to_string();
        if alias.is_empty() {
            errors.push(format!("{}: 'alias' must be a non-empty string", path.display()));
            continue;
        }
        if path.file_stem().and_then(|s| s.to_str()) != Some(alias.as_str()) {
            errors.push(format!(
                "{}: filename stem must match alias ('{alias}')",
                path.display()
            ));
        }
        known_aliases.insert(alias.clone());

        match get_str(&fm, "role_id").map(str::trim) {
            Some(r) if !r.is_empty() => {
                if !known_roles.is_empty() && !known_roles.contains(r) {
                    errors.push(format!(
                        "{}: unknown role_id '{r}' (not found in roles/)",
                        path.display()
                    ));
                }
            }
            _ => errors.push(format!("{}: 'role_id' must be a non-empty string", path.display())),
        }

        match get_seq(&fm, "skills") {
            None => errors.push(format!("{}: 'skills' must be a list", path.display())),
            Some(skills) => {
                for (idx, skill) in skills.iter().enumerate() {
                    match skill.as_str() {
                        Some(s) if !s.trim().is_empty() => {
                            let normalized = normalize_skill(s);
                            let canonical = skill_synonyms
                                .get(&normalized)
                                .cloned()
                                .unwrap_or(normalized);
                            if !canonical_skills.is_empty() && !canonical_skills.contains(&canonical) {
                                warnings.push(format!(
                                    "{}: unknown skill '{s}' (allowed, but not in canonical_skills)",
                                    path.display()
                                ));
                            }
                        }
                        _ => errors.push(format!(
                            "{}: skills[{idx}] must be a non-empty string",
                            path.display()
                        )),
                    }
                }
            }
        }

        let Some(entries) = get_seq(&fm, "allocations") else {
            errors.push(format!("{}: 'allocations' must be a list", path.display()));
            continue;
        };

        for (idx, item) in entries.iter().enumerate() {
            let location = format!("{} allocations[{idx}]", path.display());
            if item.as_mapping().is_none() {
                errors.push(format!("{location}: entry must be an object"));
                continue;
            }

            let project = get_str(item, "project").unwrap_or("").trim().to_string();
            if project.is_empty() {
                errors.push(format!("{location}: 'project' must be a non-empty string"));
            } else {
                referenced_projects.insert(project.clone());
            }

            let weeks: Vec<String> = get_seq(item, "weeks")
                .map(|seq| {
                    seq.iter()
                        .filter_map(|w| w.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();
            if get_seq(item, "weeks").is_none() || weeks.is_empty() {
                errors.push(format!("{location}: 'weeks' must be a non-empty list"));
            }

            let capacity_hours_raw = get(item, "capacity_hours");
            let capacity = match capacity_hours_raw {
                None | Some(Value::Null) => DEFAULT_CAPACITY_HOURS,
                Some(v) => match v.as_f64() {
                    Some(c) => c,
                    None => {
                        errors.push(format!("{location}: 'capacity_hours' must be a number"));
                        continue;
                    }
                },
            };
            if capacity <= 0.0 {
                errors.push(format!("{location}: 'capacity_hours' must be greater than 0"));
                continue;
            }

            let planned_hours_raw = get(item, "planned_hours");
            let load_raw = get(item, "load");
            let hours: f64;

            match planned_hours_raw {
                Some(v) if !v.is_null() => {
                    let Some(h) = v.as_f64() else {
                        errors.push(format!("{location}: 'planned_hours' must be a number"));
                        continue;
                    };
                    if h < 0.0 {
                        errors.push(format!("{location}: 'planned_hours' must be >= 0"));
                        continue;
                    }
                    hours = h;

                    if let Some(load_v) = load_raw {
                        if !load_v.is_null() {
                            match load_v.as_i64() {
                                None => errors
                                    .push(format!("{location}: 'load' must be an integer when provided")),
                                Some(load) => {
                                    let expected_load =
                                        crate::rounding::python_round((hours / capacity) * 100.0);
                                    if (load as f64 - expected_load).abs() > 1.0 {
                                        errors.push(format!(
                                            "{location}: 'load' ({load}) is inconsistent with planned_hours/capacity_hours (expected about {expected_load})"
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
                _ => match load_raw.and_then(Value::as_i64) {
                    None => {
                        errors.push(format!(
                            "{location}: either 'planned_hours' or integer 'load' is required"
                        ));
                        continue;
                    }
                    Some(load) => {
                        if load < 0 {
                            errors.push(format!("{location}: 'load' must be >= 0"));
                            continue;
                        }
                        hours = (load as f64 / 100.0) * capacity;
                    }
                },
            }

            for week in &weeks {
                if !iso_week::is_valid(week) {
                    errors.push(format!(
                        "{location}: invalid ISO week '{week}' (expected YYYY-Www)"
                    ));
                    continue;
                }
                let alias_capacities = capacities.entry(alias.clone()).or_default();
                if let Some(existing) = alias_capacities.get(week) {
                    if (existing - capacity).abs() > 0.001 {
                        errors.push(format!(
                            "{location}: conflicting capacity_hours for alias '{alias}' week {week} ({existing} vs {capacity})"
                        ));
                        continue;
                    }
                }
                alias_capacities.insert(week.clone(), capacity);
                *totals
                    .entry(alias.clone())
                    .or_default()
                    .entry(week.clone())
                    .or_insert(0.0) += hours;
            }
        }
    }

    let mut alias_keys: Vec<&String> = totals.keys().collect();
    alias_keys.sort();
    for alias in alias_keys {
        let by_week = &totals[alias];
        let mut week_keys: Vec<&String> = by_week.keys().collect();
        week_keys.sort();
        for week in week_keys {
            let total_hours = by_week[week];
            let capacity = capacities
                .get(alias)
                .and_then(|m| m.get(week))
                .copied()
                .unwrap_or(DEFAULT_CAPACITY_HOURS);
            if total_hours > capacity + 1e-9 {
                let percent = if capacity > 0.0 {
                    crate::rounding::round1((total_hours / capacity) * 100.0)
                } else {
                    0.0
                };
                let message = format!(
                    "over-allocation: alias '{alias}' has total planned_hours {:.1} in week {week} (capacity {:.1}h, {percent:.1}%)",
                    crate::rounding::round1(total_hours),
                    crate::rounding::round1(capacity)
                );
                if fail_on_overallocation {
                    errors.push(message);
                } else {
                    warnings.push(message);
                }
            }
        }
    }

    PeopleValidation {
        errors,
        warnings,
        totals,
        referenced_projects,
        known_aliases,
    }
}

pub struct ProjectValidation {
    pub errors: Vec<String>,
    pub referenced_aliases: HashSet<String>,
}

pub fn validate_projects(projects_dir: &Path) -> ProjectValidation {
    let mut errors = vec![];
    let mut referenced_aliases: HashSet<String> = HashSet::new();
    const REQUIRED: &[&str] = &[
        "project_id",
        "name",
        "owner_alias",
        "start_week",
        "end_week",
        "status",
        "team_aliases",
    ];

    for path in sorted_md_files(projects_dir) {
        let (fm, body) = match parse_frontmatter_or_error(&path) {
            Ok(v) => v,
            Err(e) => {
                errors.push(format!("{}: {e}", path.display()));
                continue;
            }
        };

        let mut missing: Vec<&str> = REQUIRED
            .iter()
            .filter(|k| get(&fm, k).is_none())
            .copied()
            .collect();
        missing.sort();
        if !missing.is_empty() {
            errors.push(format!(
                "{}: missing required frontmatter field(s): {}",
                path.display(),
                missing.join(", ")
            ));
            continue;
        }

        if get_str(&fm, "project_id").unwrap_or("").trim().is_empty() {
            errors.push(format!("{}: 'project_id' must be a non-empty string", path.display()));
        }
        if get_str(&fm, "name").unwrap_or("").trim().is_empty() {
            errors.push(format!("{}: 'name' must be a non-empty string", path.display()));
        }

        match get_str(&fm, "owner_alias").map(str::trim) {
            Some(o) if !o.is_empty() => {
                referenced_aliases.insert(o.to_string());
            }
            _ => errors.push(format!("{}: 'owner_alias' must be a non-empty string", path.display())),
        }

        for field in ["start_week", "end_week"] {
            match get_str(&fm, field) {
                Some(v) if iso_week::is_valid(v) => {}
                _ => errors.push(format!(
                    "{}: '{field}' must be ISO week string (YYYY-Www)",
                    path.display()
                )),
            }
        }

        if get_str(&fm, "status").unwrap_or("").trim().is_empty() {
            errors.push(format!("{}: 'status' must be a non-empty string", path.display()));
        }

        match get_seq(&fm, "team_aliases") {
            None => errors.push(format!("{}: 'team_aliases' must be a list", path.display())),
            Some(team) => {
                for alias in team {
                    if let Some(a) = alias.as_str() {
                        referenced_aliases.insert(a.to_string());
                    }
                }
            }
        }

        if body.trim().is_empty() {
            errors.push(format!("{}: markdown body must not be empty", path.display()));
        }
    }

    ProjectValidation {
        errors,
        referenced_aliases,
    }
}

pub fn check_pii_leaks(public_files: &[PathBuf], real_names: &[String]) -> Vec<String> {
    let mut errors = vec![];
    let lowered_names: Vec<String> = real_names.iter().map(|n| n.to_lowercase()).collect();

    for path in public_files {
        let Ok(text) = fs::read_to_string(path) else {
            continue;
        };
        let lower = text.to_lowercase();

        for name in &lowered_names {
            if !name.is_empty() && lower.contains(name.as_str()) {
                errors.push(format!(
                    "{}: potential PII leak, contains identity real_name '{name}'",
                    path.display()
                ));
            }
        }
        if email_re().is_match(&text) {
            errors.push(format!(
                "{}: potential PII leak, contains email-like text",
                path.display()
            ));
        }
        if phone_re().is_match(&text) {
            errors.push(format!(
                "{}: potential PII leak, contains phone-like text",
                path.display()
            ));
        }
    }

    errors
}

/// Full dataset validation, mirroring `validate_planning_data.py main()`.
#[derive(serde::Serialize)]
pub struct DatasetValidation {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub people_count: usize,
    pub role_count: usize,
    pub project_count: usize,
}

pub fn validate_dataset(
    planning_dir: &Path,
    identity_dir: &Path,
    allow_overallocation: bool,
) -> DatasetValidation {
    let people_dir = planning_dir.join("people");
    let roles_dir = planning_dir.join("roles");
    let projects_dir = planning_dir.join("projects");
    let skills_path = planning_dir.join("skills.md");

    let roles = validate_roles(&roles_dir);
    let skills = validate_skills_catalog(&skills_path);
    let known_roles: HashSet<String> = roles.role_names.keys().cloned().collect();
    let people = validate_people(
        &people_dir,
        &known_roles,
        &skills.canonical,
        &skills.synonyms,
        !allow_overallocation,
    );
    let projects = validate_projects(&projects_dir);

    let known_projects: HashSet<String> = sorted_md_files(&projects_dir)
        .into_iter()
        .filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(String::from))
        .collect();

    let mut cross_errors = vec![];
    let mut ref_projects: Vec<&String> = people.referenced_projects.iter().collect();
    ref_projects.sort();
    for proj in ref_projects {
        if !known_projects.contains(proj) {
            cross_errors.push(format!(
                "cross-reference: project '{proj}' referenced in people but not found in projects/"
            ));
        }
    }
    let mut ref_aliases: Vec<&String> = projects.referenced_aliases.iter().collect();
    ref_aliases.sort();
    for alias in ref_aliases {
        if !people.known_aliases.contains(alias) {
            cross_errors.push(format!(
                "cross-reference: alias '{alias}' referenced in projects but not found in people/"
            ));
        }
    }

    let real_names = load_real_names(identity_dir);
    let mut public_files: Vec<PathBuf> = sorted_md_files(&people_dir);
    public_files.extend(sorted_md_files(&roles_dir));
    public_files.extend(sorted_md_files(&projects_dir));
    public_files.push(skills_path);
    let pii_errors = check_pii_leaks(&public_files, &real_names);

    let mut errors = vec![];
    errors.extend(roles.errors);
    errors.extend(skills.errors);
    errors.extend(people.errors);
    errors.extend(projects.errors);
    errors.extend(cross_errors);
    errors.extend(pii_errors);

    let mut warnings = vec![];
    warnings.extend(roles.warnings);
    warnings.extend(skills.warnings);
    warnings.extend(people.warnings);

    DatasetValidation {
        errors,
        warnings,
        people_count: people.known_aliases.len(),
        role_count: roles.role_names.len(),
        project_count: known_projects.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn planning_dir() -> PathBuf {
        PathBuf::from("../tst-data/planning")
    }

    fn identity_dir() -> PathBuf {
        PathBuf::from("../tst-data/identity")
    }

    #[test]
    fn matches_python_reference_error_count() {
        // tst-data/ genuinely contains over-allocated people (by design, to
        // exercise the dashboard's overbooked_slots metric — see
        // core/tests/fixtures/dashboard_reference.json's metrics.overbooked_slots:10)
        // and a few markdown bodies that trip the phone-number regex false
        // positive already known from identity-shield's own PII scanner.
        // Verified this is exactly what the Python validator reports too:
        // `python3 src/validate_planning_data.py --planning-dir tst-data/planning
        //  --identity-dir tst-data/identity` -> "Validation failed with 16 error(s)
        //  and 0 warning(s)." as of this port.
        let result = validate_dataset(&planning_dir(), &identity_dir(), false);
        assert_eq!(
            result.errors.len(),
            16,
            "error count diverged from the Python reference run: {:#?}",
            result.errors
        );
        assert!(result.errors.iter().any(|e| e.contains("over-allocation")));
        assert!(result.errors.iter().any(|e| e.contains("phone-like text")));
        assert!(result.people_count > 0);
        assert!(result.role_count > 0);
        assert!(result.project_count > 0);
    }

    #[test]
    fn validate_roles_rejects_duplicate_role_id() {
        let dir = tempdir();
        write(&dir, "a.md", "---\nrole_id: dup\nname: A\n---\nbody\n");
        write(&dir, "b.md", "---\nrole_id: dup\nname: B\n---\nbody\n");
        let result = validate_roles(dir.path());
        assert!(result.errors.iter().any(|e| e.contains("duplicate role_id")));
    }

    #[test]
    fn validate_roles_accepts_well_formed_entry() {
        let dir = tempdir();
        write(&dir, "a.md", "---\nrole_id: ok-role\nname: OK\n---\nbody\n");
        let result = validate_roles(dir.path());
        assert!(result.errors.is_empty());
        assert_eq!(result.role_names.get("ok-role"), Some(&"OK".to_string()));
    }

    #[test]
    fn validate_roles_rejects_bad_role_id_format() {
        let dir = tempdir();
        write(&dir, "a.md", "---\nrole_id: 'bad role!'\nname: A\n---\nbody\n");
        let result = validate_roles(dir.path());
        assert!(result.errors.iter().any(|e| e.contains("must match")));
    }

    #[test]
    fn validate_people_flags_overallocation() {
        let dir = tempdir();
        write(
            &dir,
            "Over.md",
            "---\nalias: Over\nrole_id: Consultant\nskills: []\nallocations:\n- project: X\n  weeks: [2026-W01]\n  planned_hours: 50.0\n  capacity_hours: 40.0\n  load: 125\n  state: committed\n---\nbody\n",
        );
        let result = validate_people(dir.path(), &HashSet::new(), &HashSet::new(), &HashMap::new(), true);
        assert!(result.errors.iter().any(|e| e.contains("over-allocation")));
    }

    #[test]
    fn validate_people_allows_overallocation_as_warning_when_not_failing() {
        let dir = tempdir();
        write(
            &dir,
            "Over.md",
            "---\nalias: Over\nrole_id: Consultant\nskills: []\nallocations:\n- project: X\n  weeks: [2026-W01]\n  planned_hours: 50.0\n  capacity_hours: 40.0\n  load: 125\n  state: committed\n---\nbody\n",
        );
        let result = validate_people(dir.path(), &HashSet::new(), &HashSet::new(), &HashMap::new(), false);
        assert!(result.errors.iter().all(|e| !e.contains("over-allocation")));
        assert!(result.warnings.iter().any(|e| e.contains("over-allocation")));
    }

    #[test]
    fn validate_people_accepts_legacy_load_only_row() {
        let dir = tempdir();
        write(
            &dir,
            "Legacy.md",
            "---\nalias: Legacy\nrole_id: Consultant\nskills: []\nallocations:\n- project: X\n  weeks: [2026-W01]\n  load: 50\n---\nbody\n",
        );
        let result = validate_people(dir.path(), &HashSet::new(), &HashSet::new(), &HashMap::new(), true);
        assert!(result.errors.is_empty(), "{:#?}", result.errors);
    }

    #[test]
    fn validate_people_rejects_invalid_iso_week() {
        let dir = tempdir();
        write(
            &dir,
            "Bad.md",
            "---\nalias: Bad\nrole_id: Consultant\nskills: []\nallocations:\n- project: X\n  weeks: [not-a-week]\n  load: 50\n---\nbody\n",
        );
        let result = validate_people(dir.path(), &HashSet::new(), &HashSet::new(), &HashMap::new(), true);
        assert!(result.errors.iter().any(|e| e.contains("invalid ISO week")));
    }

    #[test]
    fn validate_projects_rejects_empty_body() {
        let dir = tempdir();
        write(
            &dir,
            "P.md",
            "---\nproject_id: p\nname: P\nowner_alias: A\nstart_week: 2026-W01\nend_week: 2026-W02\nstatus: active\nteam_aliases: [A]\n---\n\n",
        );
        let result = validate_projects(dir.path());
        assert!(result.errors.iter().any(|e| e.contains("body must not be empty")));
    }

    #[test]
    fn check_pii_leaks_detects_real_name_and_email() {
        let dir = tempdir();
        write(&dir, "leak.md", "Contact Erik Andersson at erik@example.com.\n");
        let files = vec![dir.path().join("leak.md")];
        let errors = check_pii_leaks(&files, &["Erik Andersson".to_string()]);
        assert!(errors.iter().any(|e| e.contains("real_name")));
        assert!(errors.iter().any(|e| e.contains("email-like")));
    }

    #[test]
    fn check_pii_leaks_clean_file_has_no_errors() {
        let dir = tempdir();
        write(&dir, "clean.md", "Nothing sensitive here.\n");
        let files = vec![dir.path().join("clean.md")];
        let errors = check_pii_leaks(&files, &["Erik Andersson".to_string()]);
        assert!(errors.is_empty());
    }

    // --- tiny tempdir helper, no extra dev-dependency needed ---
    struct TempDir(PathBuf);
    impl TempDir {
        fn path(&self) -> &Path {
            &self.0
        }
    }
    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn tempdir() -> TempDir {
        let dir = std::env::temp_dir().join(format!(
            "pussla-core-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&dir).unwrap();
        TempDir(dir)
    }
    fn write(dir: &TempDir, name: &str, contents: &str) {
        fs::write(dir.path().join(name), contents).unwrap();
    }
}
