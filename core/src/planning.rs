//! Person and allocation models — the person-centric planning layer (ADR-001,
//! ADR-004, ADR-005, ADR-006). Contains **no PII**: only aliases. See
//! [`crate::identity`] for the separate, physically distinct identity layer
//! (ADR-008) — the two are never combined into one struct or module.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AllocationState {
    Tentative,
    Committed,
}

/// One row of planned work: a project, a set of ISO weeks (e.g. `"2026-W09"`),
/// and the effort for that span. `weeks` following ADR-004; `planned_hours`/
/// `capacity_hours` following ADR-005 (hours are canonical, `load` percent is
/// carried as-authored today but is a derived value going forward).
///
/// `planned_hours`, `capacity_hours`, and `state` are `Option` because
/// `tst-data/` still contains pre-ADR-005 rows with only `load` (percent) —
/// see e.g. `planning/people/PixelSmith.md`. ADR-005 explicitly allows this:
/// legacy percent-only data converts via `planned_hours = load / 100 *
/// capacity_hours`. Rejecting it here would make the parser stricter than
/// the ADR it's meant to implement; conversion/validation is task-45.3.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Allocation {
    pub project: String,
    pub weeks: Vec<String>,
    #[serde(default)]
    pub planned_hours: Option<f64>,
    #[serde(default)]
    pub capacity_hours: Option<f64>,
    pub load: u32,
    #[serde(default)]
    pub state: Option<AllocationState>,
}

/// A person profile, keyed by alias (ADR-003). Lives at
/// `planning/people/<alias>.md`. The filename *is* the alias; there is no
/// separate id field required, but `alias` is also carried in frontmatter for
/// self-description.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub alias: String,
    pub role_id: String,
    #[serde(default)]
    pub skills: Vec<String>,
    #[serde(default)]
    pub allocations: Vec<Allocation>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontmatter;

    #[test]
    fn parses_person_with_allocations() {
        let raw = std::fs::read_to_string(
            "../tst-data/planning/people/ArcticPine.md",
        )
        .expect("fixture must exist");
        let (person, _body): (Person, String) = frontmatter::parse(&raw).unwrap();
        assert_eq!(person.alias, "ArcticPine");
        assert_eq!(person.role_id, "Consultant");
        assert!(!person.allocations.is_empty());
        let first = &person.allocations[0];
        assert_eq!(first.project, "Project-Bridge");
        assert_eq!(first.weeks[0], "2026-W09");
        assert_eq!(first.state, Some(AllocationState::Committed));
    }

    #[test]
    fn parses_person_with_tentative_state() {
        let raw = std::fs::read_to_string(
            "../tst-data/planning/people/ArcticPine.md",
        )
        .unwrap();
        let (person, _): (Person, String) = frontmatter::parse(&raw).unwrap();
        assert!(person
            .allocations
            .iter()
            .any(|a| a.state == Some(AllocationState::Tentative)));
    }

    #[test]
    fn parses_legacy_load_only_allocation_without_hours_or_state() {
        // PixelSmith.md predates ADR-005: rows have only `load`, no
        // planned_hours/capacity_hours/state. Must still parse (ADR-005
        // allows legacy percent-only data; conversion is a separate concern).
        let raw = std::fs::read_to_string(
            "../tst-data/planning/people/PixelSmith.md",
        )
        .unwrap();
        let (person, _): (Person, String) = frontmatter::parse(&raw).unwrap();
        let first = &person.allocations[0];
        assert_eq!(first.planned_hours, None);
        assert_eq!(first.capacity_hours, None);
        assert_eq!(first.state, None);
        assert_eq!(first.load, 85);
    }
}
