//! Identity layer (ADR-003, ADR-008) — **the only place in this crate that
//! may hold PII**. Deliberately isolated in its own module with its own
//! type: nothing in [`crate::planning`] or [`crate::project`] imports from
//! here, and `Identity` is never nested inside `Person`/`Project`/
//! `Allocation`. The only link between the two layers is the shared
//! filename/alias — callers join them locally (e.g. a desktop-side
//! resolver), never inside a single parsed struct.
//!
//! Per ADR-008 Option B, the identity vault's path is separate,
//! agent-unreachable configuration — this module only defines the shape of
//! one identity file, not vault discovery/wiring (that's a future task).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub alias: String,
    pub real_name: String,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub home_office: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontmatter;

    #[test]
    fn parses_identity_file() {
        let raw = std::fs::read_to_string("../tst-data/identity/FishCatcher.md").unwrap();
        let (identity, body): (Identity, String) = frontmatter::parse(&raw).unwrap();
        assert_eq!(identity.alias, "FishCatcher");
        assert_eq!(identity.real_name, "Erik Andersson");
        assert!(body.contains("Project-Neon"));
    }
}
