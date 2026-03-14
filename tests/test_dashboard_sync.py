import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.append(os.path.join(os.path.dirname(__file__), "..", "src", "dashboard"))

from dashboard_sync import DashboardSyncError, get_sync_status, publish_changes, refresh_changes


def _git(cwd: Path, *args: str) -> str:
    completed = subprocess.run(
        ["git", *args],
        cwd=str(cwd),
        check=True,
        capture_output=True,
        text=True,
    )
    return completed.stdout.strip()


def _write_dataset(repo_root: Path, *, body_suffix: str = "") -> Path:
    data_dir = repo_root / "tst-data"
    planning_dir = data_dir / "planning"
    identity_dir = data_dir / "identity"
    (planning_dir / "people").mkdir(parents=True, exist_ok=True)
    identity_dir.mkdir(parents=True, exist_ok=True)

    (planning_dir / "people" / "alice.md").write_text(
        (
            "---\n"
            "alias: alice\n"
            "role_id: Dev-Role\n"
            "skills: [python]\n"
            "allocations: []\n"
            "---\n"
            f"Notes{body_suffix}\n"
        ),
        encoding="utf-8",
    )
    (identity_dir / "alice.md").write_text(
        "---\nalias: alice\nreal_name: Alice Andersson\n---\n",
        encoding="utf-8",
    )
    return data_dir


def _configure_repo(repo_root: Path) -> None:
    _git(repo_root, "config", "user.name", "Test User")
    _git(repo_root, "config", "user.email", "test@example.com")


class TestDashboardSync(unittest.TestCase):
    def test_publish_changes_commits_and_pushes_data_scope(self):
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            remote = tmp_path / "remote.git"
            subprocess.run(["git", "init", "--bare", str(remote)], check=True)

            repo_root = tmp_path / "work"
            subprocess.run(["git", "clone", str(remote), str(repo_root)], check=True)
            _configure_repo(repo_root)
            data_dir = _write_dataset(repo_root)
            _git(repo_root, "add", ".")
            _git(repo_root, "commit", "-m", "Initial planning data")
            _git(repo_root, "push", "-u", "origin", "master")

            people_file = data_dir / "planning" / "people" / "alice.md"
            people_file.write_text(people_file.read_text(encoding="utf-8") + "Updated\n", encoding="utf-8")

            result = publish_changes(data_dir, "adjust staffing plan")

            self.assertEqual(result["summary"], "adjust staffing plan")
            self.assertIsNotNone(result["commit_id"])
            self.assertFalse(result["status"]["scoped_dirty"])

            clone_check = tmp_path / "check"
            subprocess.run(["git", "clone", str(remote), str(clone_check)], check=True)
            committed_text = (clone_check / "tst-data" / "planning" / "people" / "alice.md").read_text(
                encoding="utf-8"
            )
            self.assertIn("Updated", committed_text)

    def test_refresh_changes_fast_forwards_from_remote(self):
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            remote = tmp_path / "remote.git"
            subprocess.run(["git", "init", "--bare", str(remote)], check=True)

            local_repo = tmp_path / "local"
            subprocess.run(["git", "clone", str(remote), str(local_repo)], check=True)
            _configure_repo(local_repo)

            local_data_dir = _write_dataset(local_repo)
            _git(local_repo, "add", ".")
            _git(local_repo, "commit", "-m", "Initial planning data")
            _git(local_repo, "push", "-u", "origin", "master")

            upstream_repo = tmp_path / "upstream"
            subprocess.run(["git", "clone", str(remote), str(upstream_repo)], check=True)
            _configure_repo(upstream_repo)
            upstream_data_dir = _write_dataset(upstream_repo, body_suffix=" remote")
            people_file = upstream_data_dir / "planning" / "people" / "alice.md"
            people_file.write_text(people_file.read_text(encoding="utf-8") + "Remote update\n", encoding="utf-8")
            _git(upstream_repo, "add", ".")
            _git(upstream_repo, "commit", "-m", "Remote planning update")
            _git(upstream_repo, "push")

            result = refresh_changes(local_data_dir)

            self.assertTrue(result["updated"])
            refreshed_text = (local_data_dir / "planning" / "people" / "alice.md").read_text(encoding="utf-8")
            self.assertIn("Remote update", refreshed_text)

    def test_refresh_changes_blocks_when_local_changes_exist(self):
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            remote = tmp_path / "remote.git"
            subprocess.run(["git", "init", "--bare", str(remote)], check=True)

            repo_root = tmp_path / "work"
            subprocess.run(["git", "clone", str(remote), str(repo_root)], check=True)
            _configure_repo(repo_root)
            data_dir = _write_dataset(repo_root)
            _git(repo_root, "add", ".")
            _git(repo_root, "commit", "-m", "Initial planning data")
            _git(repo_root, "push", "-u", "origin", "master")

            people_file = data_dir / "planning" / "people" / "alice.md"
            people_file.write_text(people_file.read_text(encoding="utf-8") + "Local only\n", encoding="utf-8")

            with self.assertRaises(DashboardSyncError) as ctx:
                refresh_changes(data_dir)

            self.assertIn("Local unpublished changes would be affected", str(ctx.exception))

    def test_get_sync_status_reports_scoped_changes(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            subprocess.run(["git", "init", str(repo_root)], check=True)
            _configure_repo(repo_root)
            data_dir = _write_dataset(repo_root)
            _git(repo_root, "add", ".")
            _git(repo_root, "commit", "-m", "Initial planning data")

            (repo_root / "README.tmp").write_text("repo change\n", encoding="utf-8")
            people_file = data_dir / "planning" / "people" / "alice.md"
            people_file.write_text(people_file.read_text(encoding="utf-8") + "Scoped\n", encoding="utf-8")

            status = get_sync_status(data_dir)

            self.assertTrue(status["scoped_dirty"])
            self.assertTrue(status["repo_dirty"])
            self.assertEqual(status["data_scope"], "tst-data")
            self.assertGreaterEqual(status["repo_change_count"], 2)


if __name__ == "__main__":
    unittest.main()
