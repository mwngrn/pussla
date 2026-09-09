//! Integration test: every file under `tst-data/planning/` and
//! `tst-data/identity/` must parse without error, into the model matching
//! its folder.

use pussla_core::{frontmatter, Identity, Person, Project, Role};
use std::fs;
use std::path::Path;

fn read_dir_md(dir: &str) -> Vec<std::path::PathBuf> {
    let path = Path::new(dir);
    if !path.exists() {
        return vec![];
    }
    fs::read_dir(path)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("md"))
        .collect()
}

#[test]
fn all_people_fixtures_parse() {
    let files = read_dir_md("../tst-data/planning/people");
    assert!(!files.is_empty(), "expected people fixtures to exist");
    for path in files {
        let raw = fs::read_to_string(&path).unwrap();
        let (_person, _body): (Person, String) = frontmatter::parse(&raw)
            .unwrap_or_else(|e| panic!("failed to parse {path:?}: {e}"));
    }
}

#[test]
fn all_project_fixtures_parse() {
    let files = read_dir_md("../tst-data/planning/projects");
    assert!(!files.is_empty(), "expected project fixtures to exist");
    for path in files {
        let raw = fs::read_to_string(&path).unwrap();
        let (_project, _body): (Project, String) = frontmatter::parse(&raw)
            .unwrap_or_else(|e| panic!("failed to parse {path:?}: {e}"));
    }
}

#[test]
fn all_role_fixtures_parse() {
    let files = read_dir_md("../tst-data/planning/roles");
    assert!(!files.is_empty(), "expected role fixtures to exist");
    for path in files {
        let raw = fs::read_to_string(&path).unwrap();
        let (_role, _body): (Role, String) = frontmatter::parse(&raw)
            .unwrap_or_else(|e| panic!("failed to parse {path:?}: {e}"));
    }
}

#[test]
fn all_identity_fixtures_parse() {
    let files = read_dir_md("../tst-data/identity");
    assert!(!files.is_empty(), "expected identity fixtures to exist");
    for path in files {
        let raw = fs::read_to_string(&path).unwrap();
        let (_identity, _body): (Identity, String) = frontmatter::parse(&raw)
            .unwrap_or_else(|e| panic!("failed to parse {path:?}: {e}"));
    }
}

/// skills.md has its own shape (canonical_skills/synonyms), not one of the
/// five typed models yet — just prove the frontmatter/body split and YAML
/// parse as a generic value, since AC #3 covers "every file under
/// tst-data/planning/", not just the five typed models.
#[test]
fn skills_file_frontmatter_parses_as_generic_yaml() {
    let raw = fs::read_to_string("../tst-data/planning/skills.md").unwrap();
    let note = frontmatter::split(&raw).unwrap();
    let _value: serde_yaml::Value = serde_yaml::from_str(note.frontmatter_yaml).unwrap();
    assert!(note.body.contains("Skills"));
}

#[test]
fn identity_module_never_imports_planning_or_project_types() {
    // Structural guard, not a runtime check: identity.rs must not reference
    // Person/Project/Allocation. Enforced here by grepping the source so a
    // future edit that violates the ADR-008 module boundary fails CI instead
    // of silently merging.
    let identity_src = fs::read_to_string("src/identity.rs").unwrap();
    assert!(!identity_src.contains("use crate::planning"));
    assert!(!identity_src.contains("use crate::project"));
    assert!(!identity_src.contains("struct Person"));
    assert!(!identity_src.contains("struct Project"));
}
