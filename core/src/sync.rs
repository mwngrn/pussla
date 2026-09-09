//! Git-backed Publish/Refresh — ports `src/dashboard/dashboard_sync.py`.
//! Shells out to the system `git` CLI (ADR-011: no git library dependency)
//! rather than reimplementing git plumbing. Output shapes
//! (`SyncStatus`/`SyncChange`/`PublishResult`/`RefreshResult`) match
//! `src/frontend/src/api/dashboard.ts`'s existing TypeScript interfaces
//! field-for-field.

use serde::Serialize;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
pub struct SyncError {
    pub message: String,
    pub status_code: u16,
}

impl std::fmt::Display for SyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SyncError {}

impl SyncError {
    fn new(message: impl Into<String>) -> Self {
        SyncError {
            message: message.into(),
            status_code: 400,
        }
    }
    fn with_status(message: impl Into<String>, status_code: u16) -> Self {
        SyncError {
            message: message.into(),
            status_code,
        }
    }
}

fn run_git(repo_root: &Path, args: &[&str]) -> Result<String, SyncError> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(args)
        .output()
        .map_err(|e| SyncError::new(format!("failed to run git: {e}")))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let message = if !stderr.is_empty() {
            stderr
        } else if !stdout.is_empty() {
            stdout
        } else {
            format!("git {} failed", args.join(" "))
        };
        return Err(SyncError::new(message));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn try_git(repo_root: &Path, args: &[&str]) -> Option<String> {
    run_git(repo_root, args).ok()
}

fn resolve_repo_root(data_dir: &Path) -> Result<PathBuf, SyncError> {
    try_git(data_dir, &["rev-parse", "--show-toplevel"])
        .map(PathBuf::from)
        .ok_or_else(|| {
            SyncError::new(format!(
                "No Git repository found for dashboard data at {}. Open a planning repository or configure Git before using publish/refresh.",
                data_dir.display()
            ))
        })
}

fn pathspec(repo_root: &Path, data_dir: &Path) -> Result<String, SyncError> {
    let repo_root = repo_root
        .canonicalize()
        .map_err(|e| SyncError::new(e.to_string()))?;
    let data_dir = data_dir
        .canonicalize()
        .map_err(|e| SyncError::new(e.to_string()))?;
    match data_dir.strip_prefix(&repo_root) {
        Ok(rel) if rel.as_os_str().is_empty() => Ok(".".to_string()),
        Ok(rel) => Ok(rel.to_string_lossy().replace('\\', "/")),
        Err(_) => Err(SyncError::new(format!(
            "Dashboard data directory {} is not inside Git repository {}.",
            data_dir.display(),
            repo_root.display()
        ))),
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SyncChange {
    pub code: String,
    pub path: String,
}

fn parse_status_lines(output: &str) -> Vec<SyncChange> {
    output
        .lines()
        .filter(|l| l.len() >= 4)
        .map(|line| SyncChange {
            code: line[..2].to_string(),
            path: line[3..].to_string(),
        })
        .collect()
}

fn current_branch(repo_root: &Path) -> Result<String, SyncError> {
    let branch = try_git(repo_root, &["symbolic-ref", "--quiet", "--short", "HEAD"]);
    match branch {
        Some(b) if !b.is_empty() && b != "HEAD" => Ok(b),
        _ => Err(SyncError::new(
            "Detached HEAD is not supported for dashboard publish/refresh. Check out a branch first.",
        )),
    }
}

struct RemoteTarget {
    label: String,
    target_ref: String,
    auto_configured: bool,
}

fn resolve_remote_target(repo_root: &Path, branch: &str) -> Result<Option<RemoteTarget>, SyncError> {
    if let Some(upstream) = try_git(
        repo_root,
        &[
            "rev-parse",
            "--abbrev-ref",
            "--symbolic-full-name",
            "@{upstream}",
        ],
    ) {
        return Ok(Some(RemoteTarget {
            label: upstream.clone(),
            target_ref: upstream,
            auto_configured: false,
        }));
    }

    let remotes_output = run_git(repo_root, &["remote"])?;
    let remotes: Vec<&str> = remotes_output.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
    if remotes.is_empty() {
        return Ok(None);
    }
    let remote = if remotes.contains(&"origin") { "origin" } else { remotes[0] };
    Ok(Some(RemoteTarget {
        label: format!("{remote}/{branch}"),
        target_ref: format!("{remote}/{branch}"),
        auto_configured: true,
    }))
}

fn ahead_behind(repo_root: &Path, target_ref: &str) -> (Option<i64>, Option<i64>) {
    let Some(counts) = try_git(
        repo_root,
        &["rev-list", "--left-right", "--count", &format!("{target_ref}...HEAD")],
    ) else {
        return (None, None);
    };
    let mut parts = counts.split_whitespace();
    let (Some(left), Some(right)) = (parts.next(), parts.next()) else {
        return (None, None);
    };
    (right.parse().ok(), left.parse().ok())
}

#[derive(Debug, Clone, Serialize)]
pub struct SyncStatus {
    pub repo_root: String,
    pub data_dir: String,
    pub data_scope: String,
    pub branch: String,
    pub head: String,
    pub last_commit_subject: String,
    pub publish_target: Option<String>,
    pub has_publish_target: bool,
    pub auto_configured_target: bool,
    pub scoped_dirty: bool,
    pub repo_dirty: bool,
    pub scoped_changes: Vec<SyncChange>,
    pub repo_change_count: usize,
    pub ahead: Option<i64>,
    pub behind: Option<i64>,
}

pub fn get_sync_status(data_dir: &Path) -> Result<SyncStatus, SyncError> {
    let data_path = data_dir
        .canonicalize()
        .map_err(|e| SyncError::new(format!("{}: {e}", data_dir.display())))?;
    let repo_root = resolve_repo_root(&data_path)?;
    let scope = pathspec(&repo_root, &data_path)?;
    let branch = current_branch(&repo_root)?;
    let remote_target = resolve_remote_target(&repo_root, &branch)?;

    let scoped_changes = parse_status_lines(&run_git(
        &repo_root,
        &["status", "--porcelain", "--untracked-files=all", "--", &scope],
    )?);
    let repo_changes = parse_status_lines(&run_git(
        &repo_root,
        &["status", "--porcelain", "--untracked-files=all"],
    )?);

    let (ahead, behind) = match &remote_target {
        Some(t) => ahead_behind(&repo_root, &t.target_ref),
        None => (None, None),
    };

    let head = try_git(&repo_root, &["rev-parse", "--short", "HEAD"]).unwrap_or_else(|| "uncommitted".to_string());
    let last_commit_subject =
        try_git(&repo_root, &["log", "-1", "--pretty=%s"]).unwrap_or_else(|| "No commits yet".to_string());

    Ok(SyncStatus {
        repo_root: repo_root.display().to_string(),
        data_dir: data_path.display().to_string(),
        data_scope: scope,
        branch,
        head,
        last_commit_subject,
        publish_target: remote_target.as_ref().map(|t| t.label.clone()),
        has_publish_target: remote_target.is_some(),
        auto_configured_target: remote_target.as_ref().map(|t| t.auto_configured).unwrap_or(false),
        scoped_dirty: !scoped_changes.is_empty(),
        repo_dirty: !repo_changes.is_empty(),
        scoped_changes,
        repo_change_count: repo_changes.len(),
        ahead,
        behind,
    })
}

#[derive(Debug, Clone, Serialize)]
pub struct PublishResult {
    pub commit_id: String,
    pub summary: String,
    pub publish_target: String,
    pub status: SyncStatus,
}

pub fn publish_changes(data_dir: &Path, summary: &str) -> Result<PublishResult, SyncError> {
    let summary = summary.trim();
    if summary.is_empty() {
        return Err(SyncError::new("A short change summary is required before publishing."));
    }

    let status = get_sync_status(data_dir)?;
    if !status.scoped_dirty {
        return Err(SyncError::new(
            "There are no unpublished planning changes in the selected data folder.",
        ));
    }
    if !status.has_publish_target {
        return Err(SyncError::new(
            "No shared Git remote is configured for this repository. Configure a remote before using Publish.",
        ));
    }

    let repo_root = PathBuf::from(&status.repo_root);
    let scope = status.data_scope.clone();
    let branch = status.branch.clone();
    let target_label = status.publish_target.clone().unwrap();
    let remote = target_label.split('/').next().unwrap_or_default().to_string();
    let uses_upstream = !status.auto_configured_target;

    run_git(&repo_root, &["add", "--all", "--", &scope])?;
    run_git(&repo_root, &["commit", "-m", &format!("Planning update: {summary}")])
        .map_err(|e| SyncError::new(format!("Could not create a versioned save for publish: {e}")))?;

    let commit_id = run_git(&repo_root, &["rev-parse", "--short", "HEAD"])?;

    let push_args: Vec<&str> = if uses_upstream {
        vec!["push"]
    } else {
        vec!["push", "-u", &remote, &branch]
    };
    run_git(&repo_root, &push_args).map_err(|e| {
        SyncError::with_status(
            format!("Created a local versioned save, but could not send it to the shared repository. Local commit {commit_id}: {e}"),
            409,
        )
    })?;

    let next_status = get_sync_status(data_dir)?;
    Ok(PublishResult {
        commit_id,
        summary: summary.to_string(),
        publish_target: target_label,
        status: next_status,
    })
}

#[derive(Debug, Clone, Serialize)]
pub struct RefreshResult {
    pub updated: bool,
    pub message: String,
    pub previous_head: Option<String>,
    pub head: Option<String>,
    pub status: SyncStatus,
}

pub fn refresh_changes(data_dir: &Path) -> Result<RefreshResult, SyncError> {
    let status = get_sync_status(data_dir)?;
    if !status.has_publish_target {
        return Err(SyncError::new(
            "No shared Git remote is configured for this repository. Configure a remote before using Refresh.",
        ));
    }
    if status.repo_dirty {
        return Err(SyncError::with_status(
            "Local unpublished changes would be affected by refresh. Publish or discard local changes before refreshing.",
            409,
        ));
    }

    let repo_root = PathBuf::from(&status.repo_root);
    let branch = status.branch.clone();
    let target_label = status.publish_target.clone().unwrap();
    let remote = target_label.split('/').next().unwrap_or_default().to_string();

    run_git(&repo_root, &["fetch", &remote, &branch])?;

    let refreshed_status = get_sync_status(data_dir)?;
    let ahead = refreshed_status.ahead.unwrap_or(0);
    let behind = refreshed_status.behind.unwrap_or(0);

    if ahead > 0 && behind > 0 {
        return Err(SyncError::with_status(
            "Local and remote history have diverged. Refresh cannot continue safely. Use manual Git support to resolve the branch divergence.",
            409,
        ));
    }
    if ahead > 0 {
        return Err(SyncError::with_status(
            "Local unpublished commits exist. Publish your local planning changes before refreshing.",
            409,
        ));
    }
    if behind == 0 {
        return Ok(RefreshResult {
            updated: false,
            message: "Local planning data is already up to date.".to_string(),
            previous_head: None,
            head: None,
            status: refreshed_status,
        });
    }

    let previous_head = status.head.clone();
    run_git(&repo_root, &["merge", "--ff-only", &target_label])?;
    let next_status = get_sync_status(data_dir)?;

    Ok(RefreshResult {
        updated: true,
        previous_head: Some(previous_head),
        head: Some(next_status.head.clone()),
        message: "Planning data refreshed from the shared repository.".to_string(),
        status: next_status,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

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
            "pussla-core-sync-test-{name}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&dir).unwrap();
        TempDir(dir)
    }

    fn git(dir: &Path, args: &[&str]) {
        let status = Command::new("git").arg("-C").arg(dir).args(args).status().unwrap();
        assert!(status.success(), "git {args:?} failed in {}", dir.display());
    }

    /// Sets up: a bare "remote" repo, a clone of it with one commit, a
    /// tracked upstream branch, and a `planning/` subdir as the "data dir"
    /// passed to sync functions (mirrors this project's actual layout).
    fn setup_repo_with_remote() -> (TempDir, PathBuf) {
        let root = tempdir("remote-setup");
        let remote_path = root.path().join("remote.git");
        fs::create_dir_all(&remote_path).unwrap();
        git(&remote_path, &["init", "--bare", "-q"]);
        // Bare repos default HEAD to refs/heads/master; without this, a
        // `git clone` of the bare remote before any branch exists there
        // warns "HEAD of the remote points to a non-existent reference"
        // and leaves the clone without a checked-out working tree.
        git(&remote_path, &["symbolic-ref", "HEAD", "refs/heads/main"]);

        let repo_path = root.path().join("repo");
        fs::create_dir_all(repo_path.join("planning")).unwrap();
        git(&repo_path, &["init", "-q", "-b", "main"]);
        git(&repo_path, &["config", "user.email", "test@example.com"]);
        git(&repo_path, &["config", "user.name", "Test"]);
        fs::write(repo_path.join("planning/note.md"), "hello\n").unwrap();
        git(&repo_path, &["add", "-A"]);
        git(&repo_path, &["commit", "-q", "-m", "initial"]);
        git(
            &repo_path,
            &["remote", "add", "origin", remote_path.to_str().unwrap()],
        );
        git(&repo_path, &["push", "-q", "-u", "origin", "main"]);

        let data_dir = repo_path.join("planning");
        (root, data_dir)
    }

    #[test]
    fn publish_commits_and_pushes_dirty_changes() {
        let (_root, data_dir) = setup_repo_with_remote();
        fs::write(data_dir.join("new.md"), "new content\n").unwrap();

        let result = publish_changes(&data_dir, "add new note").unwrap();
        assert_eq!(result.summary, "add new note");
        assert!(!result.commit_id.is_empty());
        assert!(!result.status.scoped_dirty);
    }

    #[test]
    fn publish_fails_when_nothing_is_dirty() {
        let (_root, data_dir) = setup_repo_with_remote();
        let err = publish_changes(&data_dir, "no changes").unwrap_err();
        assert!(err.message.contains("no unpublished planning changes"));
    }

    #[test]
    fn refresh_reports_already_up_to_date_with_no_remote_changes() {
        let (_root, data_dir) = setup_repo_with_remote();
        let result = refresh_changes(&data_dir).unwrap();
        assert!(!result.updated);
        assert_eq!(result.message, "Local planning data is already up to date.");
    }

    #[test]
    fn refresh_fails_when_repo_is_dirty() {
        let (_root, data_dir) = setup_repo_with_remote();
        fs::write(data_dir.join("uncommitted.md"), "wip\n").unwrap();
        let err = refresh_changes(&data_dir).unwrap_err();
        assert_eq!(err.status_code, 409);
        assert!(err.message.contains("Local unpublished changes"));
    }

    #[test]
    fn refresh_fails_when_local_is_ahead_of_remote() {
        let (root, data_dir) = setup_repo_with_remote();
        let repo_path = root.path().join("repo");
        fs::write(data_dir.join("local-only.md"), "local\n").unwrap();
        git(&repo_path, &["add", "-A"]);
        git(&repo_path, &["commit", "-q", "-m", "local commit, not pushed"]);

        let err = refresh_changes(&data_dir).unwrap_err();
        assert_eq!(err.status_code, 409);
        assert!(err.message.contains("Publish your local planning changes"));
    }

    #[test]
    fn refresh_fails_on_diverged_history() {
        let (root, data_dir) = setup_repo_with_remote();
        let repo_path = root.path().join("repo");
        let remote_path = root.path().join("remote.git");

        // Advance the remote independently (simulating another machine's push).
        let other_clone = root.path().join("other-clone");
        git(root.path(), &["clone", "-q", remote_path.to_str().unwrap(), "other-clone"]);
        git(&other_clone, &["config", "user.email", "test@example.com"]);
        git(&other_clone, &["config", "user.name", "Test"]);
        fs::write(other_clone.join("planning").join("remote-change.md"), "remote\n").unwrap();
        git(&other_clone, &["add", "-A"]);
        git(&other_clone, &["commit", "-q", "-m", "remote commit"]);
        git(&other_clone, &["push", "-q"]);

        // Advance local independently too, so histories diverge.
        fs::write(data_dir.join("local-change.md"), "local\n").unwrap();
        git(&repo_path, &["add", "-A"]);
        git(&repo_path, &["commit", "-q", "-m", "local commit, diverged"]);

        let err = refresh_changes(&data_dir).unwrap_err();
        assert_eq!(err.status_code, 409);
        assert!(err.message.contains("diverged"));
    }
}
