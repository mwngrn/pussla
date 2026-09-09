//! Cross-checked against the actual Python originals (not just re-derived
//! logic). Reference run:
//!
//! ```text
//! python3 -c "
//! import sys; sys.path.insert(0,'src/dashboard')
//! from pussla_engine import update_week_allocations
//! import json
//! result = update_week_allocations('/tmp/write_test', 'ArcticPine', '2026-W20', [
//!     {'project': 'NewProj', 'load': 30, 'state': 'tentative'},
//!     {'project': 'NewProj', 'planned_hours': 10.0, 'state': 'tentative'},
//! ])
//! print(json.dumps(result))
//! "
//! ```
//! produced `{"alias": "ArcticPine", "week": "2026-W20", "projects_count": 1,
//! "total_load": 55, "total_planned_hours": 22.0, "capacity_hours": 40.0}`
//! and appended a `NewProj` allocation entry (weeks: [2026-W20],
//! planned_hours: 22.0, load: 55, state: tentative) — the two input rows for
//! the same project+state merge into one, hours-summed.

use pussla_core::write::{
    update_project_metadata, update_week_allocations, AllocationUpdate, MilestoneUpdate,
    ProjectMetadataUpdate,
};
use serde_yaml::Value;
use std::fs;
use std::path::{Path, PathBuf};

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
fn tempdir(name: &str) -> TempDir {
    let dir = std::env::temp_dir().join(format!(
        "pussla-core-write-test-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(dir.join("people")).unwrap();
    fs::create_dir_all(dir.join("projects")).unwrap();
    TempDir(dir)
}

fn read_frontmatter(path: &Path) -> Value {
    let raw = fs::read_to_string(path).unwrap();
    let note = pussla_core::frontmatter::split(&raw).unwrap();
    serde_yaml::from_str(note.frontmatter_yaml).unwrap()
}

#[test]
fn update_week_allocations_matches_python_reference() {
    let dir = tempdir("week-alloc");
    fs::copy(
        "../tst-data/planning/people/ArcticPine.md",
        dir.path().join("people/ArcticPine.md"),
    )
    .unwrap();

    let result = update_week_allocations(
        dir.path(),
        "ArcticPine",
        "2026-W20",
        &[
            AllocationUpdate {
                project: "NewProj".to_string(),
                load: Some(30),
                planned_hours: None,
                capacity_hours: None,
                state: Some("tentative".to_string()),
            },
            AllocationUpdate {
                project: "NewProj".to_string(),
                load: None,
                planned_hours: Some(10.0),
                capacity_hours: None,
                state: Some("tentative".to_string()),
            },
        ],
    )
    .unwrap();

    assert_eq!(result.alias, "ArcticPine");
    assert_eq!(result.week, "2026-W20");
    assert_eq!(result.projects_count, 1);
    assert_eq!(result.total_load, 55);
    assert_eq!(result.total_planned_hours, 22.0);
    assert_eq!(result.capacity_hours, 40.0);

    let fm = read_frontmatter(&dir.path().join("people/ArcticPine.md"));
    let allocations = fm.get("allocations").unwrap().as_sequence().unwrap();
    let new_entry = allocations
        .iter()
        .find(|e| e.get("project").and_then(Value::as_str) == Some("NewProj"))
        .expect("NewProj entry must exist");
    assert_eq!(new_entry.get("planned_hours").and_then(Value::as_f64), Some(22.0));
    assert_eq!(new_entry.get("load").and_then(Value::as_i64), Some(55));
    assert_eq!(
        new_entry.get("state").and_then(Value::as_str),
        Some("tentative")
    );
    let weeks: Vec<&str> = new_entry
        .get("weeks")
        .unwrap()
        .as_sequence()
        .unwrap()
        .iter()
        .map(|w| w.as_str().unwrap())
        .collect();
    assert_eq!(weeks, vec!["2026-W20"]);

    // Pre-existing allocations must survive untouched (7 original + 1 new).
    assert_eq!(allocations.len(), 7);
}

#[test]
fn update_week_allocations_removes_week_from_existing_entry_when_reassigned() {
    let dir = tempdir("week-reassign");
    fs::copy(
        "../tst-data/planning/people/ArcticPine.md",
        dir.path().join("people/ArcticPine.md"),
    )
    .unwrap();

    // 2026-W09 already belongs to Project-Bridge (committed) and
    // Ops-Rotation (committed). Reassign it entirely to a new project.
    update_week_allocations(
        dir.path(),
        "ArcticPine",
        "2026-W09",
        &[AllocationUpdate {
            project: "Reassigned".to_string(),
            load: Some(100),
            planned_hours: None,
            capacity_hours: None,
            state: None,
        }],
    )
    .unwrap();

    let fm = read_frontmatter(&dir.path().join("people/ArcticPine.md"));
    let allocations = fm.get("allocations").unwrap().as_sequence().unwrap();

    let bridge = allocations
        .iter()
        .find(|e| e.get("project").and_then(Value::as_str) == Some("Project-Bridge"))
        .unwrap();
    let bridge_weeks: Vec<&str> = bridge
        .get("weeks")
        .unwrap()
        .as_sequence()
        .unwrap()
        .iter()
        .map(|w| w.as_str().unwrap())
        .collect();
    assert!(!bridge_weeks.contains(&"2026-W09"), "2026-W09 must be removed from Project-Bridge");

    // Ops-Rotation had *only* 2026-W09 -> it becomes empty and must be dropped entirely.
    assert!(
        !allocations
            .iter()
            .any(|e| e.get("project").and_then(Value::as_str) == Some("Ops-Rotation")),
        "Ops-Rotation must be dropped once its only week is reassigned"
    );

    let reassigned = allocations
        .iter()
        .find(|e| e.get("project").and_then(Value::as_str) == Some("Reassigned"))
        .expect("Reassigned entry must exist");
    assert_eq!(reassigned.get("state").and_then(Value::as_str), Some("committed"));
}

#[test]
fn update_week_allocations_rejects_alias_mismatch() {
    let dir = tempdir("alias-mismatch");
    fs::copy(
        "../tst-data/planning/people/ArcticPine.md",
        dir.path().join("people/ArcticPine.md"),
    )
    .unwrap();
    let err = update_week_allocations(
        dir.path(),
        "SomeoneElse",
        "2026-W20",
        &[AllocationUpdate {
            project: "X".to_string(),
            load: Some(10),
            planned_hours: None,
            capacity_hours: None,
            state: None,
        }],
    );
    assert!(err.is_err());
}

#[test]
fn update_project_metadata_sets_hourly_rate_and_milestones() {
    let dir = tempdir("project-meta");
    fs::copy(
        "../tst-data/planning/projects/Internal-RnD.md",
        dir.path().join("projects/Internal-RnD.md"),
    )
    .unwrap();

    let updates = ProjectMetadataUpdate {
        hourly_rate: Some(Some(1234.567)),
        milestones: Some(vec![
            MilestoneUpdate {
                id: None,
                title: "Kickoff".to_string(),
                date: "2026-03-01".to_string(),
            },
            MilestoneUpdate {
                id: Some("ms-fixed".to_string()),
                title: "Launch".to_string(),
                date: "2026-01-15".to_string(),
            },
        ]),
        ..Default::default()
    };

    let result = update_project_metadata(dir.path(), "Internal-RnD", &updates).unwrap();
    assert_eq!(result.project, "Internal-RnD");
    assert_eq!(result.file, "Internal-RnD.md");

    let fm = read_frontmatter(&dir.path().join("projects/Internal-RnD.md"));
    assert_eq!(fm.get("hourly_rate").and_then(Value::as_f64), Some(1234.57));

    let milestones = fm.get("milestones").unwrap().as_sequence().unwrap();
    assert_eq!(milestones.len(), 2);
    // Sorted by date: Launch (2026-01-15) before Kickoff (2026-03-01).
    assert_eq!(
        milestones[0].get("title").and_then(Value::as_str),
        Some("Launch")
    );
    assert_eq!(
        milestones[0].get("id").and_then(Value::as_str),
        Some("ms-fixed")
    );
    assert_eq!(
        milestones[1].get("title").and_then(Value::as_str),
        Some("Kickoff")
    );
}

#[test]
fn update_project_metadata_rejects_bad_milestone_date() {
    let dir = tempdir("project-meta-bad-date");
    fs::copy(
        "../tst-data/planning/projects/Internal-RnD.md",
        dir.path().join("projects/Internal-RnD.md"),
    )
    .unwrap();

    let updates = ProjectMetadataUpdate {
        milestones: Some(vec![MilestoneUpdate {
            id: None,
            title: "Bad".to_string(),
            date: "not-a-date".to_string(),
        }]),
        ..Default::default()
    };
    let err = update_project_metadata(dir.path(), "Internal-RnD", &updates);
    assert!(err.is_err());
}

#[test]
fn update_project_metadata_clears_hourly_rate_when_set_to_null() {
    let dir = tempdir("project-meta-clear");
    fs::copy(
        "../tst-data/planning/projects/Internal-Enablement.md",
        dir.path().join("projects/Internal-Enablement.md"),
    )
    .unwrap();

    let before = read_frontmatter(&dir.path().join("projects/Internal-Enablement.md"));
    assert!(before.get("hourly_rate").and_then(Value::as_f64).is_some());

    let updates = ProjectMetadataUpdate {
        hourly_rate: Some(None),
        ..Default::default()
    };
    update_project_metadata(dir.path(), "Internal-Enablement", &updates).unwrap();

    let after = read_frontmatter(&dir.path().join("projects/Internal-Enablement.md"));
    assert!(after.get("hourly_rate").is_none());
}
