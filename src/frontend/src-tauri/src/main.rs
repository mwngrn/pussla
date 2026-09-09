#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::path::PathBuf;
use std::sync::Mutex;

use serde::{Deserialize, Deserializer, Serialize};
use tauri::Manager;

use pussla_core::dashboard::DashboardData;
use pussla_core::sync::{PublishResult, RefreshResult, SyncStatus};
use pussla_core::validation::DatasetValidation;
use pussla_core::write::{
    AllocationUpdate, ActivityUpdate, MilestoneUpdate, ProjectMetadataUpdate,
    UpdateProjectMetadataResult, UpdateWeekAllocationsResult,
};

/// The resolved external data folder (ADR-007): unchanged UX from the
/// previous Python-shell version — `--data-dir`, `PUSSLA_DATA_DIR`, a
/// `tst-data/` sibling in a dev checkout, or `~/Pussla-data`, in that order.
struct AppState {
    data_dir: Mutex<PathBuf>,
}

fn planning_dir(data_dir: &std::path::Path) -> PathBuf {
    let preferred = data_dir.join("planning");
    let legacy = data_dir.join("planing");
    if preferred.exists() || !legacy.exists() {
        preferred
    } else {
        legacy
    }
}

fn identity_dir(data_dir: &std::path::Path) -> PathBuf {
    data_dir.join("identity")
}

fn now_iso_seconds() -> String {
    chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string()
}

// --- Tauri commands: one per src/frontend/src/api/dashboard.ts call ---

#[tauri::command(rename_all = "snake_case")]
fn get_dashboard_data(state: tauri::State<AppState>, include_pii: bool) -> Result<DashboardData, String> {
    let data_dir = state.data_dir.lock().unwrap().clone();
    Ok(pussla_core::dashboard::build_dashboard_data(
        &planning_dir(&data_dir),
        &identity_dir(&data_dir),
        include_pii,
        now_iso_seconds(),
    ))
}

#[tauri::command(rename_all = "snake_case")]
fn validate_dataset(state: tauri::State<AppState>) -> Result<DatasetValidation, String> {
    let data_dir = state.data_dir.lock().unwrap().clone();
    let result = pussla_core::validation::validate_dataset(&planning_dir(&data_dir), &identity_dir(&data_dir), false);
    Ok(result)
}

#[tauri::command(rename_all = "snake_case")]
fn sync_status(state: tauri::State<AppState>) -> Result<SyncStatus, String> {
    let data_dir = state.data_dir.lock().unwrap().clone();
    pussla_core::sync::get_sync_status(&data_dir).map_err(|e| e.message)
}

#[derive(Serialize)]
struct OkWrapped<T: Serialize> {
    ok: bool,
    #[serde(flatten)]
    inner: T,
}

#[tauri::command(rename_all = "snake_case")]
fn sync_publish(state: tauri::State<AppState>, summary: String) -> Result<OkWrapped<PublishResult>, String> {
    let data_dir = state.data_dir.lock().unwrap().clone();
    pussla_core::sync::publish_changes(&data_dir, &summary)
        .map(|inner| OkWrapped { ok: true, inner })
        .map_err(|e| e.message)
}

#[tauri::command(rename_all = "snake_case")]
fn sync_refresh(state: tauri::State<AppState>) -> Result<OkWrapped<RefreshResult>, String> {
    let data_dir = state.data_dir.lock().unwrap().clone();
    pussla_core::sync::refresh_changes(&data_dir)
        .map(|inner| OkWrapped { ok: true, inner })
        .map_err(|e| e.message)
}

/// Distinguishes "field absent" (`None`) from "field present, value `null`"
/// (`Some(None)`) — needed for [`ProjectUpdatesPayload`]'s clear-a-field
/// semantics (ADR-011 write path: `core::write::ProjectMetadataUpdate`).
fn deserialize_present<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Option::<T>::deserialize(deserializer).map(Some)
}

#[derive(Deserialize)]
struct AllocationItemPayload {
    project: String,
    load: Option<i64>,
    planned_hours: Option<f64>,
    capacity_hours: Option<f64>,
}

#[derive(Deserialize)]
struct UpdateAllocationPayload {
    alias: String,
    week: String,
    allocations: Vec<AllocationItemPayload>,
}

#[derive(Serialize)]
struct UpdateAllocationResult {
    ok: bool,
    updated: UpdateWeekAllocationsResultWire,
}

#[derive(Serialize)]
struct UpdateWeekAllocationsResultWire {
    alias: String,
    week: String,
    projects_count: usize,
    total_load: i64,
    total_planned_hours: f64,
    capacity_hours: f64,
}

impl From<UpdateWeekAllocationsResult> for UpdateWeekAllocationsResultWire {
    fn from(r: UpdateWeekAllocationsResult) -> Self {
        Self {
            alias: r.alias,
            week: r.week,
            projects_count: r.projects_count,
            total_load: r.total_load,
            total_planned_hours: r.total_planned_hours,
            capacity_hours: r.capacity_hours,
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
fn update_allocation(
    state: tauri::State<AppState>,
    payload: UpdateAllocationPayload,
) -> Result<UpdateAllocationResult, String> {
    let data_dir = state.data_dir.lock().unwrap().clone();
    let allocations: Vec<AllocationUpdate> = payload
        .allocations
        .into_iter()
        .map(|a| AllocationUpdate {
            project: a.project,
            load: a.load,
            planned_hours: a.planned_hours,
            capacity_hours: a.capacity_hours,
            state: None,
        })
        .collect();

    pussla_core::write::update_week_allocations(&planning_dir(&data_dir), &payload.alias, &payload.week, &allocations)
        .map(|updated| UpdateAllocationResult {
            ok: true,
            updated: updated.into(),
        })
        .map_err(|e| e.to_string())
}

#[derive(Deserialize)]
struct MilestoneItemPayload {
    id: Option<String>,
    title: String,
    date: String,
}

#[derive(Deserialize)]
struct ActivityItemPayload {
    id: Option<String>,
    label: String,
    start_date: String,
    end_date: String,
}

#[derive(Deserialize, Default)]
struct ProjectUpdatesPayload {
    #[serde(default, deserialize_with = "deserialize_present")]
    hourly_rate: Option<Option<f64>>,
    #[serde(default, deserialize_with = "deserialize_present")]
    start_week_override: Option<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_present")]
    end_week_override: Option<Option<String>>,
    milestones: Option<Vec<MilestoneItemPayload>>,
    activities: Option<Vec<ActivityItemPayload>>,
}

#[derive(Deserialize)]
struct UpdateProjectPayload {
    project: String,
    updates: ProjectUpdatesPayload,
}

#[derive(Serialize)]
struct UpdateProjectResult {
    ok: bool,
    updated: UpdateProjectMetadataResultWire,
}

#[derive(Serialize)]
struct UpdateProjectMetadataResultWire {
    project: String,
    file: String,
}

impl From<UpdateProjectMetadataResult> for UpdateProjectMetadataResultWire {
    fn from(r: UpdateProjectMetadataResult) -> Self {
        Self {
            project: r.project,
            file: r.file,
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
fn update_project(
    state: tauri::State<AppState>,
    payload: UpdateProjectPayload,
) -> Result<UpdateProjectResult, String> {
    let data_dir = state.data_dir.lock().unwrap().clone();
    let updates = ProjectMetadataUpdate {
        hourly_rate: payload.updates.hourly_rate,
        start_week_override: payload.updates.start_week_override,
        end_week_override: payload.updates.end_week_override,
        milestones: payload.updates.milestones.map(|list| {
            list.into_iter()
                .map(|m| MilestoneUpdate {
                    id: m.id,
                    title: m.title,
                    date: m.date,
                })
                .collect()
        }),
        activities: payload.updates.activities.map(|list| {
            list.into_iter()
                .map(|a| ActivityUpdate {
                    id: a.id,
                    label: a.label,
                    start_date: a.start_date,
                    end_date: a.end_date,
                })
                .collect()
        }),
    };

    pussla_core::write::update_project_metadata(&planning_dir(&data_dir), &payload.project, &updates)
        .map(|updated| UpdateProjectResult {
            ok: true,
            updated: updated.into(),
        })
        .map_err(|e| e.to_string())
}

// --- Data-folder resolution (ADR-007, unchanged UX from the Python-shell version) ---

fn repo_root() -> Option<PathBuf> {
    let manifest_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_root.join("../../..").canonicalize().ok()
}

fn resolve_data_dir() -> Result<PathBuf, String> {
    if let Some(path) = parse_flag_value("--data-dir") {
        return Ok(path);
    }

    if let Ok(path) = env::var("PUSSLA_DATA_DIR") {
        let path = PathBuf::from(path);
        if path.exists() {
            return Ok(path);
        }
    }

    if let Some(root) = repo_root() {
        let test_data = root.join("tst-data");
        if test_data.exists() {
            return Ok(test_data);
        }
    }

    if let Ok(home) = env::var("HOME") {
        let home_data = PathBuf::from(home).join("Pussla-data");
        if home_data.exists() {
            return Ok(home_data);
        }
    }

    Err(
        "No external data folder is configured.\n\nPass --data-dir /path/to/data-root, set PUSSLA_DATA_DIR, or create ~/Pussla-data before starting the desktop app."
            .to_string(),
    )
}

fn parse_flag_value(flag: &str) -> Option<PathBuf> {
    let mut args = runtime_args().into_iter();
    while let Some(arg) = args.next() {
        if arg == flag {
            return args.next().map(PathBuf::from);
        }
        if let Some(value) = arg.strip_prefix(&format!("{flag}=")) {
            return Some(PathBuf::from(value));
        }
    }
    None
}

fn runtime_args() -> Vec<String> {
    if let Ok(override_args) = env::var("PUSSLA_TEST_ARGS") {
        return override_args
            .split('\n')
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
            .collect();
    }
    env::args().skip(1).collect()
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app
                .get_webview_window("main")
                .ok_or_else(|| "main window is missing".to_string())?;

            match resolve_data_dir() {
                Ok(data_dir) => {
                    app.manage(AppState {
                        data_dir: Mutex::new(data_dir),
                    });
                }
                Err(message) => {
                    window
                        .eval(format!(
                            "window.__PUSSLA_DESKTOP__?.showStatus({});",
                            serde_json::to_string(&message)?
                        ))
                        .ok();
                    // Fall back to an empty/unset data dir so the UI's own
                    // first-run flow (ADR-007) can prompt for a folder
                    // instead of the app failing to start entirely.
                    app.manage(AppState {
                        data_dir: Mutex::new(PathBuf::new()),
                    });
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_dashboard_data,
            validate_dataset,
            sync_status,
            sync_publish,
            sync_refresh,
            update_allocation,
            update_project,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Tauri application");
}

#[cfg(test)]
mod tests {
    use super::parse_flag_value;
    use std::path::PathBuf;

    #[test]
    fn parse_flag_value_supports_inline_syntax() {
        let args = vec!["--data-dir=/tmp/pussla".to_string()];
        let _guard = ArgsGuard::new(args);
        assert_eq!(
            parse_flag_value("--data-dir"),
            Some(PathBuf::from("/tmp/pussla"))
        );
    }

    #[test]
    fn parse_flag_value_supports_two_argument_syntax() {
        let args = vec!["--data-dir".to_string(), "/tmp/pussla2".to_string()];
        let _guard = ArgsGuard::new(args);
        assert_eq!(
            parse_flag_value("--data-dir"),
            Some(PathBuf::from("/tmp/pussla2"))
        );
    }

    struct ArgsGuard {
        original: Vec<String>,
    }

    impl ArgsGuard {
        fn new(replacement: Vec<String>) -> Self {
            let original: Vec<String> = std::env::args().collect();
            std::env::set_var("PUSSLA_TEST_ARGS", replacement.join("\n"));
            Self { original }
        }
    }

    impl Drop for ArgsGuard {
        fn drop(&mut self) {
            if self.original.is_empty() {
                std::env::remove_var("PUSSLA_TEST_ARGS");
            } else {
                std::env::set_var("PUSSLA_TEST_ARGS", self.original.join("\n"));
            }
        }
    }
}
