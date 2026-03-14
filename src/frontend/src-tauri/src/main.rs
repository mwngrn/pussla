#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

use tauri::{AppHandle, Manager, RunEvent};

#[derive(Default)]
struct BackendProcess(Mutex<Option<Child>>);

struct RuntimePaths {
    dashboard_script: PathBuf,
    static_dir: PathBuf,
    repo_root: Option<PathBuf>,
}

fn main() {
    tauri::Builder::default()
        .manage(BackendProcess::default())
        .setup(|app| {
            let window = app
                .get_webview_window("main")
                .ok_or_else(|| "main window is missing".to_string())?;
            let runtime_paths = resolve_runtime_paths(app.handle())?;
            let data_dir = resolve_data_dir(&runtime_paths)?;
            let port = reserve_port()?;
            let python = env::var("PUSSLA_PYTHON").unwrap_or_else(|_| "python3".to_string());

            window
                .eval(&format!(
                    "window.__PUSSLA_DESKTOP__?.showStatus({});",
                    serde_json::to_string(&format!(
                        "Starting local service...\nData folder: {}",
                        data_dir.display()
                    ))?
                ))
                .ok();

            let child = spawn_backend(&python, &runtime_paths, &data_dir, port)?;
            *app.state::<BackendProcess>().0.lock().unwrap() = Some(child);

            let window_for_thread = window.clone();
            thread::spawn(move || {
                let url = format!("http://127.0.0.1:{port}");
                match wait_for_backend(&url, Duration::from_secs(20)) {
                    Ok(()) => {
                        let script = format!(
                            "window.location.replace({});",
                            serde_json::to_string(&url).unwrap()
                        );
                        let _ = window_for_thread.eval(&script);
                    }
                    Err(message) => {
                        let script = format!(
                            "window.__PUSSLA_DESKTOP__?.showStatus({});",
                            serde_json::to_string(&message).unwrap()
                        );
                        let _ = window_for_thread.eval(&script);
                    }
                }
            });

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("failed to build Tauri application")
        .run(|app, event| match event {
            RunEvent::Exit | RunEvent::ExitRequested { .. } => {
                if let Some(mut child) = app.state::<BackendProcess>().0.lock().unwrap().take() {
                    let _ = child.kill();
                    let _ = child.wait();
                }
            }
            _ => {}
        });
}

fn resolve_runtime_paths(app: &AppHandle) -> Result<RuntimePaths, String> {
    let manifest_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_root.join("../../..").canonicalize().ok();

    if let Some(root) = repo_root.clone() {
        let script = root.join("src/dashboard/run_dashboard.py");
        let static_dir = root.join("src/frontend/dist");
        if script.exists() && static_dir.join("index.html").exists() {
            return Ok(RuntimePaths {
                dashboard_script: script,
                static_dir,
                repo_root: Some(root),
            });
        }
    }

    let resource_dir = app
        .path()
        .resource_dir()
        .map_err(|e| format!("Could not resolve Tauri resource directory: {e}"))?;
    let dashboard_script = first_existing_path(
        &resource_dir,
        &[
            "src/dashboard/run_dashboard.py",
            "dashboard/run_dashboard.py",
            "resources/src/dashboard/run_dashboard.py",
        ],
    )
    .ok_or_else(|| {
        format!(
            "Could not locate bundled dashboard backend resources under {}",
            resource_dir.display()
        )
    })?;
    let static_dir = first_existing_dir(
        &resource_dir,
        &[
            "dist",
            "src/frontend/dist",
            "resources/dist",
            "resources/src/frontend/dist",
        ],
    )
    .ok_or_else(|| {
        format!(
            "Could not locate bundled frontend dist under {}",
            resource_dir.display()
        )
    })?;

    Ok(RuntimePaths {
        dashboard_script,
        static_dir,
        repo_root: None,
    })
}

fn first_existing_path(base: &Path, candidates: &[&str]) -> Option<PathBuf> {
    candidates
        .iter()
        .map(|candidate| base.join(candidate))
        .find(|path| path.exists())
}

fn first_existing_dir(base: &Path, candidates: &[&str]) -> Option<PathBuf> {
    candidates
        .iter()
        .map(|candidate| base.join(candidate))
        .find(|path| path.join("index.html").exists())
}

fn resolve_data_dir(runtime: &RuntimePaths) -> Result<PathBuf, String> {
    if let Some(path) = parse_flag_value("--data-dir") {
        return Ok(path);
    }

    if let Ok(path) = env::var("PUSSLA_DATA_DIR") {
        let path = PathBuf::from(path);
        if path.exists() {
            return Ok(path);
        }
    }

    if let Some(root) = &runtime.repo_root {
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

fn reserve_port() -> Result<u16, String> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .map_err(|e| format!("Could not reserve a local port for the desktop backend: {e}"))?;
    listener
        .local_addr()
        .map(|addr| addr.port())
        .map_err(|e| format!("Could not read reserved local port: {e}"))
}

fn spawn_backend(
    python: &str,
    runtime: &RuntimePaths,
    data_dir: &Path,
    port: u16,
) -> Result<Child, String> {
    let script_dir = runtime
        .dashboard_script
        .parent()
        .ok_or_else(|| "Dashboard script has no parent directory".to_string())?;
    let mut command = Command::new(python);
    command
        .arg(&runtime.dashboard_script)
        .arg("--host")
        .arg("127.0.0.1")
        .arg("--port")
        .arg(port.to_string())
        .arg("--data-dir")
        .arg(data_dir)
        .arg("--static-dir")
        .arg(&runtime.static_dir)
        .current_dir(script_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    command
        .spawn()
        .map_err(|e| format!("Could not start the local Python service with '{python}': {e}"))
}

fn wait_for_backend(url: &str, timeout: Duration) -> Result<(), String> {
    let start = Instant::now();
    let host = "127.0.0.1";
    let port = url
        .rsplit(':')
        .next()
        .ok_or_else(|| "Could not parse backend URL".to_string())?
        .parse::<u16>()
        .map_err(|e| format!("Could not parse backend port: {e}"))?;

    while start.elapsed() < timeout {
        if TcpStream::connect((host, port)).is_ok() {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(150));
    }

    Err(format!(
        "The local Pussla service did not become ready in time.\n\nTried to reach {url}.\nCheck that Python is installed and that the selected data folder is valid."
    ))
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

    struct ArgsGuard {
        original: Vec<String>,
    }

    impl ArgsGuard {
        fn new(replacement: Vec<String>) -> Self {
            let original: Vec<String> = std::env::args().collect();
            // Tests run in-process, so use the unstable test-only override through env.
            // Fall back to verifying the parser helper via direct branch coverage in real runs.
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
