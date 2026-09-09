//! Project model — the PM-first artifact (ADR-009/ADR-010): customer, span,
//! phases/milestones. Assignments are **not** stored here — they live on
//! [`crate::planning::Person`] (ADR-010 §1). Contains no PII: `owner_alias`
//! and `team_aliases` are aliases, never real names.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: String,
    pub title: String,
    /// ISO date (`YYYY-MM-DD`), per ADR-010 §2: project *span* is dates.
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub label: String,
    pub start_date: String,
    pub end_date: String,
}

/// A project note, keyed by `project_id`. Lives at
/// `planning/projects/<Name>.md`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub project_id: String,
    pub name: String,
    pub owner_alias: String,
    pub start_week: String,
    pub end_week: String,
    pub status: String,
    #[serde(default)]
    pub capacity_target: Option<f64>,
    #[serde(default)]
    pub team_aliases: Vec<String>,
    #[serde(default)]
    pub milestones: Vec<Milestone>,
    #[serde(default)]
    pub activities: Vec<Activity>,
    #[serde(default)]
    pub hourly_rate: Option<f64>,
    /// Reference to a `planning/customers/<slug>.md` note (ADR-010 §4).
    /// Optional for now: introduced alongside the Customer note type.
    #[serde(default)]
    pub customer_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontmatter;

    #[test]
    fn parses_project_with_milestones_and_activities() {
        let raw = std::fs::read_to_string(
            "../tst-data/planning/projects/Internal-Enablement.md",
        )
        .unwrap();
        let (project, body): (Project, String) = frontmatter::parse(&raw).unwrap();
        assert_eq!(project.project_id, "internal-enablement");
        assert_eq!(project.owner_alias, "NorthHarbor");
        assert_eq!(project.status, "active");
        assert!(!project.milestones.is_empty());
        assert!(!project.activities.is_empty());
        assert!(body.contains("## Scope"));
    }

    #[test]
    fn parses_project_with_inline_flow_list_team_aliases() {
        let raw = std::fs::read_to_string(
            "../tst-data/planning/projects/Project-Bridge.md",
        )
        .unwrap();
        let (project, _): (Project, String) = frontmatter::parse(&raw).unwrap();
        // team_aliases: [ArcticPine] — inline YAML flow-list syntax
        assert_eq!(project.team_aliases, vec!["ArcticPine".to_string()]);
        assert_eq!(project.start_week, "2026-W09");
    }

    #[test]
    fn parses_project_without_milestones_or_activities() {
        let raw = std::fs::read_to_string(
            "../tst-data/planning/projects/Internal-RnD.md",
        )
        .unwrap();
        let (project, _): (Project, String) = frontmatter::parse(&raw).unwrap();
        assert!(project.milestones.is_empty());
        assert!(project.activities.is_empty());
    }
}
