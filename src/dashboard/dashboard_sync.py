from __future__ import annotations

import subprocess
from pathlib import Path


class DashboardSyncError(RuntimeError):
    def __init__(
        self,
        message: str,
        *,
        status_code: int = 400,
        details: dict | None = None,
    ) -> None:
        super().__init__(message)
        self.status_code = status_code
        self.details = details or {}


def _run_git(repo_root: Path, args: list[str]) -> str:
    try:
        completed = subprocess.run(
            ["git", "-C", str(repo_root), *args],
            check=True,
            capture_output=True,
            text=True,
        )
    except subprocess.CalledProcessError as exc:
        stderr = (exc.stderr or "").strip()
        stdout = (exc.stdout or "").strip()
        message = stderr or stdout or f"git {' '.join(args)} failed"
        raise DashboardSyncError(message) from exc
    return completed.stdout.strip()


def _try_git(repo_root: Path, args: list[str]) -> str | None:
    try:
        return _run_git(repo_root, args)
    except DashboardSyncError:
        return None


def _resolve_repo_root(data_dir: Path) -> Path:
    repo_root = _try_git(data_dir, ["rev-parse", "--show-toplevel"])
    if not repo_root:
        raise DashboardSyncError(
            f"No Git repository found for dashboard data at {data_dir}. "
            "Open a planning repository or configure Git before using publish/refresh.",
            status_code=400,
        )
    return Path(repo_root)


def _pathspec(repo_root: Path, data_dir: Path) -> str:
    try:
        relative = data_dir.resolve().relative_to(repo_root.resolve())
    except ValueError as exc:
        raise DashboardSyncError(
            f"Dashboard data directory {data_dir} is not inside Git repository {repo_root}.",
            status_code=400,
        ) from exc
    return "." if str(relative) in {"", "."} else relative.as_posix()


def _parse_status_lines(output: str) -> list[dict[str, str]]:
    changes: list[dict[str, str]] = []
    for line in output.splitlines():
        if len(line) < 4:
            continue
        changes.append(
            {
                "code": line[:2],
                "path": line[3:],
            }
        )
    return changes


def _current_branch(repo_root: Path) -> str:
    branch = _try_git(repo_root, ["symbolic-ref", "--quiet", "--short", "HEAD"])
    if not branch or branch == "HEAD":
        raise DashboardSyncError(
            "Detached HEAD is not supported for dashboard publish/refresh. "
            "Check out a branch first.",
            status_code=400,
        )
    return branch


def _resolve_remote_target(repo_root: Path, branch: str) -> dict[str, str | bool] | None:
    upstream = _try_git(
        repo_root,
        ["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{upstream}"],
    )
    if upstream:
        remote, _, _ = upstream.partition("/")
        return {
            "remote": remote,
            "branch": branch,
            "target_ref": upstream,
            "label": upstream,
            "uses_upstream": True,
            "auto_configured": False,
        }

    remotes_output = _run_git(repo_root, ["remote"])
    remotes = [line.strip() for line in remotes_output.splitlines() if line.strip()]
    if not remotes:
        return None

    remote = "origin" if "origin" in remotes else remotes[0]
    return {
        "remote": remote,
        "branch": branch,
        "target_ref": f"{remote}/{branch}",
        "label": f"{remote}/{branch}",
        "uses_upstream": False,
        "auto_configured": True,
    }


def _ahead_behind(repo_root: Path, target_ref: str) -> tuple[int | None, int | None]:
    counts = _try_git(repo_root, ["rev-list", "--left-right", "--count", f"{target_ref}...HEAD"])
    if not counts:
        return None, None
    left, right = counts.split()
    return int(right), int(left)


def get_sync_status(data_dir: str | Path) -> dict:
    data_path = Path(data_dir).resolve()
    repo_root = _resolve_repo_root(data_path)
    pathspec = _pathspec(repo_root, data_path)
    branch = _current_branch(repo_root)
    remote_target = _resolve_remote_target(repo_root, branch)

    scoped_changes = _parse_status_lines(
        _run_git(repo_root, ["status", "--porcelain", "--untracked-files=all", "--", pathspec])
    )
    repo_changes = _parse_status_lines(
        _run_git(repo_root, ["status", "--porcelain", "--untracked-files=all"])
    )

    ahead, behind = (None, None)
    if remote_target:
        ahead, behind = _ahead_behind(repo_root, str(remote_target["target_ref"]))

    head = _try_git(repo_root, ["rev-parse", "--short", "HEAD"]) or "uncommitted"
    last_commit_subject = _try_git(repo_root, ["log", "-1", "--pretty=%s"]) or "No commits yet"

    return {
        "repo_root": str(repo_root),
        "data_dir": str(data_path),
        "data_scope": pathspec,
        "branch": branch,
        "head": head,
        "last_commit_subject": last_commit_subject,
        "publish_target": remote_target["label"] if remote_target else None,
        "has_publish_target": remote_target is not None,
        "auto_configured_target": bool(remote_target and remote_target["auto_configured"]),
        "scoped_dirty": bool(scoped_changes),
        "repo_dirty": bool(repo_changes),
        "scoped_changes": scoped_changes,
        "repo_change_count": len(repo_changes),
        "ahead": ahead,
        "behind": behind,
    }


def publish_changes(data_dir: str | Path, summary: str) -> dict:
    summary = summary.strip()
    if not summary:
        raise DashboardSyncError("A short change summary is required before publishing.", status_code=400)

    status = get_sync_status(data_dir)
    if not status["scoped_dirty"]:
        raise DashboardSyncError(
            "There are no unpublished planning changes in the selected data folder.",
            status_code=400,
        )
    if not status["has_publish_target"]:
        raise DashboardSyncError(
            "No shared Git remote is configured for this repository. "
            "Configure a remote before using Publish.",
            status_code=400,
        )

    repo_root = Path(status["repo_root"])
    pathspec = str(status["data_scope"])
    branch = str(status["branch"])
    target_label = str(status["publish_target"])
    remote = target_label.split("/", 1)[0]
    uses_upstream = not bool(status["auto_configured_target"])

    _run_git(repo_root, ["add", "--all", "--", pathspec])
    try:
        _run_git(repo_root, ["commit", "-m", f"Planning update: {summary}"])
    except DashboardSyncError as exc:
        raise DashboardSyncError(
            f"Could not create a versioned save for publish: {exc}",
            status_code=400,
        ) from exc

    commit_id = _run_git(repo_root, ["rev-parse", "--short", "HEAD"])

    try:
        push_args = ["push"] if uses_upstream else ["push", "-u", remote, branch]
        _run_git(repo_root, push_args)
    except DashboardSyncError as exc:
        raise DashboardSyncError(
            "Created a local versioned save, but could not send it to the shared repository. "
            f"Local commit {commit_id}: {exc}",
            status_code=409,
            details={"commit_id": commit_id, "publish_target": target_label},
        ) from exc

    next_status = get_sync_status(data_dir)
    return {
        "commit_id": commit_id,
        "summary": summary,
        "publish_target": target_label,
        "status": next_status,
    }


def refresh_changes(data_dir: str | Path) -> dict:
    status = get_sync_status(data_dir)
    if not status["has_publish_target"]:
        raise DashboardSyncError(
            "No shared Git remote is configured for this repository. "
            "Configure a remote before using Refresh.",
            status_code=400,
        )
    if status["repo_dirty"]:
        raise DashboardSyncError(
            "Local unpublished changes would be affected by refresh. "
            "Publish or discard local changes before refreshing.",
            status_code=409,
            details={
                "repo_change_count": status["repo_change_count"],
                "scoped_changes": status["scoped_changes"],
            },
        )

    repo_root = Path(status["repo_root"])
    branch = str(status["branch"])
    target_label = str(status["publish_target"])
    remote = target_label.split("/", 1)[0]

    _run_git(repo_root, ["fetch", remote, branch])

    refreshed_status = get_sync_status(data_dir)
    ahead = refreshed_status["ahead"]
    behind = refreshed_status["behind"]

    if ahead and behind:
        raise DashboardSyncError(
            "Local and remote history have diverged. Refresh cannot continue safely. "
            "Use manual Git support to resolve the branch divergence.",
            status_code=409,
            details={"ahead": ahead, "behind": behind},
        )
    if ahead:
        raise DashboardSyncError(
            "Local unpublished commits exist. Publish your local planning changes before refreshing.",
            status_code=409,
            details={"ahead": ahead, "behind": behind},
        )
    if not behind:
        return {
            "updated": False,
            "message": "Local planning data is already up to date.",
            "status": refreshed_status,
        }

    previous_head = str(status["head"])
    _run_git(repo_root, ["merge", "--ff-only", target_label])
    next_status = get_sync_status(data_dir)

    return {
        "updated": True,
        "previous_head": previous_head,
        "head": next_status["head"],
        "message": "Planning data refreshed from the shared repository.",
        "status": next_status,
    }
