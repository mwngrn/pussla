//! Role catalog (ADR-006) — `planning/roles/<role_id>.md`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub role_id: String,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontmatter;

    #[test]
    fn parses_role_catalog_entry() {
        let raw = std::fs::read_to_string("../tst-data/planning/roles/Consultant.md").unwrap();
        let (role, body): (Role, String) = frontmatter::parse(&raw).unwrap();
        assert_eq!(role.role_id, "Consultant");
        assert_eq!(role.name, "Consultant");
        assert!(body.contains("Default consultant role"));
    }
}
