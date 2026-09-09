//! Pussla domain logic: models, frontmatter parsing, (later) validation and
//! aggregation. No I/O-framework dependency — consumed directly by the Tauri
//! desktop app (ADR-011) and, later, by an axum server.
//!
//! Module boundary (ADR-008): [`identity`] is the only module that may hold
//! PII. [`planning`] and [`project`] never import from it.

pub mod aggregate;
pub mod dashboard;
pub mod frontmatter;
pub mod identity;
pub mod iso_week;
pub mod planning;
pub mod project;
pub mod role;
pub mod rounding;
pub mod sync;
pub mod validation;
pub mod write;

pub use dashboard::{build_dashboard_data, DashboardData};
pub use identity::Identity;
pub use planning::{Allocation, AllocationState, Person};
pub use project::{Activity, Milestone, Project};
pub use role::Role;
pub use validation::{validate_dataset, DatasetValidation};
